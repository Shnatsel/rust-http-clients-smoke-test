use std::{env, time::Duration};

fn main() {
    match async_std::task::block_on(smoke_test_with_timeout()) {
        Ok(()) => println!("Did not hang! Success"),
        Err(err) => {
            println!("Did not hang! Error: {}", err);
            std::process::exit(1);
        }
    }
}

// There is currently no way to set timeouts in surf, see
// https://github.com/http-rs/surf/issues/274
// So we set a timeout on the entire request externally
async fn smoke_test_with_timeout() -> Result<(), Box<dyn std::error::Error>> {
    async_std::future::timeout(Duration::from_secs(45), smoke_test()).await??;
    Ok(())
}

async fn smoke_test() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}",
        env::args().skip(1).next().expect("No URL provided")
    );
    println!("Fetching {}", url);

    let mut response = surf::get(&url).send().await?;

    println!("HTTP status code: {}", response.status());

    // Print headers
    for (header_name, header_values) in response.iter() {
        for value in header_values {
            println!("Header: {}: {:?}", header_name, value);
        }
    }

    // This retains the whole body in memory, but tests show that RAM is plentiful, so I didn't bother optimizing.
    // Converting to a string lets us exercise encoding conversion routines.
    let text = response.body_string().await?;
    // Print the first 8k chars of the body to get an idea of what we've downloaded, ignore the rest.
    let first_8k_chars_of_body: String = text.chars().take(8192).collect();
    println!("{}", first_8k_chars_of_body);
    Ok(())
}