use mio_httpc::CallBuilder;
use std::{env, io::Write};

fn main() {
    match smoke_test() {
        Ok(()) => println!("Did not hang! Success"),
        Err(err) => {
            println!("Did not hang! Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn smoke_test() -> Result<(), mio_httpc::Error> {
    let url = format!(
        "http://{}",
        env::args().skip(1).next().expect("No URL provided")
    );
    println!("Fetching {}", url);

    let (response, body) = CallBuilder::get()
        .url(&url)?
        //.max_redirects(10)
        .timeout_ms(40_000)
        .exec()?;

    println!("HTTP status code: {}", response.status);

    // Print headers
    for header in response.headers() {
        println!("Header: {}: {:?}", header.name, header.value);
    }
    
    // Print the first 8kb of the body to get an idea of what we've downloaded, ignore the rest.
    // mio_httpc does not support any encoding conversion, so this will be in whatever encoding we received
    let first_8kb_of_body: Vec<u8> = body.iter().take(8192).copied().collect();
    std::io::stdout().write_all(&first_8kb_of_body)?;
    print!("\n");
    Ok(())
}
