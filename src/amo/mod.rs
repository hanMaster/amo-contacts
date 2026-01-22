use crate::amo::data_types::leads::{
    Contact, ContactForExport, FinalData, Lead, LeadInfo, Leads, RawContact, RawContacts,
};
pub(crate) use crate::amo::error::{Error, Result};
use crate::config::config;
use crate::profit::ProfitbaseClient;
use log::{error, info};
use regex::Regex;
use reqwest::{Client, StatusCode};
use std::collections::HashSet;
use std::time::Duration;
use tokio::fs;
use tokio::time::sleep;

pub(crate) mod data_types;
mod error;

pub mod city_impl;

pub trait AmoClient {
    fn new() -> Self;
    fn base_url(&self) -> String;

    // async fn check_contacts(&self) -> Result<()> {
    //     let leads = self.get_funnel_leads(config().FUNNEL).await?;
    //     let filtered = self.filter_deals(&leads).await?;
    //     for lead in filtered {
    //         info!("{}", lead);
    //     }
    //     Ok(())
    // }

    async fn collect_contacts(&self) -> Result<()> {
        let leads = self.get_funnel_leads(config().FUNNEL).await?;
        // let filtered = leads
        //     .into_iter()
        //     .filter(|l| {
        //         let house = l.val_to_str("Дом");
        //         let project = l.val_to_str("ЖК");
        //         house.contains("Дом №2") && project == "DNS Сити"
        //     })
        //     .collect::<Vec<_>>();

        // let _ = self.process_deals(&filtered).await?;
        let _ = self.process_deals(&leads).await?;
        Ok(())
    }

    async fn collect_leads(&self) -> Result<()> {
        let leads = self.get_funnel_leads(config().FUNNEL).await?;
        let parsed = leads
            .into_iter()
            .map(|l| {
                let house = l.val_to_str("Дом");
                let project = l.val_to_str("ЖК");
                let property_type = l.val_to_str("Тип помещения");
                let property_num = l.val_to_str("Номер помещения");
                let profit_id = l.val_to_str("ID Помещения");
                LeadInfo {
                    profit_id,
                    lead_id: l.id,
                    project,
                    house,
                    property_type,
                    property_num,
                }
            })
            .filter(|l| {
                // l.house.contains("Дом №5") &&
                //     l.project.contains("DNS")
                l.project.contains("Формат")
            })
            .collect::<Vec<_>>();

        let res = serde_json::to_value(&parsed)?;
        println!("Collected leads: {}", parsed.len());
        let data = serde_json::to_string_pretty(&res)?;

        fs::write("leads.json", data.as_bytes()).await?;

        Ok(())
    }

    async fn get_funnel_leads(&self, funnel_id: i64) -> Result<Vec<Lead>> {
        let url = format!(
            "{}leads?filter[statuses][0][pipeline_id]={}&filter[statuses][0][status_id]={}&with=contacts",
            self.base_url(),
            self.pipeline_id(),
            funnel_id
        );
        info!("first call url: {}", url);
        let client = Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token()));
        let response = client.send().await?;
        let mut data = response.json::<Leads>().await?;
        let mut leads = data._embedded.leads;

        let mut next = data._links.next.take();
        while next.is_some() {
            let url = next.as_ref().unwrap().href.to_string();
            info!("next call url: {}", url);
            let client = Client::new()
                .get(url)
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

    async fn process_deals(&self, leads: &[Lead]) -> Result<()> {
        let base_url = self.base_url();
        let token = self.token().to_string();

        let start = tokio::time::Instant::now();

        let mut all_contact_ids = HashSet::new();
        for lead in leads {
            for i in lead._embedded.contacts.iter() {
                all_contact_ids.insert(i.id);
            }
        }

        let ids: Vec<i64> = all_contact_ids.into_iter().collect();

        let raw_contacts = get_all_contacts(&base_url, &token, &ids).await?;

        let mut final_contacts = vec![];

        for lead in leads {
            for i in lead._embedded.contacts.iter() {
                let contact_option = raw_contacts.iter().find(|c| c.id == i.id);
                if let Some(raw) = contact_option {
                    let owner = raw.val_to_owner();
                    if !owner {
                        continue;
                    }
                    let first_name = raw.val_to_str("Имя");
                    let middle_name = raw.val_to_str("Отчество");
                    let last_name = raw.val_to_str("Фамилия");
                    let phone = raw.val_to_str("Телефон");
                    let email = raw.val_to_str("Email");
                    let c = ContactForExport {
                        first_name,
                        middle_name,
                        last_name,
                        phone,
                        email,
                        client_id: lead.val_to_str("ID Помещения").parse().unwrap(),
                        owner,
                    };
                    final_contacts.push(c);
                }
            }
        }

        let final_data = FinalData {
            users: final_contacts,
        };

        let res = serde_json::to_value(&final_data)?;
        let data = serde_json::to_string_pretty(&res)?;
        fs::write("contacts.json", data.as_bytes()).await?;

        println!("Finished in {:?}", start.elapsed());

        Ok(())
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
    info!("get contact by id: {}", url);
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

async fn get_all_contacts(
    base_url: &str,
    token: &str,
    contact_ids: &[i64],
) -> Result<Vec<RawContact>> {
    let mut data_res = vec![];

    info!("contact ids count: {}", contact_ids.len());

    for batch in contact_ids.chunks(250) {
        let url = gen_contacts_url(base_url, batch);
        info!(
            "get contact by id: {}",
            url.chars().take(100).collect::<String>()
        );

        let client = Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {token}"));
        let response_res = client.send().await;

        match response_res {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    let data = response.text().await?;
                    let contact_data = serde_json::from_str::<RawContacts>(&data)?;
                    data_res.extend(contact_data._embedded.contacts);
                } else {
                    error!("{:?}", response.text().await?);
                }
            }
            Err(e) => error!("{e:?}"),
        }
    }

    Ok(data_res)
}

fn gen_contacts_url(base_url: &str, contact_ids: &[i64]) -> String {
    let params: Vec<String> = contact_ids
        .iter()
        .enumerate()
        .map(|(idx, val)| format!("filter[id][{idx}]={val}"))
        .collect();

    format!("{base_url}contacts?{}", params.join("&"))
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    email_regex.is_match(email)
}
