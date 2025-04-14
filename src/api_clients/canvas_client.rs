// Implement ApiClient for CanvasClient




#[derive(Debug, Clone)]

pub struct CanvasClient {

    client: Client,

}



#[allow(dead_code)]

impl ApiClient for CanvasClient {

    fn get(&self, url: &str) -> Result<Response, reqwest::Error> {

        self.client.get(url).send().map_err(reqwest::Error::from)

    }



    fn post(&self, url: &str, body: serde_json::Value) -> Result<Response, reqwest::Error> {

        self.client.post(url).json(&body).send().map_err(reqwest::Error::from)

    }



    fn put(&self, url: &str, body: serde_json::Value) -> Result<Response, reqwest::Error> {

        self.client.put(url).json(&body).send().map_err(reqwest::Error::from)

    }



    fn delete(&self, url: &str) -> Result<Response, reqwest::Error> {

        self.client.delete(url).send().map_err(reqwest::Error::from)

    }
