use tokio::time::{sleep, Duration};
#[get("/tokio")]
pub async fn tokio_rs() -> String {
    let a = tokio::spawn(test());
    return a.await.unwrap();
}
async fn test() -> String {
    println!("Testing ");
    sleep(Duration::from_secs(1)).await;
    println!("waited 1s");
    // println!("test");
    return "waited 1s".to_string();
}
#[get("/tokio/wait/<time>")]
pub async fn tokio2(time: u64) -> String {
    let a = tokio::spawn(wait(time));
    return a.await.unwrap();
}
async fn wait(time: u64) -> String {
    println!("waiting {}s", time);
    sleep(Duration::from_secs(time)).await;
    println!("waited {}s", time);
    // println!("test");
    return "waited 1s".to_string();
}
