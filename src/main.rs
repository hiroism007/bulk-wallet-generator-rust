use anyhow::Result;
use ethers::{
    prelude::{rand, Signer, Wallet},
    utils::hex::ToHex,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use serde_json::json;
use tokio::fs::OpenOptions;
use tokio::io;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();
    let count = Arc::new(Mutex::new(0));

    loop {
        if *count.lock().await >= 10000 {
            break;
        }

        let my_count = Arc::clone(&count);

        tokio::spawn(async move {
            let mut lock = my_count.lock().await;
            *lock += 1;
            let key = Wallet::new(&mut rand::thread_rng());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("./wallets/{:?}.json", lock))
                .await?;

            file.write_all(
                json!({
                    "address": key.address(),
                    "private_key": key.signer().to_bytes().encode_hex::<String>(),
                })
                .to_string()
                .as_bytes(),
            )
            .await?;
            Ok::<_, io::Error>(())
        });
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
