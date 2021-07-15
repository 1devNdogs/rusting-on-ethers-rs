use std::env;
use ethers::prelude::*;
use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    let ws_string =
    env::var("WS_1").expect("You must set the WS_1 environment var! like: wss://infura.....");

    let ws = Ws::connect(ws_string).await?;
    let provider = Provider::new(ws).interval(Duration::from_millis(2000));
    let mut stream = provider.watch_blocks().await?.stream();
    
    while let Some(block) = stream.next().await {
        dbg!(block);
    }

    Ok(())
}
 
