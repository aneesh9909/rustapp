use std::str::from_utf8;
use futures::TryFutureExt;
use super::rocket;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use serde::Deserialize;
use crate::database::init_db;
use crate::Item;
use serde_json::{from_str, from_value, json, Value};

#[tokio::test]
async fn rest_get_items_list_status_test() {
    // -- FIXTURE
    let db = init_db().await;
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let mut response = client.get("/api/items").dispatch().await;

    // -- CHECK
    assert_eq!(Status::Ok, response.status());
}

#[tokio::test]
async fn rest_get_items_list_count_test() {
    // -- FIXTURE
    let db = init_db().await;
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let mut response = client.get("/api/items").dispatch().await;

    // -- CHECK
    assert_eq!(Status::Ok, response.status());

    // extract response data
    let body = response.into_string().await;
    let mut body: Value = from_str(body.as_deref().unwrap()).unwrap();

    let data = body.take();

    let data= from_value(data);

    let items: Vec<Item> = data.unwrap();

    // -- CHECK - todos
    assert_eq!(3, items.len(), "number of items");
    assert_eq!(100, items[2].id);
    assert_eq!("pizza", items[2].name);
    assert_eq!(10, items[2].time_to_cook);

}
