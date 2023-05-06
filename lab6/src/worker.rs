use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};

use crate::queue;

const TN_QUEUE_TASKS: &str = "thumbnails_tasks";
const TN_QUEUE_RESULTS: &str = "thumbnails_results";

pub struct Worker {}

impl Worker {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self) -> Result<()> {
        let connection_config = queue::RabbitQueueConnnectionConfig::default();
        let queue = queue::RabbitQueue::new(&connection_config).await?;

        queue.declare_queue(TN_QUEUE_TASKS).await?;
        queue.declare_queue(TN_QUEUE_RESULTS).await?;

        let (sender, task_done_signal) = bounded::<String>(100);

        queue
            .set_recv_callback(TN_QUEUE_TASKS, move |data| {
                sender.send(String::from("Test message")).unwrap();
            })
            .await?;

        let ctrl_c_events = ctrl_channel()?;
        let ticks = tick(Duration::from_secs(1));

        loop {
            select! {
                recv(ticks) -> _ => {
                    println!("Awaiting for job...");
                }
                recv(ctrl_c_events) -> _ => {
                    println!();
                    println!("Interruption received, shutting down!");
                    break;
                }
                recv(task_done_signal) -> data => {
                    let data = data.unwrap();
                    println!("{}", data);
                    queue.send(TN_QUEUE_RESULTS, data.into()).await?;
                }
            }
        }

        queue.close().await?;

        Ok(())
    }
}

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
