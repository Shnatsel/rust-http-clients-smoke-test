// Based on https://gist.github.com/deltaphc/2949ed292c7d1169e744e5ffa7fd0687
// (used with permission, licensed under MIT/Apache 2.0)
// and https://github.com/algesten/ureq/blob/main/examples/smoke-test/main.rs

use std::env;
use std::io::Read;
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
    // If we don't do this, the body will never actually be read or maybe even received.
    let mut reader = response.into_reader();
    drain_reader(&mut reader)?;

    Ok(())
}

/// Reads all data from the provided reader without retaining it.
fn drain_reader(reader: &mut impl Read) -> Result<(), Box<dyn std::error::Error>> {
    for byte in reader.bytes() {
        byte?;
    }
    Ok(())
}

/// For testing purposes
#[allow(dead_code)]
fn drain_reader_to_stdout(reader: &mut impl Read) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    std::io::copy(reader, &mut stdout)?;
    Ok(())
}
