use error_chain::error_chain;

use std::io::copy;

use std::fs::File;
use tempfile::Builder;

use std::io::Cursor;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;

    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    let response = reqwest::get(target).await?;

    // know the path
    let file_path = tmp_dir.path().join("rust-logo.png");

    println!("Temp dir: {}", tmp_dir.path().display());
    println!("File path: {}", file_path.display());

    //create file which is empty nothing is created here or pasted here for now
    let mut file = File::create(file_path)?;

    // we have downloaded bytes and empty file
    //  with help of cursor we treat bytes in memory as if they are a file
    let mut content = Cursor::new(response.bytes().await?);

    copy(&mut content, &mut file)?;

    Ok(())
}
