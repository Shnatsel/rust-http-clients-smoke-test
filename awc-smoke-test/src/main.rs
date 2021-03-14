use std::{env, io::Write};
use std::time::Duration;

use tokio::time::timeout;

fn main() {
    match smoke_test() {
        Ok(()) => println!("Did not hang! Success"),
        Err(err) => {
            println!("Did not hang! Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}",
        env::args().skip(1).next().expect("No URL provided")
    );
    println!("Fetching {}", url);

    let client = awc::ClientBuilder::new()
        .max_redirects(10)
        .timeout(Duration::from_secs(10))
        .finish();
    let mut response = futures::executor::block_on(client.get(&url).send())?;

    println!("HTTP status code: {}", response.status());

    // Print headers
    for header_name in response.headers().keys() {
        for value in response.headers().get_all(header_name) {
            println!("Header: {}: {:?}", header_name, value);
        }
    }

    // Print the first 8kb of the body to get an idea of what we've downloaded, ignore the rest.
    // awc does not support any encoding conversion, so this will be in whatever encoding we received
    let body = futures::executor::block_on(timeout(Duration::from_secs(30), response.body()))??;
    // Print the first 8k chars of the body to get an idea of what we've downloaded, ignore the rest.
    let first_8kb_of_body: Vec<u8> = body.iter().take(8192).copied().collect();
    std::io::stdout().write_all(&first_8kb_of_body)?;
    print!("\n");
    Ok(())
}
