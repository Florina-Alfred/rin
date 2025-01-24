use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

#[tokio::main]
async fn main() {
    // let mut topic_channel: HashMap<String, Vec<String>> = HashMap::new();
    let mut topic_channel: Arc<Mutex<HashMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let topics = vec!["topic1", "topic2", "topic3"]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();

    let topic1_msgs = vec![
        "topic1_msg1",
        "topic1_msg2",
        "topic1_msg3",
        "topic1_msg4",
        "topic1_msg5",
        "topic1_msg6",
        "topic1_msg7",
        "topic1_msg8",
        "topic1_msg9",
        "topic1_msg10",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();
    let topic2_msgs = vec!["topic2_msg1", "topic2_msg2"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let topic3_msgs = vec![
        "topic3_msg1",
        "topic3_msg2",
        "topic3_msg3",
        "topic3_msg4",
        "topic3_msg5",
        "topic3_msg6",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

    let topic_channel_clone = topic_channel.clone();
    let insert_task = tokio::task::spawn(async move {
        for i in 0..10 {
            let mut topic_channel_guard = topic_channel_clone.lock().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            match topic_channel_guard.get_mut(&format!("topic{}", (i % 3) + 1)) {
                Some(channels) => {
                    let (tx, rx): (Sender<String>, Receiver<String>) = broadcast::channel(10);
                    channels.push(format!("rx{}", i));
                }
                None => {
                    topic_channel_guard
                        .insert(format!("topic{}", (i % 3) + 1), vec![format!("rx{}", i)]);
                }
            }
        }
    });
    println!("topics: {:?}", topic_channel);

    let topic_channel_clone = Arc::clone(&topic_channel);
    tokio::spawn(async move {
        for msg in topic1_msgs {
            println!("topic1: {:?}", msg);
            println!("topic_clone: {:?}", topic_channel_clone);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    let topic_channel_clone = Arc::clone(&topic_channel);
    tokio::spawn(async move {
        for msg in topic2_msgs {
            println!("topic2: {:?}", msg);
            println!("topic_clone: {:?}", topic_channel_clone);
            tokio::time::sleep(tokio::time::Duration::from_micros(10)).await;
        }
    });
    let topic_channel_clone = Arc::clone(&topic_channel);
    tokio::spawn(async move {
        for msg in topic3_msgs {
            println!("topic3: {:?}", msg);
            println!("topic_clone: {:?}", topic_channel_clone);
            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
        }
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    insert_task.await.unwrap();
}
