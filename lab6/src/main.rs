use std::time::Duration;

use anyhow::Result;

mod app;
mod queue;

#[tokio::main]
async fn main() -> Result<()> {
    let connection_config = queue::RabbitQueueConnnectionConfig::default();
    let queue = queue::RabbitQueue::new(&connection_config).await?;

    queue.declare_queue("test_queue").await?;
    // queue.send("test_queue", "test1".into()).await?;
    // queue.send("test_queue", "test2".into()).await?;
    // queue.send("test_queue", "test3".into()).await?;
    queue.set_recv_callback("test_queue", my_callback).await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    queue.close().await?;

    Ok(())
}

fn my_callback(data: Vec<u8>) {
    println!("{}", String::from_utf8(data).unwrap());
}
