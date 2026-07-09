

use actix_web::{get, App, HttpServer, Responder, web::Path};

use rhai::Engine;

use std::io::Result;


#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder{

    // get the numbers from path
    let(num1, num2) = path.into_inner();

    // create an instance of rhai engine
    let mut engine = Engine::new();

    // reginster an api that expose numbers to engine
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);


    // run the script
    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    // return ans
    format!("{result}")

}



#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder{

    // get the numbers from path
    let(num1, num2) = path.into_inner();

    // create an instance of rhai engine
    let mut engine = Engine::new();

    // reginster an api that expose numbers to engine
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);


    // run the script
    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    // return ans
    format!("{result}")

}



#[actix_web::main]
async fn main() -> Result<()>{

    HttpServer::new(||{
        App::new()
        .service(multiply)
        .service(add)
    })
    .bind(("0.0.0.0",7777))
    .unwrap()
    .run()
    .await



}