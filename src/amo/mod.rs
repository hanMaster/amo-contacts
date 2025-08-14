use crate::amo::data_types::leads::{
    Contact, ContactInfo, DealWithContact, Leads, LeadsPrepared, RawContact,
};
use crate::amo::data_types::pipeline::{Funnel, Pipeline};
pub(crate) use crate::amo::error::{Error, Result};
use reqwest::{Client, StatusCode};
use tokio::task::JoinSet;

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
        let mut leads = self.extract_deals(data.into()).await?;

        while next.is_some() {
            let client = Client::new()
                .get(next.as_ref().unwrap().href.to_string())
                .header("Authorization", format!("Bearer {}", self.token()));
            let mut data = client.send().await?.json::<Leads>().await?;

            next = data._links.next.take();
            let leads_in_while = self.extract_deals(data.into()).await?;

            leads.extend(leads_in_while);
        }
        Ok(leads)
    }

    async fn extract_deals(&self, leads: LeadsPrepared) -> Result<Vec<DealWithContact>> {
        let base_url = self.base_url();
        let token = self.token().to_string();

        let mut res: Vec<DealWithContact> = vec![];

        let start = tokio::time::Instant::now();
        for chunk in leads.deals.chunks(7) {
            println!(
                "processing from {} to {}",
                chunk.first().unwrap().deal_id,
                chunk.last().unwrap().deal_id
            );
            let mut set = JoinSet::new();

            for i in chunk {
                println!("processing {:?}", i);
                let bu = base_url.clone();
                let t = token.clone();
                let id = i.contact_id;
                let deal_id = i.deal_id;
                set.spawn(async move { get_contact_by_id(bu, t, deal_id, id).await });
            }

            let output = set.join_all().await;

            let mut have_error = false;
            for o in output {
                if o.is_err() {
                    eprintln!("Error: {:?}", o.unwrap_err());
                    have_error = true;
                } else {
                    let (deal_id, raw) = o?;
                    let contact: Contact = raw.into();

                    if contact.owner {
                        let ci = ContactInfo {
                            is_main: true,
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

    fn token(&self) -> &str;
}
async fn get_contact_by_id(
    base_url: String,
    token: String,
    deal_id: u64,
    contact_id: i64,
) -> Result<(u64, RawContact)> {
    let url = format!("{}contacts/{}", base_url, contact_id);
    println!("url: {}", url);
    let client = Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {token}"));
    let response = client.send().await?;

    let data = response.json::<RawContact>().await?;
    Ok((deal_id, data))
}
