use rocket::*;
#[path = "./connect.rs"] mod connect;
use connect::mongo;
#[get("/db/<option>")]
pub fn db(option: &str) -> &str {
    mongo();
    match option {
        "save"=> return save(),
        "delete"=> return delete(),
    _ => return "option not found" 
    }
}
fn save() -> &str {
    println!("save");
    return "save"
}

fn delete() -> &str {
println!("delete");
return "save"
}