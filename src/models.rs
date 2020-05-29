// Standard lib: None
use std::env;
// External crates - Primary: None

use deadpool_postgres::{  Manager, Client, Pool};
use tokio_postgres::{Config,Row};
// External crates - Utilities
use serde::{Serialize, Deserialize};

// Other internal modules: None
use crate::errors;
use crate::errors::{MyError};

// Const and type declarations: None
//type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Result<T> = std::result::Result<T, errors::MyError>;
const TABLE: &str = "todo";
// Struct declarations

// To do struct that represents the database table
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub checked: bool,
  //  pub priority: Option<String>,
}

// Struct for sending parameters to create Todo item
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoCreate {
    pub name: String,
}

//Struct for sending parameters to update Todo item
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoUpdate {
    pub name: Option<String>,
    pub checked: Option<bool>,
}



pub fn create_pool() -> std::result::Result<Pool, warp::Rejection> {
    let mut cfg = Config::new();
    cfg.host(&env::var("HOST").unwrap());
    cfg.user(&env::var("USER").unwrap());
    cfg.password(&env::var("PASSWORD").unwrap());
    cfg.dbname(&env::var("DBNAME").unwrap());
    let port = &env::var("DBPORT").unwrap();
    cfg.port(port.parse::<u16>().unwrap());
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let pool = Pool::new(mgr, 16);

    Ok(pool)
}

pub async fn get_db_con(db_pool: &Pool) -> std::result::Result<deadpool_postgres::Client, warp::Rejection> {
    Ok(db_pool.get().await.unwrap())
}


// Function to convert database row to Rust Todo struct
fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get("id");
    let name: String = row.get("name");
    let checked = row.get("checked");
   // let priority = Some("".into());
    Todo { id, name,checked }
}


// Function to fetch todos from database
pub async fn fetch_to_dos(db_pool: &Pool) -> Result<Vec<Todo>> {
  //  let client = db_pool.get().await.unwrap();
  let client: Client = get_db_con(db_pool).await.unwrap();
    let rows = client
        .query("SELECT id, name, checked from todo", &[])
        .await
        .map_err(MyError::DBQueryError)?;
   
        Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
    
}

// Function to create a Todo
pub async fn create_todo(db_pool: &Pool, body: TodoCreate) -> Result<Todo> {
 //   let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name,checked) VALUES ($1,$2) RETURNING *", TABLE);
    let con: Client = db_pool.get().await.map_err(MyError::DBPoolError)?;

    let b: bool = false;
 //   let p: String = String::from("low");
    let row = con
        .query_one(query.as_str(), &[&body.name,&b])
        .await
        .map_err(MyError::DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn update_todo(db_pool: &Pool, id: i32, body: TodoUpdate) -> Result<Todo> {
    let con: Client = db_pool.get().await.map_err(MyError::DBPoolError)?;
    let chkquery = con
    .query_one("SELECT id, name, checked from todo WHERE id = $1", &[&id])
    .await
    .map_err(MyError::NotFound)?;
    println!("Before update 1: rec is {:#?}",chkquery);
    let mut rec_to_update = row_to_todo(&chkquery);
    println!("Before update 2: rec is {:#?}",rec_to_update);
    if let Some(name) = body.name {
        rec_to_update.name = name
    }
    if let Some(checked) = body.checked {
        rec_to_update.checked = checked
    }
    let query = format!(
        "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&rec_to_update.name, &rec_to_update.checked, &id])
        .await
        .map_err(MyError::DBQueryError)?;
    Ok(row_to_todo(&row))
}

pub async fn delete_todo(db_pool: &Pool, id: i32) -> Result<i32> {
    let con: Client = db_pool.get().await.map_err(MyError::DBPoolError)?;
    let _chkquery = con
    .query_one("SELECT name from todo WHERE id = $1", &[&id])
    .await
    .map_err(MyError::NotFound)?;
    
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    let res = con.execute(query.as_str(), &[&id])
        .await
        .map_err(MyError::DBQueryError)?;
        
        Ok(res as i32)
}
