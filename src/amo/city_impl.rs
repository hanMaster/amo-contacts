use crate::amo::AmoClient;
use crate::config::config;
use crate::profit::ProfitbaseClient;
use crate::PROJECTS;

pub struct AmoCityClient {
    account_id: &'static str,
    token: &'static str,
    pipeline_id: i64,
    profitbase_client: ProfitbaseClient,
}

impl AmoClient for AmoCityClient {
    fn new() -> Self {
        Self {
            account_id: &config().AMO_CITY_ACCOUNT,
            token: &config().AMO_CITY_TOKEN,
            pipeline_id: 7486918,
            profitbase_client: ProfitbaseClient::new(
                &config().PROF_CITY_ACCOUNT,
                &config().PROF_CITY_API_KEY,
                PROJECTS[0],
            ),
        }
    }

    fn base_url(&self) -> String {
        format!("https://{}.amocrm.ru/api/v4/", self.account_id)
    }

    fn pipeline_id(&self) -> i64 {
        self.pipeline_id
    }

    fn profitbase_client(&self) -> &ProfitbaseClient {
        &self.profitbase_client
    }

    fn token(&self) -> &str {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> AmoCityClient {
        AmoCityClient::new()
    }
    #[test]
    fn gen_correct_base_url() {
        let client = setup();
        let url = client.base_url();
        assert_eq!("https://dnscity.amocrm.ru/api/v4/", url);
    }

    #[tokio::test]
    async fn test_get_city_funnels() {
        let client = setup();
        let funnels = client.get_funnels().await.unwrap();
        assert_ne!(0, funnels.len());
        println!("{:#?}", funnels);
    }

    #[tokio::test]
    async fn test_get_funnel_leads() {
        let client = setup();
        let leads = client.get_funnel_leads(65830426).await.unwrap();
        println!("{:?}", leads);
        assert_ne!(0, leads.len());
    }
}
