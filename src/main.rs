use std::env;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};
use dicecloud_sheet_printer::{generate_pdf,get_token,get_character,get_char_url,bns_translator,holding_structs::*};
use serde_json::Value;
use std::collections::HashMap;
use tokio;
use std::{io,process};

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
    if token.len() ==0{
        println!("Failed to login! Try accessing with no token?(y/n)");
        let mut ans= String::new();
        stdin.read_line(&mut ans).expect("failed to get answer");
        if !ans.to_lowercase().contains("y"){
            println!("Exiting to terminal");
            process::exit(0);
        }
        println!("continuing");
    } else {
        println!("Successfully logged in");
    }
    println!("Enter Character ID:");
    let mut char_id = String::new();
    stdin.read_line(&mut char_id).expect("Failed to get character id");
    println!("getting character asycronously");
    let char_json = get_character(token,get_char_url(char_id));
    println!("Setting up heading");
    let mut header = elements::TableLayout::new(vec![2,15]);
    header
        .row()
        .element(elements::Paragraph::new(""))
        .element(elements::Paragraph::new("DUNGEONS AND DRAGONS")
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(11)))
        .push().expect("Invalid row");
    doc.push(header);
    let mut detail = elements::TableLayout::new(vec![1,2]);
    println!("Processing Character...");
    let character = Character::new(char_json.await);
    println!("Setting up document...");
    detail.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let detail_left = elements::LinearLayout::vertical()
        .element(elements::Break::new(1.0))
        .element(elements::Paragraph::new(&character.char_name)
            .aligned(Alignment::Center)
            .framed());
    let mut detail_right = elements::TableLayout::new(vec![1,1,1]);
    detail_right.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let mut class_str = String::new();
    for class in &character.classes{
        class_str+=&format!("{} {}",class.get_name(),class.get_level());
    }
    detail_right
        .row()
        .element(
            elements::Paragraph::new(class_str).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            elements::Paragraph::new(character.background.get_name()).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            elements::Paragraph::new("").styled(style::Style::new().with_line_spacing(0.5))
        )
        .push().expect("Invalid row");
    detail_right
        .row()
        .element(
            elements::Paragraph::new("Class").styled(style::Style::new().with_font_size(7))
        )
        .element(
            elements::Paragraph::new("Background").styled(style::Style::new().with_font_size(7))
        )
        .element(
            elements::Paragraph::new("Player Name").styled(style::Style::new().with_font_size(7))
        )
        .push().expect("Invalid row");
    detail_right
        .row()
        .element(
            elements::Paragraph::new(&character.race).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            elements::Paragraph::new(&character.alignment).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            elements::Paragraph::new("").styled(style::Style::new().with_line_spacing(0.5))
        )
        .push().expect("Invalid row");
    detail_right
        .row()
        .element(
            elements::Paragraph::new("Race")
                .styled(style::Style::new().with_font_size(7))
        )
        .element(
            elements::Paragraph::new("Alignment")
                .styled(style::Style::new().with_font_size(7))
        )
        .element(
            elements::Paragraph::new("Experience Points")
                .styled(style::Style::new().with_font_size(7))
        )
        .push().expect("Invalid row");
    detail
        .row()
        .element(
            detail_left.padded(1)
        )
        .element(
            detail_right
                .padded(1).framed()
        )
        .push()
        .expect("Invalid Table Row");
    doc.push(detail);
    let mut main_sheet = elements::TableLayout::new(vec![1,1,1]);
    let mut ability_scores: HashMap<&str,&AbilityScore> = HashMap::new();
    for score in &character.ability_scores{
        ability_scores.insert(score.get_name(),score);
    }
    main_sheet.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let stren = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::new("STRENGTH")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator((*ability_scores.get("Strength")
                .expect("No strength score! Means your char is not 5e")).get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new((*ability_scores.get("Strength").expect("")).get_score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        );

    let dex = elements::LinearLayout::vertical()
        .element(
          elements::Paragraph::new("DEXTERITY")
          .aligned(Alignment::Center)
          .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator((*ability_scores.get("Dexterity")
                .expect("No dexterity score! Means your char is not 5e")).get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new((*ability_scores.get("Dexterity").expect("")).get_score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        );
    let con = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::new("CONSTITUTION")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator((*ability_scores.get("Constitution")
                .expect("No constitution score! Means your char is not 5e")).get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new((*ability_scores.get("Constitution").expect("")).get_score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        );
    let int = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::new("INTELLIGENCE")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator((*ability_scores.get("Intelligence")
                .expect("No intelligence score! Means your char is not 5e")).get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new((*ability_scores.get("Intelligence").expect("")).get_score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        );
    let wis = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::new("WISDOM")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator((*ability_scores.get("Wisdom")
                .expect("No wisdom score! Means your char is not 5e")).get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new(format!("{}",(*ability_scores.get("Wisdom").expect("")).get_score()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        );
    let chr = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::new("CHARISMA")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator((*ability_scores.get("Charisma")
                .expect("No charisma score! Means your char is not 5e")).get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new(format!("{}",(*ability_scores.get("Charisma").expect("")).get_score()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        );
    let mut left_bar = elements::TableLayout::new(vec![1,2]);
    left_bar.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    left_bar
        .row()
        .element(elements::LinearLayout::vertical()
            .element(stren.framed())
            .element(elements::Break::new(1.0))
            .element(dex.framed())
            .element(elements::Break::new(1.0))
            .element(con.framed())
            .element(elements::Break::new(1.0))
            .element(int.framed())
            .element(elements::Break::new(1.0))
            .element(wis.framed())
            .element(elements::Break::new(1.0))
            .element(chr.framed())
        )
        .element(elements::Paragraph::new(""))
        .push().expect("failed to add row");
    main_sheet
        .row()
        .element(left_bar)
        .element(elements::Paragraph::new(""))
        .element(elements::Paragraph::new(""))
        .push().expect("failed to add row");
    doc.push(main_sheet);
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
