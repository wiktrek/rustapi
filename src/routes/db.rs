use rocket::*;
#[get("/db/<option>/<data>")]
pub fn db<'a>(option: &str, data: &str) -> &'a str {
match option {
    "save"=> {
        save();
        return "save"
    },
    "delete"=> {
        delete();
   return "delete"
    },
_ => return "choose one option save or write"
}
return "db"
}
fn save() -> String {
return "Hello from save".to_string();
}
fn delete() {

}