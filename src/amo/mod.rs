use crate::amo::data_types::leads::{
    Contact, ContactInfo, DealWithContact, Leads, LeadsPrepared, RawContact,
    RawData,
};
use crate::amo::data_types::pipeline::{Funnel, Pipeline};
pub(crate) use crate::amo::error::{Error, Result};
use crate::profit::ProfitbaseClient;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

pub(crate) mod data_types;
mod error;

pub mod city_impl;
pub mod format_impl;

pub trait AmoClient {
    fn new() -> Self;
    fn base_url(&self) -> String;
    async fn get_funnels(&self) -> Result<Vec<Funnel>> {
        let url = format!("{}leads/pipelines/{}", self.base_url(), self.pipeline_id());
        let client = Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token()));
        let response = client.send().await?;
        match response.status() {
            StatusCode::OK => {
                let data = response.json::<Pipeline>().await?;
                let funnels = data._embedded.statuses;
                Ok(funnels)
            }
            _ => {
                let body = response.text().await?;
                eprintln!("Failed to get funnels: {}", body);
                Err(Error::Funnels(body))
            }
        }
    }
    async fn get_funnel_leads(&self, funnel_id: i64) -> Result<Vec<DealWithContact>> {
        let url = format!(
            "{}leads?filter[statuses][0][pipeline_id]={}&filter[statuses][0][status_id]={}&with=contacts",
            self.base_url(),
            self.pipeline_id(),
            funnel_id
        );
        let client = Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token()));
        let response = client.send().await?;
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(vec![]);
        }
        let mut data = response.json::<Leads>().await?;
        let mut next = data._links.next.take();
        let mut leads = self.extract_deals(data).await?;

        while next.is_some() {
            let client = Client::new()
                .get(next.as_ref().unwrap().href.to_string())
                .header("Authorization", format!("Bearer {}", self.token()));
            let mut data = client.send().await?.json::<Leads>().await?;

            next = data._links.next.take();
            let leads_in_while = self.extract_deals(data).await?;

            leads.extend(leads_in_while);
        }
        Ok(leads)
    }

    async fn extract_deals(&self, leads: Leads) -> Result<Vec<DealWithContact>> {
        let base_url = self.base_url();
        let amo_token = self.token().to_string();
        let profit_token = self.profitbase_client().get_profit_token().await?;

        let deal_ids: Vec<u64> = leads._embedded.leads.iter().map(|d| d.id).collect();

        println!("deal_ids: {:?}", deal_ids);
        let mut profit_with_contact_summary: Vec<RawData> = vec![];

        for lead in leads._embedded.leads {
            println!("call profit for: {}", lead.id);
            let profit_data = self
                .profitbase_client()
                .get_profit_data(lead.id, &profit_token)
                .await?;
            profit_with_contact_summary.push(RawData {
                profit_data,
                contacts: lead._embedded.contacts,
            });
        }

        println!("profit_res: {:#?}", profit_with_contact_summary);


        let mut res: Vec<DealWithContact> = vec![];

        let start = tokio::time::Instant::now();
        for chunk in profit_with_contact_summary.chunks(20) {
            println!(
                "processing from {} to {}",
                chunk.first().unwrap().profit_data.deal_id,
                chunk.last().unwrap().profit_data.deal_id
            );
            let mut set = JoinSet::new();

            for i in chunk {
                let bu = base_url.clone();
                let t = amo_token.clone();
                let id = i.contacts[0].id;
                let deal_id = i.profit_data.deal_id;
                let is_main = i.contacts[0].is_main;
                set.spawn(async move { get_contact_by_id(bu, t, deal_id, is_main, id).await });
                sleep(Duration::from_millis(300)).await;
            }

            let output = set.join_all().await;

            let mut have_error = false;
            for o in output {
                if o.is_err() {
                    eprintln!("Error: {:?}", o.unwrap_err());
                    have_error = true;
                } else {
                    let (deal_id, is_main, raw) = o?;
                    let contact: Contact = raw.into();

                    if contact.owner {
                        let ci = ContactInfo {
                            is_main,
                            info: contact,
                        };
                        let dwc = DealWithContact {
                            deal_id,
                            contact: ci,
                        };

                        res.push(dwc);
                    }
                }
            }

            if have_error {
                panic!("Processing failed");
            }
        }
        println!("Finished in {:?}", start.elapsed());

        Ok(res)
    }

    fn pipeline_id(&self) -> i64;

    fn profitbase_client(&self) -> &ProfitbaseClient;

    fn token(&self) -> &str;
}
async fn get_contact_by_id(
    base_url: String,
    token: String,
    deal_id: u64,
    is_main: bool,
    contact_id: i64,
) -> Result<(u64, bool, RawContact)> {
    let url = format!("{}contacts/{}", base_url, contact_id);
    let client = Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {token}"));
    let response_res = client.send().await;

    match response_res {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let data = response.json::<RawContact>().await?;
                Ok((deal_id, is_main, data))
            } else {
                Err(Error::GetContactFailed(response.text().await?))
            }
        }
        Err(e) => Err(Error::Request(e)),
    }
}
