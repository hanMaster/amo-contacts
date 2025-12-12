use crate::amo::data_types::leads::{Contact, Lead, Leads, RawContact};
pub(crate) use crate::amo::error::{Error, Result};
use crate::config::config;
use crate::profit::ProfitbaseClient;
use log::{debug, info};
use regex::Regex;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::sleep;

pub(crate) mod data_types;
mod error;

pub mod city_impl;

pub trait AmoClient {
    fn new() -> Self;
    fn base_url(&self) -> String;

    async fn check_contacts(&self) -> Result<()> {
        let leads = self.get_funnel_leads(config().FUNNEL).await?;
        let filtered = self.filter_deals(&leads).await?;
        for lead in filtered {
            info!("{}", lead);
        }
        Ok(())
    }
    async fn get_funnel_leads(&self, funnel_id: i64) -> Result<Vec<Lead>> {
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
        let mut data = response.json::<Leads>().await?;
        let mut leads = data._embedded.leads;

        let mut next = data._links.next.take();
        while next.is_some() {
            let client = Client::new()
                .get(next.as_ref().unwrap().href.to_string())
                .header("Authorization", format!("Bearer {}", self.token()));
            let mut data = client.send().await?.json::<Leads>().await?;

            next = data._links.next.take();
            leads.extend(data._embedded.leads);
        }

        Ok(leads)
    }

    async fn filter_deals(&self, leads: &[Lead]) -> Result<Vec<Lead>> {
        let base_url = self.base_url();
        let token = self.token().to_string();

        let start = tokio::time::Instant::now();

        let mut bad_leads: Vec<Lead> = vec![];

        for lead in leads {
            println!("processing contacts for {}", lead.id,);
            let mut is_contact_ok = false;

            for i in lead._embedded.contacts.iter() {
                let contact_id = i.id;
                let contact_option = get_contact_by_id(&base_url, &token, contact_id).await?;
                match contact_option {
                    Some(raw_contact) => {
                        let c: Contact = raw_contact.into();
                        if c.owner
                            && is_valid_email(&c.email)
                            && c.doc_type.len() > 0
                            && c.doc_serial.len() > 3
                            && c.doc_number.len() == 6
                        {
                            is_contact_ok = true;
                        } else {
                            println!("{:#?}", c);
                        }
                    }
                    None => {}
                }
                sleep(Duration::from_millis(100)).await;
            }
            if !is_contact_ok {
                bad_leads.push(lead.clone());
            }
        }
        println!("Finished in {:?}", start.elapsed());

        Ok(bad_leads)
    }

    fn pipeline_id(&self) -> i64;

    fn profitbase_client(&self) -> &ProfitbaseClient;

    fn token(&self) -> &str;
}
async fn get_contact_by_id(
    base_url: &str,
    token: &str,
    contact_id: i64,
) -> Result<Option<RawContact>> {
    let url = format!("{}contacts/{}", base_url, contact_id);
    let client = Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {token}"));
    let response_res = client.send().await;

    match response_res {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let data = response.text().await?;
                // debug!("Contact ID: {}", data);
                let contact_data = serde_json::from_str::<RawContact>(&data).ok();
                Ok(contact_data)
            } else {
                Err(Error::GetContactFailed(response.text().await?))
            }
        }
        Err(e) => Err(Error::Request(e)),
    }
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    email_regex.is_match(email)
}
