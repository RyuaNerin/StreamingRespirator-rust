use std::sync::Mutex;

use lazy_static::lazy_static;
use reqwest::{Client, Method, Request, Url, header};
use serde_derive::*;
use regex::Regex;

use super::twitter::TwitterUser;


#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterCredential {
    pub id: u64,
    pub screen_name: String,
    pub cookie: String,

    #[serde(skip)]
    x_csrf_token: Mutex<String>,

    #[serde(skip)]
    pub(crate) client: Client,
}

impl Default for TwitterCredential {
    fn default() -> Self {
        TwitterCredential {
            id: 0,
            screen_name: String::default(),
            cookie: String::default(),
            x_csrf_token: Mutex::new(String::default()),
            client: Client::default(),
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new("ct0=([^;]+)").unwrap();
}

impl TwitterCredential {
    pub fn id(&self) -> u64 { self.id }
    pub fn screen_name(&self) -> &str { self.screen_name.as_str() }

    pub fn create_reqwest(&self, method: Method, url: Url) -> Request {
        let mut x_csrf_token = self.x_csrf_token.lock().unwrap();

        if *x_csrf_token == String::default() {
            match RE.captures(self.cookie.as_str()) {
                Some(x) => match x.get(1) {
                    Some(x) => *x_csrf_token = x.as_str().to_string(),
                    None => (),
                }
                None => (),
            }
        }
        

        let mut req = Request::new(method, url);
        let header = req.headers_mut();
        header.append(header::AUTHORIZATION, "Bearer AAAAAAAAAAAAAAAAAAAAAF7aAAAAAAAASCiRjWvh7R5wxaKkFp7MM%2BhYBqM%3DbQ0JPmjU9F6ZoMhDfI4uTNAaQuTDm2uO9x3WFVr2xBZ2nhjdP0".parse().unwrap());
        header.append(header::COOKIE, self.cookie.parse().unwrap());
        header.append(header::USER_AGENT, "Streaming-Respirator".parse().unwrap());

        header.append("X-Csrf-Token", x_csrf_token.parse().unwrap());
        header.append("X-Twitter-Auth-Type", "OAuth2Session".parse().unwrap());
        header.append("X-Twitter-Client-Version", "Twitter-TweetDeck-blackbird-chrome/4.0.190115122859 web/".parse().unwrap());

        req
    }

    pub async fn verify_credentials(&mut self) -> Result<TwitterUser, reqwest::Error> {
        let req = self.create_reqwest(Method::GET, "https://api.twitter.com/1.1/account/verify_credentials.json".parse().unwrap());

        let resp = self.client.execute(req).await?;

        let r: TwitterUser = resp.json().await?;

        self.id = r.id;
        self.screen_name = r.screen_name.clone();

        Ok(r)
    }
}
