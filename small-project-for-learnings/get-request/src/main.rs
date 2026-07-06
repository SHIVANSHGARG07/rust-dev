use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


fn main() -> Result<()>{

    // // used to block thread
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;


    drop(res);

    let mut body = String::new();

    // res has its own read to string
    res.read_to_string(&mut body)?;

    // println!("Status :{}", res.status());

    // println!("Headers: {:?}", res.headers());

    // println!("Body: {}", body);

    Ok(())




}