use rand::Rng;
#[get("/random/<amount>/<min>/<max>")]
pub fn random(amount: i32, min: i32, max: i32) -> String {
    let mut nums: Vec<i32> = vec![];
    let mut i = 0;
    while i < amount {
        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen_range(min..max);
        nums.push(num);
        i += 1;
    }
    format!("{:?}", nums)
        .replace(" ", "")
        .replace("[", "")
        .replace("]", "")
}
