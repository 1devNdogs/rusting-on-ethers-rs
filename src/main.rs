mod watch_blocks;
mod subscribe_blocks;
mod subscribe_pending_txs;
mod watch_pending_transactions;

use futures::executor::block_on;


#[tokio::main]
async fn main()  {
    let future = async_main();
    block_on(future);
}



async fn async_main() {

    let async_one = subscribe_blocks::run();
    let async_two = subscribe_pending_txs::run();
    let async_three = watch_blocks::run();
    let async_four = watch_pending_transactions::run();

    futures::join!(async_two);
//    futures::join!(async_one, async_two, async_three, async_four);

}
