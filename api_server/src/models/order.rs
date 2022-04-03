use diesel::RunQueryDsl;
use crate::database::Db;
use serde::{Serialize, Deserialize};
use rocket::form::Form;
use sqlx::postgres::PgQueryResult;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub item: i32,
    pub table: i32,
    pub qty: i32,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct FormOrder {
    pub item_qty: String,
    pub table: i32,
}

pub struct OrderAccessController;

impl OrderAccessController {
    pub async fn create_order(db: Db, order: Order) -> Result<i32, sqlx::Error> {
        let sql = "INSERT INTO Orders values(nextval('order_id_seq'),$1,$2,$3)";
        let query = sqlx::query(&sql)
            .bind(order.item)
            .bind(order.table)
            .bind(order.qty)
            .execute(&db).await?;
        Ok(query.rows_affected().to_string().parse::<i32>().unwrap())
    }
}