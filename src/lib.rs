use std::collections::HashMap;

use reqwest;
use futures::{stream, StreamExt};
use serde_json::Value;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};

pub mod holding_structs;

pub async fn get_token(username: String, psw: String)->Result<String,reqwest::Error>{
    let client = reqwest::Client::new();
    let mut map =HashMap::new();
    map.insert("username",username);
    map.insert("password",psw);
    let res = client.post("https://beta.dicecloud.com/api/login")
        .json(&map)
        .send()
        .await?;
    let txt =res.text().await?;
    if txt.contains("error"){
        return Ok("".to_string()); //if login fails, just pretend everything is ok
    }
    token = sedre_json::from_str(&txt)?["token"].as_str();
    Ok(token.to_string())
}
/// should have charcter_url=https://beta.dicecloud.com/api/creature/<creatureId>
pub async fn get_character(token: String, character_url: String)->Value{
    let client= reqwest::Client::new();
    let res = client.post(character_url)
        .header("Authorization",format!("Bearer {}",token))
        .send()
        .await
        .expect("Dicecloud failed to respond");
    let txt=res.text().await.expect("Dicecloud did not respond to request properly");
    if txt.contains("'error': 'Permission denied'"){
        panic!("Permision Denied!"); //this is definitely a panic case.
    }
    serde_json::from_str(&txt).expect("bad format")
}
pub fn get_char_url(caracter_id: String) -> String{
    format!("https://beta.dicecloud.com/api/creature/{}",caracter_id)
}
pub fn generate_pdf()->genpdf::Document{
    let font = fonts::from_files("./Roboto","Roboto",None).expect("Failed to load font");
    let mut doc = genpdf::Document::new(font);
    doc.set_title("Character Sheet");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
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