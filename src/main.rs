use std::io::Result;

use nats::jetstream::{self, JetStream, StreamConfig, SubscribeOptions};

const SCRATCH: &str = "orders.scratch";
const QWERTY: &str = "orders.qwerty";

fn main() -> std::io::Result<()> {
    let nc = nats::Options::with_user_pass("vinicius", "vinicius")
        .with_name("rustlang")
        .connect("localhost:4222")?;

    let js = jetstream::new(nc);

    js.add_stream(StreamConfig {
        name: "orders".to_string(),
        subjects: vec!["orders.*".to_string()],
        ..Default::default()
    })?;

    js.publish(SCRATCH, b"scratch")?;
    js.publish(QWERTY, b"qwerty")?;

    subscribe(&js, SCRATCH)?;
    subscribe(&js, QWERTY)?;

    loop {}
}

fn subscribe(js: &JetStream, topic: &str) -> Result<()> {
    let consumer = topic.replace(".", "_");
    let options = SubscribeOptions::new().durable_name(consumer);

    js.subscribe_with_options(topic, &options)?
        .with_process_handler(|msg| {
            println!(
                "Received a message: {:?}",
                String::from_utf8(msg.data.clone()).unwrap_or_default()
            );

            Ok(())
        });

    Ok(())
}
