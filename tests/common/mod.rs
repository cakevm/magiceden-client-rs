use magiceden_client_rs::{types::Chain, MagicedenApiConfig, MagicedenClient};

pub fn test_client() -> MagicedenClient {
    let cfg = MagicedenApiConfig { chain: Chain::Ethereum, ..Default::default() };

    MagicedenClient::new(cfg)
}

#[allow(dead_code)]
pub fn live_client() -> MagicedenClient {
    let cfg = MagicedenApiConfig { ..Default::default() };

    MagicedenClient::new(cfg)
}
