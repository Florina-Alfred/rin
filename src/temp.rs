use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);

    tokio::spawn(async move {
        for i in 1..=5 {
            // tx.send(i).unwrap();
            match tx.send(i) {
                Ok(_) => {
                    println!("Sent {}", i);
                }
                Err(e) => {
                    println!("sender Error: {}", e);
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        println!("Sender done");
    });

    loop {
        // let msg = rx1.recv().await.unwrap();
        match rx1.recv().await {
            Ok(msg) => {
                println!("GOT = {}", msg);
            }
            Err(e) => {
                println!("receiver Error: {}", e);
                break;
            }
        }
    }
    println!("Exiting loop");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
