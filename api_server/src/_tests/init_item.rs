use crate::database::init_db;
use crate::item::ItemAccessController;

#[tokio::test]
pub async fn db_item_get_all_test() -> Result<(),Box<dyn std::error::Error>>{
    let db = init_db().await?;
    let items = ItemAccessController::get_all_items(db).await?;
    assert_eq!(3,items.len());
    Ok(())
}

#[tokio::test]
pub async fn db_item_get_all_count_test() -> Result<(),Box<dyn std::error::Error>>{
    let db = init_db().await?;
    let items = ItemAccessController::get_all_items(db).await?;
    let ids : Vec<i32> = items.into_iter().map(|i| i.id).collect();
    assert_eq!(ids,[102,101,100]);
    Ok(())
}