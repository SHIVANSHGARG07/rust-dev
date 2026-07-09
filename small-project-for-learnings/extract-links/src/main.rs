

use error_chain::error_chain;

use select::document::Document;
use select::predicate::Name;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {


    let res = reqwest::get("https://www.google.com")
    .await?
    // read response body and convert it into string
    // another asyn operation because reading data from network is async
    .text()
    .await?;


    // till here we have a String Response
    // Document::from expects a &str slice from code , so convert to as_str, it doesnt need ownership
    Document::from(res.as_str())


    // only want tags with name a
    // it doesnt return a Vec
    // returns an iterator
    .find(Name("a"))

    // till here we have an iterator of Node
    // for each node get its href attribute
    .filter_map(|n| n.attr("href"))

    // for each string print them 
    .for_each(|x| println!("{}", x));

    Ok(())
}