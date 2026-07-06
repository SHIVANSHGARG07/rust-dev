// ❌ THIS WILL NOT COMPILE - Shows the problem we're solving

use std::io::Read;

fn main() {
    match run() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}

// ❌ PROBLEM: What type should go in Result<(), ???>?
// We have TWO different error types in this function!
fn run() -> Result<(), Box<dyn std::error::Error>> {
    // This returns reqwest::Error
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    
    // This returns std::io::Error
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    
    println!("Status: {}", res.status());
    println!("Body: {}", body);
    
    // Solution: Use Box<dyn std::error::Error> but this is ugly and loses type info
    Ok(())
}
