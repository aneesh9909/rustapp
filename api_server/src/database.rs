use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;
use diesel::dsl::sql;
use futures::StreamExt;
use rocket::form::validate::range;
use sqlx::{Pool, Database, Postgres, Error};
use sqlx::postgres::{PgPoolOptions, PgQueryResult, PgRow};
use rand::distributions::{Distribution, Uniform};


const PG_HOST : &str = "rustapp_db_1";
const PG_USER : &str = "postgres";
const PG_PASS : &str =  "supersecretpassword";
const PG_DB : &str = "rustydb";
const ROOT_DB : &str = "postgres";
const PG_MAX_CONNECTION : u32 = 20;

const SQL_DIR : &str = "src/sql/";
const SQL_RECREATE: &str = "src/sql/00-recreate-db.sql";

pub type Db = Pool<Postgres>;

//initialize the database tables and seed values for items and table numbers
pub async fn init_db() -> Result<Db,sqlx::Error>{

    let root_db = create_db_pool(PG_HOST,ROOT_DB,PG_USER,PG_PASS,1).await?;
    seed_db(&root_db,SQL_RECREATE).await?;

    let app_db = create_db_pool(PG_HOST,PG_DB,PG_USER,PG_PASS,1).await?;
    let mut sql_init_files : Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    sql_init_files.sort();

    for s in sql_init_files{
        if let Some(s) = s.to_str(){
            if s.ends_with(".sql") && s!=SQL_RECREATE{
                seed_db(&app_db,&s).await?;
            }
        }
    }
    seed_tables(&app_db).await?;
    create_db_pool(PG_HOST,PG_DB,PG_USER,PG_PASS,PG_MAX_CONNECTION).await
}

pub async fn get_db_conn() -> Result<Db,sqlx::Error>{
    create_db_pool(PG_HOST,PG_DB,PG_USER,PG_PASS,PG_MAX_CONNECTION).await
}

async fn seed_db(db : &Db , file : &str) -> Result<(),sqlx::Error>{
    // println!("{:?}",std::env::current_dir());
    //read SQL from files
    let content = fs::read_to_string(file).map_err(|e|{
        println!("Error reading {} , {:?}",file,e);
        e
    })?;

    let statement : Vec<&str> = content.split(";").collect();
    for s in statement{
        match sqlx::query(&s).execute(db).await{
            Ok(_) => (),
            Err(ex) => println!("Sql file '{}' FAILED cause: {:?}", file, ex),
        }
    }
    Ok(())
}

//create new db connection
async fn create_db_pool(host : &str , db: &str , user : &str , pwd : &str , max_conn : u32) -> Result<Db,sqlx::Error>{
    let conn_str = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_conn)
        .connect_timeout(Duration::from_millis(5000))
        .connect(&conn_str)
        .await
}

//seed table numbers 1-250
async fn seed_tables(db : &Db) -> Result<(),sqlx::Error> {
    let mut rng = rand::thread_rng();
    let seats = Uniform::from(1..5);
    for i in 1..=250{
        match sqlx::query(&*format!("INSERT INTO tables values({},{})", i, seats.sample(&mut rng)))
            .execute(db)
            .await{
            Ok(_) => (),
            Err(ex) => println!("Tables seed FAILED cause: {:?}", ex),
        }
    }
    Ok(())
}

#[cfg(test)]
#[path="_tests/init_db.rs"]
mod tests;

