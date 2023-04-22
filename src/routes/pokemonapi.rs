use reqwest::*;
#[get("/pokemon/<pokemon>")]
pub fn pokemon(pokemon: String) -> String {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);

    return url.to_string();
}
