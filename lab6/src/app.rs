use std::{path::Path, process, time::Duration};

use crate::queue;
use anyhow::Result;
use crossbeam_channel::{bounded, select};
use json::object;
use text_io::read;
use tokio::time::sleep;

pub struct ConsoleApp {}

const TN_QUEUE_TASKS: &str = "thumbnails_tasks";
const TN_QUEUE_RESULTS: &str = "thumbnails_results";

impl ConsoleApp {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self, is_automated: bool) -> Result<()> {
        let connection_config = queue::RabbitQueueConnnectionConfig::default();
        let queue = queue::RabbitQueue::new(&connection_config).await?;

        ctrlc::set_handler(move || {
            println!();
            println!("Interruption received, shutting down!");
            process::exit(0);
        })?;

        queue.declare_queue(TN_QUEUE_TASKS).await?;
        queue.declare_queue(TN_QUEUE_RESULTS).await?;

        let (sender, receiver) = bounded::<()>(100);

        queue
            .set_recv_callback(TN_QUEUE_RESULTS, move |data| {
                let data_str = String::from_utf8(data).unwrap();
                let data = json::parse(&data_str).unwrap();
                let data_path = data["path"].as_str().unwrap();
                println!("Done, thumbnail path: {}", data_path);
                sender.send(()).unwrap();
            })
            .await?;

        loop {
            println!("Press CTRL^C OR write `exit` to exit");
            print!("Write path to an image\n> ");
            let user_input_path: String = read!("{}\r\n");
            if user_input_path == "exit" {
                break;
            }
            let image_path = Path::new(&user_input_path);
            if !image_path.exists() || !image_path.is_file() {
                println!("ERROR: Invalid path.");
                continue;
            }
            print!("Write max width for thumbnail (leave empty for 32px)\n> ");

            let size_str: String = read!("{}\r\n");
            if size_str == "exit" {
                break;
            }
            let size = size_str.parse::<i32>().unwrap_or(32);
            let size = if size < 32 { 32 } else { size };

            let image_path_str = image_path.to_str().unwrap();
            let send_data = object! {
                image_path: image_path_str,
                pref_width: size,
            };
            let send_data_str = send_data.dump();
            queue.send(TN_QUEUE_TASKS, send_data_str.into()).await?;
            loop {
                select! {
                    recv(receiver) -> _ => {
                        break;
                    }
                    default => {
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }

        queue.close().await?;

        Ok(())
    }
}
