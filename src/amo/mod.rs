use crate::amo::data_types::leads::{
    Contact, ContactInfo, Leads, ProfitWithContact, RawContact, RawDataFlat,
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
    async fn get_funnel_leads(&self, funnel_id: i64) -> Result<Vec<ProfitWithContact>> {
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

    async fn extract_deals(&self, leads: Leads) -> Result<Vec<ProfitWithContact>> {
        let base_url = self.base_url();
        let amo_token = self.token().to_string();

        let data = self
            .profitbase_client()
            .collect_profit_data(leads)
            .await?;

        let mut res: Vec<ProfitWithContact> = vec![];

        let start = tokio::time::Instant::now();
        for chunk in data.chunks(20) {
            println!(
                "processing contacts from {} to {}",
                chunk.first().unwrap().profit_data.deal_id,
                chunk.last().unwrap().profit_data.deal_id
            );
            let mut set = JoinSet::new();

            for i in chunk {
                let bu = base_url.clone();
                let t = amo_token.clone();
                let clonned_raw_data = i.clone();
                set.spawn(async move { get_contact_by_id(bu, t, clonned_raw_data).await });
                sleep(Duration::from_millis(200)).await;
            }

            let output = set.join_all().await;

            let mut have_error = false;
            for o in output {
                if o.is_err() {
                    eprintln!("Error: {:?}", o.unwrap_err());
                    have_error = true;
                } else {
                    let (raw_data, contact_data) = o?;
                    let contact: Contact = contact_data.into();

                    if contact.owner {
                        let ci = ContactInfo {
                            is_main: raw_data.contact.is_main,
                            info: contact,
                        };
                        let dwc = ProfitWithContact {
                            profit_data: raw_data.profit_data,
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
    raw_data: RawDataFlat,
) -> Result<(RawDataFlat, RawContact)> {
    let url = format!("{}contacts/{}", base_url, raw_data.contact.id);
    let client = Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {token}"));
    let response_res = client.send().await;

    match response_res {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let contact_data = response.json::<RawContact>().await?;
                Ok((raw_data, contact_data))
            } else {
                Err(Error::GetContactFailed(response.text().await?))
            }
        }
        Err(e) => Err(Error::Request(e)),
    }
}
