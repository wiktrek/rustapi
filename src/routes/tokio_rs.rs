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
