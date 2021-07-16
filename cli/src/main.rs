fn main() {
    println!("Hello, world!");

    let cm = kaf9s_core::config::ConfigManager::load();
    println!("{:?}", cm);

    kaf9s_core::config::set_secret_in_keyring("dev01-cluster-eu", "niceuuuu").expect("Unable to store secret");
    let secret = kaf9s_core::config::get_secret_from_keyring("dev01-cluster-eu").expect(&format!("Unable to retrieve secret for {}", "dev01-cluster-eu"));

    //kaf9s_core::topic::get_consumer_groups();
    //kaf9s_core::topic::get_topics();
}
