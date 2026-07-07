

use serde::Deserialize;

use reqwest:: { Error, header::{USER_AGENT}};


#[derive(Deserialize, Debug)]
struct User{
    login: String,
    id: u64,
    avatar_url: String,
    url: String,
}

#[tokio::main]
async fn main()-> Result<(), Error>{

    // format is string interpolation macro
    let url = format!("https://api.github.com/repos/{owner}/{repo}/contributors", owner="rust-lang", repo = "rust");


    println!("{}", url);

    let client = reqwest::Client::new();

    let response = client.get(&url).header(USER_AGENT, "rust api client demo").send().await?;

    // here we have multiple structus
    let users: Vec<User> = response.json().await?;


    println!("Users: {:?}", users);

    Ok(())

}
