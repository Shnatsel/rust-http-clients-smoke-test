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

    let client = reqwest::blocking::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(10))
        // This is a read timeout, not a full request timeout
        // It will not protect from malicious remote hosts that try to DoS us by keeping the connection open forever
        // https://github.com/seanmonstar/reqwest/issues/1161
        .timeout(Duration::from_secs(30))
        .build()?;

    let response = client.get(&url).send()?;

    println!("HTTP status code: {}", response.status());

    // Print headers
    for header_name in response.headers().keys() {
        for value in response.headers().get_all(header_name).iter() {
            println!("Header: {}: {:?}", header_name, value);
        }
    }

    // This retains the whole body in memory, but tests show that RAM is plentiful, so I didn't bother optimizing.
    // Converting to a string lets us exercise encoding conversion routines.
    let text = response.text()?;
    // Print the first 8k chars of the body to get an idea of what we've downloaded, ignore the rest.
    let first_8k_chars_of_body: String = text.chars().take(8192).collect();
    println!("{}", first_8k_chars_of_body);
    Ok(())
}
