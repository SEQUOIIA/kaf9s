fn main() {
    println!("Hello, world!");

    let mut cm = kaf9s_core::config::ConfigManager::load();
    cm.save_user_secrets();
    std::process::exit(1);
    cm.reconcile();
    let current_context = cm.get_current_context();

    println!("Contexts:");
    for (key, context) in &cm.contexts {
        let cluster = cm.clusters.get(&context.cluster).expect("Cluster specified in Context doesn't exist");
        let user = cm.users.get(&context.user).expect("User specified in Context doesn't exist");
        if context.name.eq(&current_context.name) {
            println!(" *{}", &context.name);
            for (k, v) in &cluster.data {
                println!("    {}: {}", k, v);
            }
            for (k, v) in &user.data {
                println!("    {}: {}", k, v);
            }
        } else {
            println!("  {}", &context.name);
            for (k, v) in &cluster.data {
                println!("    {}: {}", k, v);
            }
            for (k, v) in &user.data {
                println!("    {}: {}", k, v);
            }
        }
    }



    //kaf9s_core::config::set_secret_in_keyring("dev01-cluster-eu", "niceuuuu").expect("Unable to store secret");
    //let secret = kaf9s_core::config::get_secret_from_keyring("dev01-cluster-eu").expect(&format!("Unable to retrieve secret for {}", "dev01-cluster-eu"));

    //kaf9s_core::topic::get_consumer_groups();
    //kaf9s_core::topic::get_topics();
}
