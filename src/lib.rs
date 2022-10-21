use reqwest;
use futures::executor;
use serde_json::Value;

pub mod holding_structs;

pub fn get_token(username: String, psw: String)->String{
    let client = reqwest::Client::new();
    let res = executor::block_on(client.post("https://beta.dicecloud.com/api/login")
        .header("Username",username)
        .header("password",psw)
        .send())
        .expect("Dicecloud failed to respond");
    let txt = executor::block_on(res.text()).expect("Dicecloud did not respond properly");
    txt.split(",").next().unwrap().to_string().split(":").next().unwrap().to_string() //don't worry about it, I am sure this works :)
}
/// should have charcter_url=https://beta.dicecloud.com/api/creature/<creatureId>
pub fn get_character(token: String, character_url: String)->Value{
    let client= reqwest::Client::new();
    let res = executor::block_on(client.post(character_url)
        .header("Autorization",token)
        .send())
        .expect("Dicecloud failed to respond");
    let txt=executor::block_on(res.text()).expect("Dicecloud did not respond to request properly");
    serde_json::from_str(&txt).expect("bad format")
}
pub fn get_char_url(caracter_id: String) -> String{
    format!("https://beta.dicecloud.com/api/creature/{}",caracter_id)
}