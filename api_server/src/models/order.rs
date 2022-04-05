use diesel::RunQueryDsl;
use crate::database::Db;
use serde::{Serialize, Deserialize};
use rocket::form::Form;
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::{Error, Row};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub item_id: i32,
    pub table_id: i32,
    pub quantity: i32,
}

//Model for posted form data to create order
#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct FormOrder {
    pub item_qty: String,
    pub table: i32,
}

//Model to get list of items according to table id
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct TableItems{
    pub order_id : i32,
    pub item_name : String,
    pub item_qty : i32,
    pub total_time_to_cook : i32,
}

pub struct OrderAccessController;

impl OrderAccessController {
    //Creates a new order with item_id , table_id and item quantity
    pub async fn create_order(db: Db, order: Order) -> Result<i32, sqlx::Error> {
        let sql = "INSERT INTO Orders values(nextval('order_id_seq'),$1,$2,$3)";
        let query = sqlx::query(&sql)
                                        .bind(order.item_id)
                                        .bind(order.table_id)
                                        .bind(order.quantity)
                                        .execute(&db)
                                        .await?;
        Ok(query.rows_affected() as i32)
    }

    //Get a list of all orders according to table id
    pub async fn get_all_orders_by_table(db: Db, table: i32 , item_id : Option<u32>) -> Result<Vec<TableItems>, sqlx::Error> {
        let mut sql = "SELECT orders.id as order_id,item.name as item_name,orders.quantity as item_qty,(orders.quantity * item.time_to_cook) as total_time_to_cook
                                                            FROM orders
                                                            INNER JOIN item on orders.item_id=item.id
                                                            where table_id=$1";
        if item_id.is_some(){
            let final_sql : String = sql.to_owned() + " AND item.id=$2";
            let query = sqlx::query_as(&*final_sql)
                                                .bind(table)
                                                .bind(item_id)
                                                .fetch_all(&db)
                                                .await?;
            return Ok(query)
        }

        let query = sqlx::query_as(sql)
                                            .bind(table)
                                            .fetch_all(&db)
                                            .await?;
        Ok(query)
    }

    //Remove and item completely from table or reduce the quantity
    pub async fn delete_item_from_table(db : Db, table : i32, item_id : i32, item_qty_to_remove: Option<u32>) -> Result<i32 , sqlx::Error>{
        let find_current_qty_sql = "SELECT quantity from orders WHERE item_id=$1 AND table_id=$2";
        let current_qty_query = sqlx::query(find_current_qty_sql)
                                                .bind(item_id)
                                                .bind(table)
                                                .fetch_one(&db)
                                                .await?;

        let current_qty : i32 = current_qty_query.try_get("quantity").unwrap();

        if item_qty_to_remove.is_some(){
            let diff : i32 = current_qty-(item_qty_to_remove.unwrap() as i32);
            if  diff.is_positive() {
                let update_sql = "UPDATE orders SET quantity=$1 WHERE item_id=$2 AND table_id=$3";
                let update_query = sqlx::query(update_sql)
                    .bind(diff)
                    .bind(item_id)
                    .bind(table)
                    .execute(&db)
                    .await?;
                return Ok(update_query.rows_affected() as i32)
            }
        }

        //if difference in quantities is negaitve or quantity is not set , delete row
        let delete_sql = "DELETE from orders WHERE item_id=$1 AND table_id=$2";
        let delete_query = sqlx::query(delete_sql)
            .bind(item_id)
            .bind(table)
            .execute(&db)
            .await?;
        Ok(delete_query.rows_affected() as i32)
    }
}

#[cfg(test)]
#[path="../_tests/rest_order.rs"]
mod tests;