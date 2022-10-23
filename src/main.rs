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
    let token =get_token(username, psw).await;
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
    let mut header = elements::TableLayout::new(vec![1,2]);
    let character = Character::new(char_json.await);
    header.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let header_left = elements::LinearLayout::vertical()
        .element(elements::Break::new(0.5))
        .element(elements::Paragraph::new(&character.char_name)
            .aligned(Alignment::Center)
            .framed());
    let mut header_right = elements::TableLayout::new(vec![1,1,1]);
    header_right.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let mut class_str = String::new();
    for class in &character.classes{
        class_str+=&format!("{} {}",class.get_name(),class.get_level());
    }
    header_right
        .row()
        .element(
            elements::Paragraph::new(class_str)
        )
        .element(
            elements::Paragraph::new(character.background.get_name())
        )
        .element(
            elements::Paragraph::new("")
        )
        .push().expect("Invalid row");
    header_right
        .row()
        .element(
            elements::Paragraph::new("Class").styled(style::Style::new().with_font_size(10))
        )
        .element(
            elements::Paragraph::new("Background").styled(style::Style::new().with_font_size(10))
        )
        .element(
            elements::Paragraph::new("").styled(style::Style::new().with_font_size(10))
        )
        .push().expect("Invalid row");
    header_right
        .row()
        .element(
            elements::Paragraph::new(&character.race)
        )
        .element(
            elements::Paragraph::new(&character.alignment)
        )
        .element(
            elements::Paragraph::new(format!("{}",character.xp))
        )
        .push().expect("Invalid row");
    header_right
        .row()
        .element(
            elements::Paragraph::new("Race")
                .styled(style::Style::new().with_font_size(10))
        )
        .element(
            elements::Paragraph::new("Alignment")
                .styled(style::Style::new().with_font_size(10))
        )
        .element(
            elements::Paragraph::new("Experience Points")
                .styled(style::Style::new().with_font_size(10))
        )
        .push().expect("Invalid row");
    header
        .row()
        .element(
            header_left.padded(1)
        )
        .element(
            header_right.padded(1).framed()
        )
        .push()
        .expect("Invalid Table Row");
    doc.push(header);
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
