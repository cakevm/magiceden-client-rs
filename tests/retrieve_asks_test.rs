mod common;
use common::test_client;

use magiceden_client_rs::types::api::AsksRequest;

#[tokio::test]
async fn can_retrieve_listing() {
    let client = test_client();

    let req = AsksRequest { limit: Some(1000), ..Default::default() };

    let res = client.retrieve_asks(req).await.unwrap();

    assert_eq!(res.orders.len(), 1000);
    assert!(res.continuation.is_some());
}
