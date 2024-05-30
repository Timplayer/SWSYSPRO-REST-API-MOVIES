use tokio_postgres::{NoTls, Client, Error,Config};
use std::{env, time::Duration};

const DEFAULT_PORT : i32 = 5432;

pub async fn init() -> Result<Client, Error> {

    println!("DB Connection Init!");

    let (client, connection) = Config::new()
    .user(&env::var("DB_USER").expect("DB_USER must be set"))
    .password(&env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"))
    .host(&env::var("DB_HOST").expect("DB_HOST must be set"))
    .dbname(&env::var("DB_NAME").expect("DB_NAME must be set"))
    .port(env::var("DB_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string()).parse().expect("DB_PORT must be a valid integer"))
    .connect_timeout(Duration::new(3, 0))
    .connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let create_table_query = 
    "
        create table if not exists movies (
            id serial primary key,
            name text not null
        );
    ";

    client.execute(create_table_query, &[]).await.expect("Failed to create table");

    Ok(client)
}
