/*

broadcast::channelでRecieverが受信できるのはrecieverがsubscribeで生成されてからの物？
ギリギリ最後の奴だけ受信できる？
send 1
send 2
tx create
send 3
tx recv 3
...

*/
#![allow(unused)]
use std::time::Duration;
//
use tracing::{debug, error, info, instrument};
//
use tokio::sync::broadcast::{self, Receiver, Sender};

#[tokio::main]
async fn main() {
    init_logger();
    let (tx, mut rx) = broadcast::channel::<u32>(16);
    let tx_clone = tx.clone();

    // 2つのタスクを spawn する。
    // タスク1 はキーによる "get" を行い、// タスク2 は値を "set" する。
    let t1 = tokio::spawn(produce(tx));

    let mut handles = Vec::new();
    for i in 0..5 {
        let t2 = tokio::spawn(consume(i, tx_clone.subscribe()));
        handles.push(t2);
        tokio::time::sleep(Duration::new(1, 0)).await;
    }
    drop(tx_clone); // ここでclone元をdropしておかないとrecver側が閉じたのを検出できない
    drop(rx); // こっちはまぁそのままでも何とかなる

    let x = t1.await.unwrap();
    futures_util::future::join_all(handles).await;
}

#[instrument(skip_all)]
async fn produce(tx: Sender<u32>) -> () {
    info!("produce() enter");
    for i in 1..=10 {
        let rest_reciever = tx.send(i).unwrap();
        info!("rest reciever: {}", rest_reciever);

        tokio::time::sleep(Duration::new(1, 0)).await;
    }
}

#[instrument(skip(rx))]
async fn consume(id: usize, mut rx: Receiver<u32>) -> () {
    info!("consume() enter");
    loop {
        let r = rx.recv().await;
        if let Err(e) = r {
            error!("error: {}", e);
            break;
        }

        info!("recv: {}", r.unwrap());
    }
}

////////////////////////////////////////////////////////////////////////////////
fn init_logger() {
    let builder = tracing_subscriber::FmtSubscriber::builder()
        .with_thread_ids(true)
        // .with_max_level(tracing::Level::TRACE);
        .with_max_level(tracing::Level::DEBUG);
    // .with_max_level(tracing::Level::INFO);
    let subscriber = builder.finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
