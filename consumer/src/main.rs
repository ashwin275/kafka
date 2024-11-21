

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::config::ClientConfig;
use rdkafka::message::Message;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rust-consumer-group") 
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .create()
        .expect("Consumer creation failed");

    let topic = "test-topic";

    consumer
        .subscribe(&[topic])
        .expect("Failed to subscribe to topic");

    println!("Consumer started. Listening for messages...");

    while let Ok(msg) = consumer.recv().await {
        match msg.payload_view::<str>() {
            Some(Ok(payload)) => {
                println!("Received: {}", payload);
            }
            Some(Err(e)) => {
                eprintln!("Error decoding message payload: {}", e);
            }
            None => {
                println!("Received a message with no payload.");
            }
        }
    }
}
