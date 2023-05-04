use async_trait::async_trait;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};

use anyhow::Result;

pub struct RabbitQueue {
    connection: Connection,
    channel: Channel,
}

impl RabbitQueue {
    pub async fn new(config: &RabbitQueueConnnectionConfig) -> Result<Self> {
        let connection = Connection::open(&OpenConnectionArguments::new(
            &config.host,
            config.port,
            &config.username,
            &config.password,
        ))
        .await?;

        connection
            .register_callback(DefaultConnectionCallback)
            .await?;

        let channel = connection.open_channel(None).await.unwrap();
        channel.register_callback(DefaultChannelCallback).await?;

        Ok(Self {
            connection,
            channel,
        })
    }
    pub async fn declare_queue(&self, queue_name: &str) -> Result<()> {
        let queue_args = QueueDeclareArguments::new(queue_name);
        self.channel.queue_declare(queue_args).await?;
        Ok(())
    }
    pub async fn send(&self, queue_name: &str, content: Vec<u8>) -> Result<()> {
        let args = BasicPublishArguments::new("", queue_name);

        self.channel
            .basic_publish(BasicProperties::default(), content, args)
            .await?;

        Ok(())
    }
    pub async fn set_recv_callback(
        &self,
        queue_name: &str,
        callback: impl FnMut(Vec<u8>) + Send + 'static,
    ) -> Result<()> {
        let mut args = BasicConsumeArguments::new(queue_name, "");
        args.auto_ack(true);

        self.channel
            .basic_consume(RabbitQueueConsumer::new(callback), args)
            .await?;

        Ok(())
    }

    pub async fn close(self) -> Result<()> {
        self.channel.close().await?;
        self.connection.close().await?;
        Ok(())
    }
}

pub struct RabbitQueueConnnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Default for RabbitQueueConnnectionConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5672,
            username: String::from("user"),
            password: String::from("password"),
        }
    }
}

struct RabbitQueueConsumer {
    callback: Box<dyn FnMut(Vec<u8>) + Send>,
}

impl RabbitQueueConsumer {
    pub fn new(callback: impl FnMut(Vec<u8>) + Send + 'static) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }
}

#[async_trait]
impl AsyncConsumer for RabbitQueueConsumer {
    async fn consume(
        &mut self,
        _: &Channel,
        _: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        (self.callback)(content);
    }
}
