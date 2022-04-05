use std::str::from_utf8;
use futures::TryFutureExt;
use crate::rocket;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use serde::Deserialize;
use crate::database::{get_db_conn, init_db};
use crate::{Order};
use serde_json::{from_str, from_value, json, Value};
use crate::order::{OrderAccessController, TableItems};

#[tokio::test]
async fn get_all_orders_by_table_test() -> Result<(),Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await;
    let query = sqlx::query("INSERT INTO Orders values(nextval('order_id_seq'),100,5,2)").execute(&db.unwrap()).await?;
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let mut response = client.get("/api/current_orders/5").dispatch().await;

    // -- CHECK
    assert_eq!(Status::Ok, response.status());

    let body = response.into_string().await;
    let mut body: Value = from_str(body.as_deref().unwrap()).unwrap();

    let data = body.take();

    let data= from_value(data);

    let orders: Vec<TableItems> = data.unwrap();

    assert_eq!(1, orders.len(), "number of items");
    assert_eq!("pizza", orders[0].item_name);
    assert_eq!(2, orders[0].item_qty);
    assert_eq!(20, orders[0].total_time_to_cook);
    Ok(())
}

#[tokio::test]
async fn create_order_test() -> Result<(),Box<dyn std::error::Error>>{
    // -- FIXTURE
    let db = init_db().await;
    OrderAccessController::create_order(db.unwrap(),Order{
        id: 0,
        item_id: 100,
        table_id: 7,
        quantity: 12
    }).await;
    let new_db = get_db_conn().await;
    OrderAccessController::create_order(new_db.unwrap(),Order{
        id: 0,
        item_id: 101,
        table_id: 8,
        quantity: 3
    }).await;
    let new_db2 = get_db_conn().await;
    let res : Vec<Order> = sqlx::query_as("select * from orders").fetch_all(&new_db2.unwrap()).await?;

    assert_eq!(2, res.len(), "number of items");
    assert_eq!(101, res[1].item_id);
    assert_eq!(8, res[1].table_id);
    assert_eq!(12, res[0].quantity);

    Ok(())
}
