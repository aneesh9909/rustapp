use crate::database::Db;
use serde::{Serialize,Deserialize};
#[derive(sqlx::FromRow,Debug,Clone,Serialize,Deserialize)]
pub struct Item {
    pub id : i32,
    pub name : String,
    pub time_to_cook : i32
}

pub struct ItemAccessController;

impl ItemAccessController {
    pub async fn get_all_items(db : Db) -> Result<Vec<Item> , sqlx::Error>{
        let sql = "SELECT id,name,time_to_cook from item ORDER BY id desc";
        let query = sqlx::query_as(&sql);
        let items = query.fetch_all(&db).await?;
        Ok(items)
    }
}

#[cfg(test)]
#[path="../_tests/init_item.rs"]
mod tests;