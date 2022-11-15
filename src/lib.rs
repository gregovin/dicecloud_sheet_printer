use std::collections::HashMap;
use serde_json::Value;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};
use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use std::io::Cursor;
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
    token
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
pub async fn get_img_from_url(img_url: String)->image::DynamicImage{
    let client = reqwest::Client::new();
    let res_bytes = client.get(img_url)
        .send()
        .await
        .expect("Failed to send request")
        .bytes()
        .await
        .expect("Failed to get image bytes");
    ImageReader::new(Cursor::new(res_bytes)).with_guessed_format().expect("Failed to parse format").decode().expect("failed to decode").resize(540,2000,FilterType::CatmullRom)
}
pub fn get_char_url(caracter_id: String) -> String{
    format!("https://beta.dicecloud.com/api/creature/{}",caracter_id.trim())
}
pub fn generate_pdf()->genpdf::Document{
    //define the default font for the document
    let font = fonts::from_files("./fonts/Roboto","Roboto",None).expect("Failed to load main font");
    let mut doc = genpdf::Document::new(font);
    //set the title and other basic parameter
    doc.set_title("Character Sheet");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    //define the margins and header(may remove header)
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
pub fn bns_translator(b: i64)->String{
    if b>=0{
        return format!("+{}",b);
    }
    format!("{}",b)
}