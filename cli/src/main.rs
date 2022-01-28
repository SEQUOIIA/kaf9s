fn main() {
    let mut cm = kaf9s_core::config::ConfigManager::load();
    let current_context = cm.get_current_context();

    let kafka_conf = cm.context_to_kafka_config(&current_context.name).unwrap();

    println!("Connecting to cluster {} with user {}", current_context.cluster, current_context.user);

    //kaf9s_core::topic::get_consumer_groups();
    kaf9s_core::topic::get_topics(kafka_conf);
}
