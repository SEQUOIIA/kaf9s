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
    use std::io::BufRead;
    use rdkafka::consumer::{BaseConsumer, Consumer};
    use rdkafka::config::RDKafkaLogLevel;
    use std::time::Duration;
    use byteorder::ReadBytesExt;
    use rdkafka::{ClientConfig, TopicPartitionList};

    pub fn get_consumer_groups(conf : ClientConfig) {
        let consumer : BaseConsumer = conf
            .create()
            .expect("Unable to create consumer");

        let groups = consumer.fetch_group_list(None, std::time::Duration::from_secs(10)).expect("Unable to get consumer groups");

        println!("Groups:");
        for rd_group in groups.groups() {
            println!("  Name: {}", rd_group.name());
            println!("  State: {}", rd_group.state());
            println!("  Protocol: {}", rd_group.protocol());
            println!("  Protocol type: {}", rd_group.protocol_type());
            println!("  Members:");

            for member in rd_group.members() {
                println!("    Name: {}", member.id());
                println!("    Client host: {}", member.client_host());
                println!("    Client id: {}", member.client_id());
                println!("    Topics:");
                if member.metadata().is_some() && rd_group.protocol_type().eq("consumer") {
                    // Based upon @messense work at https://github.com/fede1024/rust-rdkafka/pull/184/files
                    let payload = member.metadata().unwrap();
                    let mut cursor = std::io::Cursor::new(payload);
                    let _version = cursor.read_i16::<byteorder::BigEndian>().unwrap();
                    let assign_len = cursor.read_i32::<byteorder::BigEndian>().unwrap();
                    for _ in 0..assign_len {
                        let topic = read_str(&mut cursor)
                            .unwrap()
                            .to_string();
                        println!("      {}", topic);
                    }
                }
            }
        }
    }

    pub fn get_topics(conf : ClientConfig) {
        let consumer : BaseConsumer = conf
            .create()
            .expect("Unable to create consumer");

        let metadata = consumer
            .fetch_metadata(None, std::time::Duration::from_secs(10)).expect("Failed to fetch metadata");


        println!("Cluster information:");
        println!("  Broker count: {}", metadata.brokers().len());
        println!("  Topics count: {}", metadata.topics().len());
        println!("  Metadata broker name: {}", metadata.orig_broker_name());
        println!("  Metadata broker id: {}\n", metadata.orig_broker_id());

        println!("\nTopics:");
        for topic in metadata.topics() {
            let mut message_count = 0;
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

    fn read_str<'a>(rdr: &'a mut std::io::Cursor<&[u8]>) -> Result<&'a str, Box<dyn std::error::Error>> {
        let len = (rdr.read_i16::<byteorder::BigEndian>())? as usize;
        let pos = rdr.position() as usize;
        let slice = std::str::from_utf8(&rdr.get_ref()[pos..(pos + len)])?;
        rdr.consume(len);
        Ok(slice)
    }

    pub struct MemberAssignment {
        pub topic : String,
        pub partitions : Vec<i32>
    }
}