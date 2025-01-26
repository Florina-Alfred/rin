use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        println!("rx1: {:?}", rx1.recv().await);
        println!("rx1: {:?}", rx1.recv().await);
        println!("rx1: {:?}", rx1.recv().await);
        println!("rx1: {:?}", rx1.recv().await);
    });

    tokio::spawn(async move {
        println!("rx2: {:?}", rx2.recv().await);
        println!("rx2: {:?}", rx2.recv().await);
        println!("rx2: {:?}", rx2.recv().await);
        println!("rx2: {:?}", rx2.recv().await);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    tx_clone.send(10).unwrap();
    tx_clone.send(20).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

