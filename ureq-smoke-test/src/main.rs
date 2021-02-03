// Based on https://gist.github.com/deltaphc/2949ed292c7d1169e744e5ffa7fd0687
// (used with permission, licensed under MIT/Apache 2.0)
// and https://github.com/algesten/ureq/blob/main/examples/smoke-test/main.rs

use std::env;
use std::io::Read;
use std::time::Duration;

fn main() {
    match smoke_test() {
        Ok(()) => println!("\nDid not hang! Success"),
        Err(err) => {
            println!("\nDid not hang! Error: {}", err);
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
        println!("Header: {}: {:?}", header_name, response.all(&header_name));
    }

    // Read the first 16Kb of the body to get an idea of what we've downloaded, ignore the rest.
    let mut reader = response.into_reader();
    print_first_n_bytes(&mut reader, 16384)?;
    // If we don't do this, the rest of the body will never actually be read or maybe even received.
    drain_reader(&mut reader)?;

    Ok(())
}

fn print_first_n_bytes(reader: &mut impl Read, bytes: usize)-> Result<(), Box<dyn std::error::Error>> {
    let mut limited_reader = reader.take(bytes as u64);
    drain_reader_to_stdout(&mut limited_reader)?;
    Ok(())
}

/// Reads all data from the provider reader and prints it to stdout
fn drain_reader_to_stdout(reader: &mut impl Read) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    std::io::copy(reader, &mut stdout)?;
    Ok(())
}

/// Reads all data from the provided reader and throws it away
fn drain_reader(reader: &mut impl Read) -> Result<(), Box<dyn std::error::Error>> {
    for byte in reader.bytes() {
        byte?;
    }
    Ok(())
}
