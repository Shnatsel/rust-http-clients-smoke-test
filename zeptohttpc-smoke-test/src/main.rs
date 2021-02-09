use std::env;
use std::time::Duration;

use zeptohttpc::{RequestBuilderExt, RequestExt, ResponseExt};

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

    let req = zeptohttpc::http::Request::get(&url).empty()?;

    let mut opts = zeptohttpc::Options::default();
    opts.connect_timeout = Duration::from_secs(10);
    opts.timeout = Some(Duration::from_secs(30));
    opts.follow_redirects = Some(10);

    let response = req.send_with_opts(opts)?;

    println!("HTTP status code: {}", response.status());

    // Print headers
    for header_name in response.headers().keys() {
        for value in response.headers().get_all(header_name).iter() {
            println!("Header: {}: {:?}", header_name, value);
        }
    }

    // This retains the whole body in memory, but tests show that RAM is plentiful, so I didn't bother optimizing.
    // Converting to a string lets us exercise encoding conversion routines.
    let text = response.into_string()?;
    // Print the first 8k chars of the body to get an idea of what we've downloaded, ignore the rest.
    let first_8k_chars_of_body: String = text.chars().take(8192).collect();
    println!("{}", first_8k_chars_of_body);
    Ok(())
}