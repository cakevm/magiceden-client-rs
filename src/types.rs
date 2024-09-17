pub mod api;

use crate::types::api::{
    MagicedenBuyTokensErrorResponse, MagicedenErrorParseResponse, MagicedenErrorResponse, MagicedenOrderAlreadyFilledError, ServerError,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumString};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MagicedenApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    ResponseParseError(#[from] MagicedenErrorParseResponse),
    #[error(transparent)]
    ServerError(#[from] ServerError),
    #[error(transparent)]
    MagicedenError(#[from] MagicedenErrorResponse),
    #[error(transparent)]
    MagicedenBuyTokensError(#[from] MagicedenBuyTokensErrorResponse),
    #[error(transparent)]
    MagicedenOrderAlreadyFilledError(#[from] MagicedenOrderAlreadyFilledError),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, AsRefStr, Default)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    #[default]
    Ethereum,
    Goerli,
}
impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_ref())
    }
}

impl Chain {
    pub fn is_test_chain(&self) -> bool {
        use Chain::*;
        matches!(self, Goerli)
    }

    #[inline]
    pub fn is_live_chain(&self) -> bool {
        !self.is_test_chain()
    }
}

/// API endpoints
#[derive(Debug, Clone)]
pub struct ApiUrl {
    pub base: String,
}

impl ApiUrl {
    pub fn retrieve_asks(&self, chain: &Chain, query_parameters: String) -> String {
        format!("{}/rtp/{}/orders/asks/v5?{}", self.base, chain, query_parameters)
    }
    pub fn buy_tokens(&self, chain: &Chain) -> String {
        format!("{}/rtp/{}/execute/buy/v7", self.base, chain)
    }
}
