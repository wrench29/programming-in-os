use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};
use image::{
    imageops::{resize, FilterType},
    io::Reader as ImageReader,
};
use json::object;
use std::{path::Path, time::Duration};

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
                println!("Received message, making thumbnail.");

                let data_json = json::parse(&String::from_utf8(data).unwrap()).unwrap();
                let path = data_json["image_path"].as_str().unwrap();
                let pref_width = data_json["pref_width"].as_usize().unwrap();
                let result = Worker::make_thumbnail(path, pref_width).unwrap();
                sender.send(result).unwrap();

                println!("Thumbnail has been made.");
            })
            .await?;

        let ctrl_c_events = ctrl_channel()?;
        let ticks = tick(Duration::from_secs(2));

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
                    let data_json = object!{
                        path: data
                    };
                    queue.send(TN_QUEUE_RESULTS, data_json.dump().into()).await?;
                }
            }
        }

        queue.close().await?;

        Ok(())
    }

    fn make_thumbnail(path: &str, pref_width: usize) -> Result<String> {
        let img = ImageReader::open(path)?.decode()?;
        let ratio = img.height() as f32 / img.width() as f32;
        let pref_height = ratio * pref_width as f32;
        let buffer = resize(
            &img,
            pref_width as u32,
            pref_height as u32,
            FilterType::Nearest,
        );

        let image_path = Path::new(path);
        let image_name = image_path.file_name().unwrap();

        let image_path_str = image_path.to_str().unwrap();
        let image_path_len = image_path_str.len();
        let image_folder = Path::new(&image_path_str[..image_path_len - image_name.len()]);

        let new_filename_str = image_name.to_str().unwrap();
        let new_image_path = vec![
            &new_filename_str[..new_filename_str.len() - 4],
            &format!("_{}", pref_width),
            &new_filename_str[new_filename_str.len() - 4..],
        ]
        .join("");

        let new_file_path = image_folder.join(new_image_path);
        buffer.save(&new_file_path)?;

        Ok(String::from(new_file_path.to_str().unwrap()))
    }
}

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
