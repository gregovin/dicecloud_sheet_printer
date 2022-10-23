use std::collections::HashMap;

use reqwest;
use futures::{stream, StreamExt};
use serde_json::Value;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};
use std::process;
pub mod holding_structs;

pub async fn get_token(username: String, psw: String)->String{
    let client = reqwest::Client::new();
    let mut map =HashMap::new();
    map.insert("username",username.trim());
    map.insert("password",psw.trim());
    let res = client.post("https://beta.dicecloud.com/api/login")
        .json(&map)
        .send()
        .await
        .expect("Failed to reach dicecloud");
    let txt =res.text().await.expect("failed to get text");
    if txt.contains("error"){
        return String::new(); //if login fails, just pretend everything is ok
    }
    let js_res: Result<Value,_>= serde_json::from_str(&txt);
    let token =js_res.expect("Failed to parse Json")["token"].as_str().unwrap().to_string();
    token.to_string()
}
/// should have charcter_url=https://beta.dicecloud.com/api/creature/<creatureId>
pub async fn get_character(token: String, character_url: String)->Value{
    let client= reqwest::Client::new();
    let bearer = format!("Bearer {}",token);
    let res = client.get(character_url)
        .header("Authorization",bearer)
        .send()
        .await
        .expect("Dicecloud failed to respond");
    
    let txt=res.text().await.expect("Dicecloud did not respond to request properly");
    if  txt.trim()== ""{
        panic!("invalid response");
    }
    let out: Value =serde_json::from_str(&txt).expect("bad format");
    if out["error"].as_str() != None{
        println!("{}. Exiting program",out["error"].as_str().unwrap());
        process::exit(1);
    }
    out
}
pub fn get_char_url(caracter_id: String) -> String{
    format!("https://beta.dicecloud.com/api/creature/{}",caracter_id.trim())
}
pub fn generate_pdf()->genpdf::Document{
    //define the default font for the document
    let font = fonts::from_files("./Roboto","Roboto",None).expect("Failed to load font");
    let mut doc = genpdf::Document::new(font);
    //set the title and other basic parameter
    doc.set_title("Character Sheet");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    //define the margins and header(may remove header)
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(6);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page>1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Right),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    #[cfg(feature = "hyphenation")]
    {
        use hyphenation::Load;

        doc.set_hyphenator(
            hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS)
                .expect("Failed to load hyphenation data"),
        );
    }
    doc
}