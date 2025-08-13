use crate::amo::AmoClient;
use crate::config::config;

pub struct AmoFormatClient {
    account_id: &'static str,
    token: &'static str,
    pipeline_id: i64,
}

impl AmoClient for AmoFormatClient {
    fn new() -> Self {
        Self {
            account_id: &config().AMO_FORMAT_ACCOUNT,
            token: &config().AMO_FORMAT_TOKEN,
            pipeline_id: 1983685,
        }
    }

    fn base_url(&self) -> String {
        format!("https://{}.amocrm.ru/api/v4/", self.account_id)
    }

    fn pipeline_id(&self) -> i64 {
        self.pipeline_id
    }

    fn token(&self) -> &str {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> AmoFormatClient {
        AmoFormatClient::new()
    }
    #[test]
    fn gen_correct_base_url() {
        let client = setup();
        let url = client.base_url();
        assert_eq!("https://dnsdom.amocrm.ru/api/v4/", url);
    }

    #[tokio::test]
    async fn test_get_funnels() {
        let client = setup();
        let funnels = client.get_funnels().await.unwrap();
        assert_ne!(0, funnels.len());
        println!("{:#?}", funnels);
    }

    #[tokio::test]
    async fn test_funnel_leads() {
        let client = setup();
        let leads = client.get_funnel_leads(42397663).await.unwrap();
        assert_ne!(0, leads.len());
        println!("{:#?}", leads);
    }

    #[tokio::test]
    async fn test_get_contact() {
        let client = setup();
        let contact = client.get_contact_by_id(43136297).await.unwrap();
        println!("{:#?}", contact);
    }
}