
// Date Time
use chrono::prelude::*;

// use db function
use db::DB;

use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use warp::{Filter, Rejection};


// modules import
mod db;
mod error;
mod handler;


// declare two types
type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;


// debug for printing
// serial -> struct to json
// deserial -> json to struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Book{

    pub id: String,
    pub name: String,
    pub author: String,
    pub num_pages: usize,
    pub added_at: DateTime<Utc>,
    pub tags: Vec<String>,

}


#[tokio::main]
async fn main() -> Result<()> {

    println!("Inside main function");

    println!(" ");


    let db = DB::init().await?;


    let book_path = warp::path("book");


    let book_routes = book_path
    .and(warp::post())
    .and(warp::body::json()) // extracts json from http request body (sent by client), the client send json always , then it converst to bookRequest rust struct, passes to handler as param
    .and(with_db(db.clone()))
    .and_then(handler::create_book_handler)
    .or(book_path
    .and(warp::get())
    .and(with_db(db.clone()))
    .and_then(handler::books_list_handler))
    .or(book_path
    .and(warp::put())
    .and(warp::path::param())
    .and(warp::body::json())
    .and(with_db(db.clone()))
    .and_then(handler::edit_book_handler))
    .or(book_path
    .and(warp::delete())
    .and(warp::path::param())
    .and(with_db(db.clone()))
    .and_then(handler::delete_book_handler));

    let routes = book_routes.recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0,0,0,0], 8080)).await;

    Ok(())
}


// Motive:
   // provide every handler a cloned db

// filter in warp is simple response
// returning an implement trait
// impl here doesnt means we are implementing anything


// ////// what is Filter ?? 
// Trait Filter has Extract and Error


fn with_db(db: DB) -> impl Filter<Extract = (DB,) , Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
