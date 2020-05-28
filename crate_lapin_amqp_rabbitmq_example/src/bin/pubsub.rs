use async_std::task;
use futures_executor::{LocalPool, ThreadPool};
use futures_util::stream::StreamExt;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use log::info;
use std::sync::Arc;

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let executor = ThreadPool::new()?;

    LocalPool::new().run_until(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await?;

        info!("CONNECTED");

        let channel_a = conn.create_channel().await?;
        let channel_b = conn.create_channel().await?;

        let queue = channel_a
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Declared queue {:?}", queue);

        let channel_b = Arc::new(channel_b);
        let mut consumer = channel_b
            .clone()
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        executor.spawn_ok(async move {
            info!("will consume");
            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("error in consumer");
                channel_b
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("ack");
                info!("Received: {:?}", delivery);
            }
        });

        let payload = b"Hello world!";

        loop {
            info!("Sending: {:?}", payload);
            let confirm = channel_a
                .basic_publish(
                    "",
                    "hello",
                    BasicPublishOptions::default(),
                    payload.to_vec(),
                    BasicProperties::default(),
                )
                .await?
                .await?;
            assert_eq!(confirm, Confirmation::NotRequested);
            task::sleep(std::time::Duration::from_secs(2)).await;
        }
    })
}
