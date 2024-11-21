use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092") 
        .create()
        .expect("Producer creation error");

    let topic = "test-topic";

    for i in 0..10 {
        let message = format!("Message {}", i);
        producer
            .send(
                FutureRecord::to(topic)
                    .payload(&message)
                    .key(&format!("Key {}", i)),
                Duration::from_secs(0),
            )
            .await
            .expect("Failed to send message");
        println!("Sent: {}", message);
    }
}
