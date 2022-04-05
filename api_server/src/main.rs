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
use sqlx::encode::IsNull::No;
use sqlx::Error;
use tokio::io::split;
use crate::database::Db;
use crate::item::Item;
use crate::order::Order;
use crate::order::FormOrder;

//get all items in DB from item table
#[get("/items")]
async fn get_all_items() -> String {
    //TODO: refactor match to breakout statement
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

/**
Get all current orders by table id and item id
item_id is optional parameter , if None , all items returned
*/
#[get("/current_orders/<table>?<item_id>")]
async fn get_current_orders_by_table(table : i32 , item_id : Option<u32>) -> String{
    let db = database::get_db_conn().await;
    if db.is_err(){
        return "Unable to connect to database".to_string()
    }
    let res = order::OrderAccessController::get_all_orders_by_table(db.unwrap(),table,item_id).await;
    match res{
        Ok(orders) => serde_json::to_string(&orders).unwrap(),
        Err(_) => "Unable to fetch orders".to_string()
    }
}

/**
Create new order from posted form data
params :
item_qty -> item quantity , format : itemID1_qty1,itemID2_qty2,...
table -> table ID
*/
#[post("/order", data = "<input>")]
async fn create_order(input : Form<FormOrder>) -> String{
    //parse input items into vector
    let mut data : Vec<&str> = input.item_qty.split(",").collect();
    let mut rows_affected : i32 = 0;

    for item in data{
        //parse item id and quantity
        let details_str = item.split("_");
        let vec : Vec<&str> = details_str.collect();
        let item_id : i32 = vec[0].parse::<i32>().unwrap();
        let item_qty : i32 = vec[1].parse::<i32>().unwrap();
        let order = Order {
            id: 0,
            item_id,
            table_id: input.table,
            quantity: item_qty
        };
        let db  = database::get_db_conn().await;
        if db.is_err(){
            return "Unable to connect to database".to_string()
        }
        let res = order::OrderAccessController::create_order(db.unwrap(),order).await;
        match res{
            Ok(_) => { rows_affected+=res.unwrap() }
            Err(_) => {break;}
        }
    }
    rows_affected.to_string()
}
/**
 Delete an item from an order.
params :
table -> table ID
item_id -> item ID
quantity -> number of items to delete. If not specified or is greater than current number of items in order , all items deleted.
 */
#[delete("/cancel_item/<table>/<item_id>?<quantity>")]
async fn cancel_item_from_order(table : i32 , item_id : i32 , quantity : Option<u32>) -> String{
    let db = database::get_db_conn().await;
    if db.is_err(){
        return "Unable to connect to database".to_string()
    }

    let res = order::OrderAccessController::delete_item_from_table(db.unwrap(),table,item_id,quantity).await;
    match res{
        Ok(orders) => serde_json::to_string(&orders).unwrap(),
        Err(_) => "Unable to remove items".to_string()
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/api", routes![get_all_items])
        .mount("/api", routes![create_order])
        .mount("/api", routes![get_current_orders_by_table])
        .mount("/api",routes![cancel_item_from_order])
}

#[cfg(test)]
#[path="_tests/rest_item.rs"]
mod tests;
