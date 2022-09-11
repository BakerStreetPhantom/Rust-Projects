use crate::Clients;
use chrono::{DateTime, Utc};
use rand::prelude::*;
use serde::Serialize;
use tokio;
use tokio::time::Duration;
use warp::ws::Message;

#[derive(Serialize)]
struct TestData {
    widget_type: String,
    widget_count: u32,
    creation_date: DateTime<Utc>,
}

pub async fn main_worker(clients: Clients) {
    loop {
        tokio::time::sleep(Duration::from_millis(2000)).await;

        let connected_client_count = clients.lock().await.len();
        if connected_client_count == 0 {
            println!("No clients connected, skip sending data");
            continue;
        }
        println!("{} connected client(s)", connected_client_count);

        let test_data_batch = generate_random_data();
        clients.lock().await.iter().for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(
                    serde_json::to_string(&test_data_batch).unwrap(),
                )));
            }
        });
    }
}

fn generate_random_data() -> Vec<TestData> {
    let mut rng = rand::thread_rng();
    let mut test_data_batch = Vec::new();
    for i in 0..10 {
        test_data_batch.push(TestData {
            widget_type: String::from(format!("widget{}", i)),
            widget_count: rng.gen_range(0..100),
            creation_date: Utc::now(),
        });
    }
    return test_data_batch;
}