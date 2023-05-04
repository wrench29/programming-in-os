use std::time::Duration;

use anyhow::Result;

use crate::queue;

pub struct ConsoleApp {}

impl ConsoleApp {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self) -> Result<()> {
        let connection_config = queue::RabbitQueueConnnectionConfig::default();
        let queue = queue::RabbitQueue::new(&connection_config).await?;

        queue.declare_queue("test_queue").await?;

        queue.send("test_queue", "test message".into()).await?;

        tokio::time::sleep(Duration::from_secs(1)).await;

        queue.close().await?;

        Ok(())
    }
}

fn my_callback(data: Vec<u8>) {
    println!("{}", String::from_utf8(data).unwrap());
}
