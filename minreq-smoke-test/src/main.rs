use std::env;

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

    let response = minreq::get(&url)
        .with_timeout(40)
        .with_max_redirects(10)
        .send()?;

    println!("HTTP status code: {}", response.status_code);

    // Print headers
    // Note: this is not a multimap, so duplicate values for the same header will be lost
    for (header_name, value) in response.headers.iter() {
        println!("Header: {}: {:?}", header_name, value);
    }

    // This retains the whole body in memory, but tests show that RAM is plentiful, so I didn't bother optimizing.
    // Converting to a string lets us exercise encoding conversion routines.
    let text = response.as_str()?;
    // Print the first 8k chars of the body to get an idea of what we've downloaded, ignore the rest.
    let first_8k_chars_of_body: String = text.chars().take(8192).collect();
    println!("{}", first_8k_chars_of_body);
    Ok(())
}
