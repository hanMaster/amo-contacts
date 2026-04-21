use crate::amo::AmoClient;
use crate::amo::city_impl::AmoCityClient;
use crate::error::Result;
use crate::xlsx::Xlsx;
use dotenvy::dotenv;
use log::info;

mod amo;
mod config;
mod error;
mod interface;
mod profit;
mod xlsx;

pub const PROJECTS: [&str; 2] = ["ЖК Формат", "DNS Сити"];

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("dotenv init failed");
    pretty_env_logger::init();

    let client = AmoCityClient::new();
    // let data = client.get_funnel_leads(config().FUNNEL).await?;
    // client.check_contacts().await?;

    // client.collect_contacts().await?;
    // client.collect_leads().await?;

    // let data = client.collect_lead_ids().await?;
    // for (idx, lead) in data.iter().enumerate() {
    //     info!("step: {:?}", idx + 1);
    //     client.move_to_sold(lead.lead_id).await?;
    // }

    let data = client.collect_leads_xls().await?;
    Xlsx::create(data)?;

    Ok(())
}
