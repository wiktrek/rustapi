use rocket::*;
use mongo;
#[get("/db/users")]
pub fn get_users() -> Json {
    let users= mongo.fetch_users().await.map_err(|e| reject::custom(e))?;
    Ok(json(&users))
}