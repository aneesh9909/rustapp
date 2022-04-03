#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused)]

mod database;
#[path="models/item.rs"] mod item;
#[path="models/order.rs"] mod order;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::fs::File;
use rocket::{Build, Rocket};
use rocket::form::Form;
use sqlx::Error;
use tokio::io::split;
use crate::database::Db;
use crate::item::Item;
use crate::order::Order;
use crate::order::FormOrder;

#[get("/items")]
async fn index() -> String {
    let db  = database::get_db_conn().await;
    match db{
        Ok(_) => {
            let res = item::ItemAccessController::get_all_items(db.unwrap()).await;
            match res{
                Ok(ref item) => serde_json::to_string(&res.unwrap()).unwrap(),
                Err(_) => "Unable to fetch items".to_string()
            }
        }
        Err(_) => "Unable to connect to database".to_string()
    }
}

#[post("/order", data = "<input>")]
async fn create_order(input : Form<FormOrder>) -> String{
    let mut data : Vec<&str> = input.item_qty.split(",").collect();
    let mut rows_affected : i32 = 0;

    for item in data{
        let details_str = item.split("_");
        let vec : Vec<&str> = details_str.collect();
        let item_id : i32 = vec[0].parse::<i32>().unwrap();
        let item_qty : i32 = vec[1].parse::<i32>().unwrap();
        let order = Order {
            id: 0,
            item: item_id,
            table: input.table,
            qty: item_qty
        };
        let db  = database::get_db_conn().await;
        let res = order::OrderAccessController::create_order(db.unwrap(),order).await;
        match res{
            Ok(_) => { rows_affected+=res.unwrap() }
            Err(_) => {break;}
        }
    }
    rows_affected.to_string()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/api", routes![index])
        .mount("/api", routes![create_order])
}

#[cfg(test)]
#[path="_tests/rest_item.rs"]
mod tests;
