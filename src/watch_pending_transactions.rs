use std::env;
use ethers::prelude::*;
use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    let ws_string =
    env::var("WS_2").expect("You must set the WS_2 environment var! like: wss://NOT_INFURA.....");
    // infura throw me an error where the method not exist, seems like works with subscribe_pending_txs

    let ws = Ws::connect(ws_string).await?;
    let provider = Provider::new(ws).interval(Duration::from_millis(2000));
    let mut stream = provider.watch_pending_transactions().await?.stream();
    
    while let Some(tx) = stream.next().await {
        dbg!(tx);
    }

    Ok(())
}
 