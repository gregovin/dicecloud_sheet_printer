use std::env;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};
use dicecloud_sheet_printer::{generate_pdf,get_token,get_character,get_char_url,holding_structs::*};
use serde_json::Value;
use tokio;
use std::io;

#[tokio::main]
async fn main() {
    let mut doc = generate_pdf();
    let mut username = String::new();
    println!("Username:");
    let stdin= io::stdin();
    
    stdin.read_line(&mut username).expect("could not read username");
    let mut psw = String::new();
    println!("Password:");
    stdin.read_line(&mut psw).expect("Fallied to get password");
    let token =get_token(username, psw).await.expect("oops!");
    println!("Success! Got token.");
    println!("Enter Character ID:");
    let mut char_id = String::new();
    stdin.read_line(&mut char_id).expect("Failed to get character id");
    println!("getting character");
    let char_json = get_character(token,get_char_url(char_id));
    println!("success!");
    doc.push(
        elements::Paragraph::new("Dungeons and Dragons")
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    let mut table = elements::TableLayout::new(vec![1,2]);
    if let Value::Object(mp)=char_json{

    }
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    table
        .row()
        .element(
            elements::LinearLayout::vertical()
                .element(elements::Break::new(0.5))
                .element(elements::Paragraph::new("Charname").aligned(Alignment::Center))
                .padded(1)
        )
        .element(
            elements::LinearLayout::vertical()
                .element(elements::Paragraph::new("stuff"))
                .element(elements::Break::new(0.25))
                .element(elements::Paragraph::new("Things"))
                .padded(1)
        )
        .push()
        .expect("Invalid Table Row");
    doc.push(table);
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
