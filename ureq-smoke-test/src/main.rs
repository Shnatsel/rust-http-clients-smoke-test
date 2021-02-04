// Based on https://gist.github.com/deltaphc/2949ed292c7d1169e744e5ffa7fd0687
// (used with permission, licensed under MIT/Apache 2.0)
// and https://github.com/algesten/ureq/blob/main/examples/smoke-test/main.rs

use std::env;
use std::time::Duration;

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

    let agent = ureq::builder()
        .timeout_connect(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build();

    let response = agent.get(&url).call()?;
    println!("HTTP status code: {}", response.status());

    // Print headers
    for header_name in response.headers_names() {
        for value in response.all(&header_name) {
            println!("Header: {}: {:?}", header_name, value);
        }
    }

    // This retains the whole body in memory, but tests show that RAM is plentiful, so I've rolled back clever optimizations.
    // Converting to a string lets us exercise encoding conversion routines.
    let text = response.into_string()?;
    // Print the first 8k chars of the body to get an idea of what we've downloaded, ignore the rest.
    let first_8k_chars_of_body: String = text.chars().take(8192).collect();
    println!("{}", first_8k_chars_of_body);

    Ok(())
}
