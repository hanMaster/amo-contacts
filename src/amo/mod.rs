use crate::amo::data_types::leads::{Contact, ContactInfo, DealWithContacts, Leads, RawContact};
use crate::amo::data_types::pipeline::{Funnel, Pipeline};
pub(crate) use crate::amo::error::{Error, Result};
use reqwest::{Client, StatusCode};

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
    async fn get_funnel_leads(&self, funnel_id: i64) -> Result<Vec<DealWithContacts>> {
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

    async fn extract_deals(&self, leads: Leads) -> Result<Vec<DealWithContacts>> {
        let deals = leads._embedded.leads;

        let mut res: Vec<DealWithContacts> = vec![];

        for d in deals {
            let mut contacts: Vec<ContactInfo> = vec![];
            for cs in d._embedded.contacts.iter() {
                println!("{:#?}", cs);
                let raw = self.get_contact_by_id(cs.id).await?;
                if let Some(c) = raw {
                    let ci = ContactInfo {
                        is_main: cs.is_main,
                        info: c,
                    };
                    contacts.push(ci);
                }
            }

            let dwc = DealWithContacts {
                deal_id: d.id,
                contacts,
            };

            res.push(dwc);
        }

        Ok(res)
    }

    fn pipeline_id(&self) -> i64;

    fn token(&self) -> &str;

    async fn get_contact_by_id(&self, contact_id: i64) -> Result<Option<Contact>> {
        let url = format!("{}contacts/{}", self.base_url(), contact_id);
        let client = Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token()));
        let response = client.send().await?;

        let data = response.json::<RawContact>().await?;
        let res: Contact = data.into();
        match res.owner {
            true => Ok(Some(res)),
            false => Ok(None),
        }
    }
}
