use std::env;
use ethers::prelude::*;
use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    let ws_string =
    env::var("WS_1").expect("You must set the WS_1 environment var! like: wss://infura.....");

    let ws = Ws::connect(ws_string).await?;
    let provider = Provider::new(ws).interval(Duration::from_millis(2000));
    let mut stream = provider.subscribe_pending_txs().await?;
    
    while let Some(tx) = stream.next().await {
        println!("{}",tx);
        let transaction = provider.get_transaction(tx).await?;
        dbg!(transaction);

    }

    Ok(())
}
 