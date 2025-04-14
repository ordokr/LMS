use crate::api_clients::ApiClient;

pub struct DiscourseApiClient {
    base_url: String,
}

impl ApiClient for DiscourseApiClient {
    fn get_base_url(&self) -> &String {
        &self.base_url
    }
}