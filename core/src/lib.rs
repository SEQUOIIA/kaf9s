#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod error;
pub mod config;

pub mod topic {
    use rdkafka::consumer::{BaseConsumer, Consumer};
    use rdkafka::config::RDKafkaLogLevel;
    use std::time::Duration;
    use rdkafka::ClientConfig;

    pub fn get_consumer_groups() {
        let consumer : BaseConsumer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", "")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", "")
            .set("sasl.password", "")
            .set("security.protocol", "SASL_SSL")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Unable to create consumer");

        let metadata = consumer
            .fetch_metadata(None, std::time::Duration::from_secs(10)).expect("Failed to fetch metadata");
        let groups = consumer.fetch_group_list(None, std::time::Duration::from_secs(10)).expect("Unable to get consumer groups");

        println!("Groups:");
        for rd_group in groups.groups() {
            println!("  Name: {}", rd_group.name());
            println!("  Members:");

            for member in rd_group.members() {
                println!("    Name: {}", member.id());
                println!("    Client host: {}", member.client_host());
                println!("    Client id: {}", member.client_id());
            }
        }
    }

    pub fn get_topics(conf : ClientConfig) {
        let consumer : BaseConsumer = conf
            .create()
            .expect("Unable to create consumer");

        let metadata = consumer
            .fetch_metadata(None, std::time::Duration::from_secs(10)).expect("Failed to fetch metadata");

        let mut message_count = 0;

        println!("Cluster information:");
        println!("  Broker count: {}", metadata.brokers().len());
        println!("  Topics count: {}", metadata.topics().len());
        println!("  Metadata broker name: {}", metadata.orig_broker_name());
        println!("  Metadata broker id: {}\n", metadata.orig_broker_id());

        // println!("Brokers:");
        // for broker in metadata.brokers() {
        //     println!(
        //         "  Id: {}  Host: {}:{}  ",
        //         broker.id(),
        //         broker.host(),
        //         broker.port()
        //     );
        // }

        println!("\nTopics:");
        for topic in metadata.topics() {
            println!("  Topic: {}  Err: {:?}", topic.name(), topic.error());
            for partition in topic.partitions() {
                println!(
                    "     Partition: {}  Leader: {}  Replicas: {:?}  ISR: {:?}  Err: {:?}",
                    partition.id(),
                    partition.leader(),
                    partition.replicas(),
                    partition.isr(),
                    partition.error()
                );
                    let (low, high) = consumer
                        .fetch_watermarks(topic.name(), partition.id(), Duration::from_secs(1))
                        .unwrap_or((-1, -1));
                    println!(
                        "       Low watermark: {}  High watermark: {} (difference: {})",
                        low,
                        high,
                        high - low
                    );
                    message_count += high - low;
            }
                println!("     Total message count: {}", message_count);
        }
    }


}