mod common;

use common::test_client;

use magiceden_client_rs::types::api::{BuyTokensRequest, Listing};

// Test is optional because it requires an active order
#[ignore]
#[tokio::test]
async fn can_buy_tokens() {
    let client = test_client();

    let req = BuyTokensRequest {
        items: vec![Listing {
            order_id: Some("0x260a17195de36319209a099f2f90527b7e40e99724e7f8426e947c8f7b325e8d".to_string()),
            ..Default::default()
        }],
        taker: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
        ..Default::default()
    };

    let res = client.buy_tokens(req).await.unwrap();

    assert_eq!(res.path.len(), 1);
    assert_eq!(res.steps.len(), 2);
}
