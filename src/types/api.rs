use crate::types::MagicedenApiError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::HashMap, fmt};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExecutionProvider {
    #[serde(rename = "seaport-v1.5-intent")]
    SeaportV15,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcludeItem {
    pub order_id: String,
    pub price_: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SwapProvider {
    #[serde(rename = "uniswap")]
    Uniswap,
    #[serde(rename = "1inch")]
    OneInch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FillMethod {
    Trade,
    Mint,
    PreferMint,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RawOrderKind {
    #[default]
    #[serde(rename = "opensea")]
    OpenSea,
    #[serde(rename = "blur-partial")]
    BlurPartial,
    #[serde(rename = "looks-rare")]
    LooksRare,
    #[serde(rename = "zeroex-v4")]
    ZeroExV4,
    #[serde(rename = "seaport")]
    Seaport,
    #[serde(rename = "seaport-v1.4")]
    SeaportV14,
    #[serde(rename = "seaport-v1.5")]
    SeaportV15,
    #[serde(rename = "seaport-v1.6")]
    SeaportV16,
    #[serde(rename = "x2y2")]
    X2Y2,
    #[serde(rename = "rarible")]
    Rarible,
    #[serde(rename = "sudoswap")]
    SudoSwap,
    #[serde(rename = "nftx")]
    NFTX,
    #[serde(rename = "alienswap")]
    AlienSwap,
    #[serde(rename = "mint")]
    Mint,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RawOrder {
    pub kind: RawOrderKind,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    // Collection to buy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    // Token to buy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    // Quantity of tokens to buy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u16>,
    // Optional order id to fill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    // Optional raw order to fill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_order: Option<RawOrder>,
    // Optionally specify a particular fill method. Only relevant when filling via collection.
    // Default: preferMint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_method: Option<FillMethod>,
    // If there are multiple listings with equal best price, prefer this source over others.
    // NOTE: if you want to fill a listing that is not the best priced, you need to pass a specific order id or use exactOrderSource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_order_source: Option<String>,
    // Only consider orders from this source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_order_source: Option<String>,
    // Items to exclude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<ExcludeItem>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokensRequest {
    // List of items to buy.
    pub items: Vec<Listing>,
    // Address of wallet filling (receiver of the NFT).
    pub taker: String,
    // Address of wallet relaying the fill transaction (paying for the NFT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relayer: Option<String>,
    // If true, only the path will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_path: Option<bool>,
    // If true, all fills will be executed through the router (where possible)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_router: Option<bool>,
    // Currency to be used for purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    // The chain id of the purchase currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_chain_id: Option<u16>,
    // Charge any missing royalties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalize_royalties: Option<bool>,
    // If true, inactive orders will not be skipped over (only relevant when filling via a specific order id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_inactive_order_ids: Option<bool>,
    // Filling source used for attribution. Example: magiceden.io
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    // List of fees (formatted as feeRecipient:feeAmount) to be taken when filling.
    // Unless overridden via the currency param, the currency used for any fees on top matches the buy-in currency detected by the backend.
    // Example: 0xF296178d553C8Ec21A2fBD2c5dDa8CA9ac905A00:1000000000000000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<String>>,
    // If true, any off-chain or on-chain errors will be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    // If true, balance check will be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_balance_check: Option<bool>,
    // Exclude orders that can only be filled by EOAs, to support filling with smart contracts. If marked true, blur will be excluded.
    #[serde(rename = "excludeEOA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_eoa: Option<bool>,
    // Optional custom gas settings. Includes base fee & priority fee in this limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<String>,
    // Optional custom gas settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<String>,
    // When true, will use permit to avoid approvals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit: Option<bool>,
    // Choose a specific swapping provider when buying in a different currency (defaults to uniswap)
    // Default: uniswap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_provider: Option<SwapProvider>,
    // Optional execution method to use for filling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_method: Option<ExecutionProvider>,
    // Referrer address (where supported)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    // Mint comment (where supported)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    // Optional X2Y2 API key used for filling.
    #[serde(rename = "x2y2ApiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2y2_api_key: Option<String>,
    // Optional OpenSea API key used for filling. You don't need to pass your own key, but if you don't, you are more likely to be rate-limited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensea_api_key: Option<String>,
    // Advanced use case to pass personal blurAuthToken; the API will generate one if left empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_auth_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuyTokensKind {
    Signature,
    Transaction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Complete,
    Incomplete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokenItemData {
    pub from: String,
    pub to: String,
    pub data: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Post,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokenCheckBody {
    pub kind: BuyTokensKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuyTokenCheck {
    pub endpoint: String,
    pub method: Method,
    pub body: BuyTokenCheckBody,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokenItem {
    pub status: Status,
    pub tip: Option<String>,
    pub order_ids: Option<Vec<String>>,
    pub data: BuyTokenItemData,
    // Approximation of gas used (only applies to transaction items)
    pub gas_estimate: u64,
    // The details of the endpoint for checking the status of the step
    pub check: BuyTokenCheck,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokenError {
    pub message: String,
    pub order_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInFees {
    pub kind: String,
    pub recipient: String,
    pub bps: u64,
    pub amount: f64,
    pub raw_amount: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokenPath {
    pub order_id: String,
    pub contract: String,
    pub token_id: String,
    pub quantity: u16,
    pub source: String,
    pub currency: String,
    pub currency_symbol: String,
    pub currency_decimals: u8,
    pub quote: f64,
    pub raw_quote: String,
    pub buy_in_currency: Option<String>,
    pub buy_in_currency_symbol: Option<String>,
    pub buy_in_currency_decimals: Option<u8>,
    pub buy_in_quote: Option<f64>,
    pub buy_in_raw_quote: Option<String>,
    pub total_price: f64,
    pub total_raw_price: String,
    // Can be marketplace fees or royalties
    pub built_in_fees: Vec<BuildInFees>,
    // Can be referral fees.
    pub fees_on_top: Vec<BuildInFees>,
    pub from_chain_id: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxQuantities {
    pub item_index: u16,
    pub max_quantity: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokensStep {
    pub id: String,
    pub action: String,
    pub description: String,
    pub kind: BuyTokensKind,
    pub items: Vec<BuyTokenItem>,

    pub max_quantities: Option<Vec<MaxQuantities>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokensResponse {
    pub request_id: String,
    pub steps: Vec<BuyTokensStep>,
    pub errors: Vec<BuyTokenError>,
    pub path: Vec<BuyTokenPath>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortBy {
    CreatedAt,
    UpdatedAt,
    Price,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AsksRequest {
    pub ids: Option<Vec<String>>,
    // Filter to a particular token. Example: 0x8d04a8c79ceb0889bdd12acdf3fa9d207ed3ff63:123
    pub token: Option<String>,
    // Filter to a particular set, e.g. contract:0x8d04a8c79ceb0889bdd12acdf3fa9d207ed3ff63
    pub token_set_id: Option<String>,
    // Filter to a particular user. Example: 0xF296178d553C8Ec21A2fBD2c5dDa8CA9ac905A00
    pub maker: Option<String>,
    // Filter to a particular community. Example: artblocks
    pub community: Option<String>,
    // Filter to a particular collection set. Example: 8daa732ebe5db23f267e58d52f1c9b1879279bcdf4f78b8fb563390e6946ea65
    pub collection_set_id: Option<String>,
    // Filter to a particular contracts set.
    pub contract_set_id: Option<String>,
    pub contracts: Option<Vec<String>>,
    // activeª^º = currently valid
    // inactiveª^ = temporarily invalid
    // expiredª^, cancelledª^, filledª^ = permanently invalid
    // anyªº = any status
    // ª when an id is passed
    // ^ when a maker is passed
    // º when a contract is passed
    pub status: Option<String>,
    pub sources: Option<Vec<String>>,
    // If true, results will filter only Reservoir orders.
    pub native: Option<bool>,
    // If true, private orders are included in the response.
    pub include_private: Option<bool>,
    // If true, criteria metadata is included in the response.
    pub include_criteria_metadata: Option<bool>,
    // If true, raw data is included in the response.
    pub include_raw_data: Option<bool>,
    // If true, dynamic pricing data will be returned in the response.
    pub include_dynamic_pricing: Option<bool>,
    // Exclude orders that can only be filled by EOAs, to support filling with smart contracts.
    #[serde(rename = "excludeEOA")]
    pub exclude_eoa: Option<bool>,
    pub exclude_sources: Option<Vec<String>>,
    // Get events after a particular unix timestamp (inclusive)
    pub start_timestamp: Option<u64>,
    // Get events before a particular unix timestamp (inclusive)
    pub end_timestamp: Option<u64>,
    // If true, prices will include missing royalties to be added on-top.
    pub normalize_royalties: Option<bool>,
    // Order the items are returned in the response. Sorting by price is ascending order only.
    pub sort_by: Option<SortBy>,
    pub sort_direction: Option<String>,
    // Use continuation token to request next offset of items. Going back in time.
    pub continuation: Option<String>,
    // Amount of items returned in response. Max limit is 1000.
    pub limit: Option<u16>,
    // Return result in given currency
    pub display_currency: Option<String>,
}

pub(crate) fn value_to_string(v: &Value) -> Result<String, MagicedenApiError> {
    match v {
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::String(s) => Ok(s.to_owned()),
        _ => Err(MagicedenApiError::Other(format!("Wrong value type: {v:?}"))),
    }
}

impl AsksRequest {
    /// Converts RetrieveListingsRequest into serde_json::Map<String, serde_json::Value>
    pub fn to_map(&self) -> serde_json::Result<Map<String, Value>> {
        Ok(serde_json::to_value(self)?.as_object().expect("This should never happen").to_owned())
    }

    /// Converts AsksRequest into a vector of key-value pairs
    /// Magiceden API expects arrays to be passed as a sequence of parameters with the same key (e.g. ?token_ids=1&token_ids=209)
    pub fn to_qs_vec(&self) -> Result<Vec<(String, String)>, MagicedenApiError> {
        let map = self.to_map()?;
        let mut vec = Vec::new();
        for (k, v) in map.iter() {
            match v {
                Value::Array(arr) => {
                    for v in arr {
                        vec.push((k.clone(), value_to_string(v)?))
                    }
                }
                _ => vec.push((k.clone(), value_to_string(v)?)),
            }
        }
        Ok(vec)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Active,
    Inactive,
    Expired,
    Canceled,
    Filled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Currency {
    pub contract: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Amount {
    pub raw: String,
    decimal: f64,
    usd: f64,
    native: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub currency: Currency,
    pub amount: Amount,
    pub net_amount: Amount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub token_id: String,
    pub name: String,
    pub image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CriteriaData {
    pub token: Option<Token>,
    pub collection: Option<Collection>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Criteria {
    pub kind: String,
    pub data: Amount,
    pub net_amount: CriteriaData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeBreakdown {
    // Can be marketplace or royalty
    pub kind: String,
    pub recipient: String,
    pub bps: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Depth {
    pub price: u64,
    pub quantity: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "blur")]
    Blur,
    #[serde(rename = "seaport-v1.4")]
    SeaportV14,
    #[serde(rename = "seaport-v1.5")]
    SeaportV15,
    #[serde(rename = "seaport-v1.6")]
    SeaportV16,
    #[serde(rename = "x2y2")]
    X2Y2,
    #[serde(rename = "looks-rare-v2")]
    LooksRareV2,
    #[serde(rename = "superrare")]
    Superrare,
    #[serde(rename = "payment-processor-v2")]
    PaymentProcessorV2, // Magic Eden
    #[serde(rename = "element-erc721")]
    ElementErc721, // element.market
    #[serde(rename = "foundation")]
    Foundation,
    #[serde(rename = "rarible")]
    Rarible,
    #[serde(rename = "caviar-v1")]
    CaviarV1,
    #[serde(rename = "nftx")]
    NFTX,
    #[serde(rename = "sudoswap")]
    Sudoswap,
    #[serde(rename = "payment-processor")]
    PaymentProcessor, // Magic Eden
    #[serde(rename = "alienswap")]
    Alienswap,
    #[serde(rename = "manifold")]
    Manifold,
    #[serde(rename = "cryptopunks")]
    Cryptopunks,
    #[serde(rename = "zeroex-v4-erc721")]
    ZeroExV4Erc721,
    #[serde(rename = "sudoswap-v2")]
    SudoswapV2,
    #[serde(rename = "zeroex-v4-erc1155")]
    ZeroExV4Erc1155,
    #[serde(rename = "mintify")]
    Mintify,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub kind: Kind,
    pub side: Side,
    pub status: OrderStatus,
    pub token_set_id: String,
    pub token_set_schema_hash: String,
    pub contract: Option<String>,
    pub contract_kind: Option<String>,
    pub maker: String,
    pub taker: String,
    pub price: Option<Price>,
    pub valid_from: u64,
    pub valid_until: u64,
    pub quantity_filled: Option<u64>,
    pub quantity_remaining: Option<u64>,
    //pub dynamic_pricing: Option<DynamicPricing>
    pub critera: Option<Criteria>,
    pub source: Option<HashMap<String, String>>,
    pub fee_bps: Option<u64>,
    pub fee_breakdown: Vec<FeeBreakdown>,
    pub expiration: u64,
    pub is_reservoir: Option<bool>,
    pub is_dynamic: Option<bool>,
    // Time when added to indexer
    pub created_at: DateTime<Utc>,
    // Time when updated in indexer
    pub updated_at: DateTime<Utc>,
    // Time when created by maker
    pub originated_at: Option<DateTime<Utc>>,
    pub raw_data: Option<HashMap<String, String>>,
    pub is_native_off_chain_cancellable: Option<bool>,
    pub depth: Option<Vec<Depth>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsksResponse {
    pub orders: Vec<Order>,
    pub continuation: Option<String>,
}

#[derive(Error, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicedenBuyTokensErrorResponse {
    pub status_code: u32,
    pub error: String,
    pub message: String,
}

impl fmt::Display for MagicedenBuyTokensErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status code: {} Error: {} Message: {}", self.status_code, self.error, self.message)
    }
}

#[derive(Error, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerError {
    pub status_code: u16,
    pub body: String,
}
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status code {}, Body: {}", self.status_code, self.body)
    }
}

#[derive(Error, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MagicedenErrorResponse {
    pub msg: String,
    pub errors: Vec<String>,
}

impl fmt::Display for MagicedenErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Msg: {} Error: {:?}", self.msg, self.errors)
    }
}

#[derive(Error, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MagicedenErrorParseResponse {
    pub body: String,
    pub status_code: u16,
    pub error: String,
}

impl fmt::Display for MagicedenErrorParseResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Body: {} Error: {}", self.body, self.error)
    }
}

#[derive(Error, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicedenOrderAlreadyFilledError {
    pub status_code: u32,
    pub error: String,
    pub message: String,
    pub code: u32,
}

impl fmt::Display for MagicedenOrderAlreadyFilledError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Already filled: status_code={}, error={}, message={}, code={}", self.status_code, self.error, self.message, self.code)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_serialize_buy_tokens_request() {}
}
