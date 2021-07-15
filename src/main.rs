mod blocks;
mod pending;

use futures::executor::block_on;


#[tokio::main]
async fn main()  {
    let future = async_main();
    block_on(future);
}



async fn async_main() {

    let async_one = blocks::run();
    let async_two = pending::run();

    futures::join!(async_one, async_two);
}
