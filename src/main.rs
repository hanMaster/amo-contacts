use crate::amo::city_impl::AmoCityClient;
use crate::amo::format_impl::AmoFormatClient;
use crate::amo::AmoClient;
use crate::error::Result;
use crate::interface::{read_funnel, read_project};
use crate::xlsx::Xlsx;
use dotenvy::dotenv;

mod amo;
mod config;
mod error;
mod interface;
mod xlsx;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("dotenv init failed");

    let projects = ["ЖК Формат", "ДНС Сити"];
    let project = read_project(&projects);
    if project == "ЖК Формат" {
        let client = AmoFormatClient::new();
        let funnels = client.get_funnels().await?;
        let filtered = funnels
            .into_iter()
            .filter(|f| f.name.to_lowercase().contains("передача"))
            .collect::<Vec<_>>();
        let funnel = read_funnel(filtered);
        let data = client.get_funnel_leads(funnel.id).await?;
        Xlsx::create(&project, &funnel.name, data)?;
    }

    if project == "ДНС Сити" {
        let client = AmoCityClient::new();
        let funnels = client.get_funnels().await?;
        let filtered = funnels
            .into_iter()
            .filter(|f| f.name.to_lowercase().contains("передача"))
            .collect::<Vec<_>>();
        let funnel = read_funnel(filtered);
        let data = client.get_funnel_leads(funnel.id).await?;
        Xlsx::create(&project, &funnel.name, data)?;
    }

    Ok(())
}
