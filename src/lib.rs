pub mod schema;

use reqwest::{Client, header};

const BASE_URL: &str = "https://api.artifactsmmo.com/";

#[derive(Debug, Clone)]
pub struct ArtifactClient {
    key: String,
}

impl ArtifactClient {
    fn client(&self) -> Result<Client, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(format!("Bearer {}", &self.key).as_str()).unwrap(),
        );
        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
    }

    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }

    pub async fn get_character(
        &self,
        name: impl Into<String>,
    ) -> Result<schema::CharacterSchema, reqwest::Error> {
        let name: String = name.into();
        println!("Getting character: {}", name);
        let client = self.client()?;
        let url = reqwest::Url::parse(BASE_URL)
            .unwrap()
            .join(format!("characters/{}", name).as_str())
            .unwrap();
        println!("Using URL: {}", url.clone());

        let resp = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<schema::ResponseWrapper<schema::CharacterSchema>>()
            .await?;
        Ok(resp.data)
    }

    pub fn character(&self, character: impl Into<String>) -> ArtifactClientCharacter {
        let client = self.client().unwrap();
        ArtifactClientCharacter {
            client,
            character: character.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactClientCharacter {
    client: Client,
    character: String,
}

impl ArtifactClientCharacter {
    fn base_url(&self) -> reqwest::Url {
        let base = reqwest::Url::parse(BASE_URL).unwrap();
        base.join(format!("my/{}/", self.character).as_str())
            .unwrap()
    }

    pub async fn mov(
        &self,
        x: i32,
        y: i32,
    ) -> Result<schema::CharacterMovementDataSchema, reqwest::Error> {
        let url = self.base_url().join("action/move").unwrap();
        let body = serde_json::json!({ "x": x, "y": y });
        let resp = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<schema::ResponseWrapper<schema::CharacterMovementDataSchema>>()
            .await?;

        Ok(resp.data)
    }
}
