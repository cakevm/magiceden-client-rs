use crate::{
    constants::{API_BASE_MAINNET, API_BASE_TESTNET, PROTOCOL_VERSION},
    types::{
        api::{
            AsksRequest, AsksResponse, BuyTokensRequest, BuyTokensResponse, MagicedenBuyTokensErrorResponse, MagicedenErrorParseResponse,
            MagicedenOrderAlreadyFilledError, ServerError,
        },
        ApiUrl, Chain, MagicedenApiError,
    },
};
use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder, StatusCode,
};

#[derive(Debug, Clone)]
pub struct MagicedenClient {
    client: Client,
    chain: Chain,
    url: ApiUrl,
}

#[derive(Debug, Clone, Default)]
pub struct MagicedenApiConfig {
    pub api_key: Option<String>,
    pub chain: Chain,
}

impl MagicedenClient {
    /// Create a new client with the given configuration.
    pub fn new(cfg: MagicedenApiConfig) -> Self {
        let mut builder = ClientBuilder::new();
        let mut headers = HeaderMap::new();

        if let Some(ref api_key) = cfg.api_key {
            headers.insert("Authorization", header::HeaderValue::from_str(format!("Bearer {}", api_key).as_str()).unwrap());
        }

        builder = builder.default_headers(headers);
        let client = builder.build().unwrap();

        let base_url = if cfg.chain.is_test_chain() { API_BASE_TESTNET } else { API_BASE_MAINNET };

        let base_url = format!("{base_url}/{PROTOCOL_VERSION}");

        Self { client, chain: cfg.chain, url: ApiUrl { base: base_url } }
    }

    pub async fn retrieve_asks(&self, params: AsksRequest) -> Result<AsksResponse, MagicedenApiError> {
        let query_parameters = serde_url_params::to_string(&params).unwrap();
        let res = self.client.get(self.url.retrieve_asks(&self.chain, query_parameters)).send().await;
        match res {
            Ok(res) => {
                let status_code = res.status();
                let body = res.text().await?;
                let res = serde_json::from_str::<AsksResponse>(&body);
                match res {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        let e = MagicedenErrorParseResponse { body, status_code: status_code.as_u16(), error: e.to_string() };
                        Err(MagicedenApiError::ResponseParseError(e))
                    }
                }
            }
            Err(e) => Err(MagicedenApiError::Reqwest(e)),
        }
    }

    pub async fn buy_tokens(&self, req: BuyTokensRequest) -> Result<BuyTokensResponse, MagicedenApiError> {
        let res = self.client.post(self.url.buy_tokens(&self.chain)).json(&req).send().await;
        match res {
            Ok(res) => {
                let status_code = res.status();
                let body = res.text().await?;

                if status_code == StatusCode::BAD_REQUEST {
                    let res = serde_json::from_str::<MagicedenBuyTokensErrorResponse>(&body);
                    return match res {
                        Ok(r) => Err(MagicedenApiError::MagicedenBuyTokensError(r)),
                        Err(e) => {
                            let e = MagicedenErrorParseResponse { status_code: status_code.as_u16(), body, error: e.to_string() };
                            Err(MagicedenApiError::ResponseParseError(e))
                        }
                    };
                } else if status_code == StatusCode::GONE {
                    let res = serde_json::from_str::<MagicedenOrderAlreadyFilledError>(&body);
                    return match res {
                        Ok(r) => Err(MagicedenApiError::MagicedenOrderAlreadyFilledError(r)),
                        Err(e) => {
                            let e = MagicedenErrorParseResponse { body, status_code: status_code.as_u16(), error: e.to_string() };
                            Err(MagicedenApiError::ResponseParseError(e))
                        }
                    };
                }
                if status_code != 200 {
                    return Err(MagicedenApiError::ServerError(ServerError { status_code: status_code.as_u16(), body }));
                }

                let res = serde_json::from_str::<BuyTokensResponse>(&body);
                match res {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        let e = MagicedenErrorParseResponse { body, status_code: status_code.as_u16(), error: e.to_string() };
                        Err(MagicedenApiError::ResponseParseError(e))
                    }
                }
            }
            Err(e) => Err(MagicedenApiError::Reqwest(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn can_deserialize_buy_magiceden_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_buy_magiceden.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: BuyTokensResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.path.first().unwrap().token_id, "837");
    }

    #[test]
    fn can_deserialize_asks_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_asks.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: AsksResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.orders.first().unwrap().id, "0x5844792a36ff5966a325d2180ebda80f8f63a7f3d4585e1c88615a111ce42942");
    }
}
