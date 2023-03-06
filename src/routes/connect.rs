use mongodb::{bson::doc, options::ClientOptions, Client};
use exitfailure::ExitFailure;
use std::env;
#[tokio::main]
pub async fn mongo() -> Result<(), ExitFailure> {
    // Parse your connection string into an options struct
    let db_url =  env::var("DB").expect("$USER is not set");
    let mut client_options =
        ClientOptions::parse(db_url)
            .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("rust")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}