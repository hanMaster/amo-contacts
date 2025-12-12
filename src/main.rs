use crate::amo::city_impl::AmoCityClient;
use crate::amo::AmoClient;
use crate::error::Result;
use dotenvy::dotenv;

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
    client.check_contacts().await?;
    // Xlsx::create("Передача ЖК", data)?;

    Ok(())
}
