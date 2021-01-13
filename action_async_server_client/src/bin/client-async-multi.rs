extern crate async_std;
extern crate futures;

use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let now = Instant::now();

    let mut futs = FuturesUnordered::new();
    futs.push(task::spawn(job("task1", now.clone())));
    futs.push(task::spawn(job("task2", now.clone())));
    futs.push(task::spawn(job("task3", now.clone())));
    while let Some(_handled) = futs.next().await {}

    Ok(())
}

async fn job(label: &str, now: std::time::Instant) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Simulate network delay using async delay for 2 seconds
    println!(
        "OS Thread {:?} - {} started: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed(),
    );
    task::sleep(Duration::from_secs(2)).await;

    // Write to server - server will echo this back to us with 8 second delay
    let mut stream = TcpStream::connect("127.0.0.1:16142").await?;
    stream.write_all(label.as_bytes()).await?;
    println!(
        "OS Thread {:?} - {} written: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed()
    );

    // Read 5 chars we expect (to avoid dealing with EOF, etc.)
    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).await?;
    stream.shutdown(std::net::Shutdown::Both)?;
    println!(
        "OS Thread {:?} - {} read: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed()
    );

    // Simulate computation work by sleeping actual thread for 4 seconds
    sleep(std::time::Duration::from_secs(4));
    println!(
        "OS Thread {:?} - {} finished: {:?}",
        std::thread::current().id(),
        std::str::from_utf8(&buffer)?,
        now.elapsed()
    );
    Ok(())
}
