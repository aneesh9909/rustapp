use crate::database::seed_tables;
use crate::Db;
use super::init_db;

#[tokio::test]
async fn init_db_test() -> Result<(),Box<dyn std::error::Error>>{
    let db = init_db().await?;
    let res = sqlx::query("select * from item").fetch_all(&db).await?;
    assert_eq!(3,res.len(),"count of seeded rows");
    Ok(())
}

#[tokio::test]
async fn tables_seed_test_test() -> Result<(),Box<dyn std::error::Error>>{
    let db = init_db().await?;
    let res = sqlx::query("select * from tables").fetch_all(&db).await?;
    assert_eq!(250,res.len(),"count of seeded rows");
    Ok(())
}