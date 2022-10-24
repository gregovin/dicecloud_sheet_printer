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
    let symbol_font = doc.add_font_family(fonts::from_files("./fonts/Noto_Sans_Symbols_2","NotoSansSymbols2",None)
        .expect("Failed to load symbol font"));
    let symbol = style::Style::from(symbol_font);
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
    let mut xp = String::new();
    if character.xp>0{
        let mut want_xp = String::new();
        println!("Do you want to include your xp in the sheet?(y/n) Keep in mind this will make it harder to pencil it in later");
        stdin.read_line(&mut want_xp).expect("Failed to read answer");
        if want_xp.to_lowercase().contains("y"){
            xp+=&character.xp.to_string();
        }
    }
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
            elements::Paragraph::new(xp).styled(style::Style::new().with_line_spacing(0.5))
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
    doc.push(elements::Break::new(1.0));
    let mut main_sheet = elements::TableLayout::new(vec![1,1,1]);
    let mut ability_scores: HashMap<&str,&AbilityScore> = HashMap::new();
    for score in &character.ability_scores{
        ability_scores.insert(score.get_name(),score);
    }
    main_sheet.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));    
    let mut left_bar = elements::TableLayout::new(vec![1,2]);
    left_bar.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let mut skills = character.skills;
    let mut saves: HashMap<&str,&Skill> = HashMap::new();
    for save in &character.saving_throws{
        saves.insert(save.get_name(),save);
    }
    let saving_throws = elements::LinearLayout::vertical()
        .element(element_from_skill(saves.get("Strength Save").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Dexterity Save").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Constitution Save").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Intelligence Save").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Wisdom Save").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Charisma Save").unwrap(),&symbol))
        .element(elements::Paragraph::new("SAVING THROWS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        );
    let mut skill_element = elements::LinearLayout::vertical();
    skills.sort();
    for skill in skills{
        skill_element=skill_element.element(element_from_skill(&skill,&symbol));
    }
    skill_element=skill_element.element(
        elements::Paragraph::new("SKILLS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))

    );
    let mut inspiration = elements::TableLayout::new(vec![2,9]);
    inspiration.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    inspiration.row()
        .element(elements::Paragraph::new(""))
        .element(elements::Paragraph::new("INSPIRATION")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
            .padded(2)
        ).push().expect("Failed to add row");
    let mut prof_bonus = elements::TableLayout::new(vec![2,9]);
    prof_bonus.set_cell_decorator(elements::FrameCellDecorator::new(true,true,false));
    prof_bonus.row()
        .element(elements::Paragraph::new(character.prof_bonus.to_string()))
        .element(elements::Paragraph::new("PROFICIENCY BONUS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
            .padded(2)
    ).push().expect("Failed to add row");
    left_bar
        .row()
        .element(elements::LinearLayout::vertical()
            .element(elements::Break::new(0.5))
            .element(element_from_score(ability_scores.get("Strength").unwrap()).framed())
            .element(elements::Break::new(1.0))
            .element(element_from_score(ability_scores.get("Dexterity").unwrap()).framed())
            .element(elements::Break::new(1.0))
            .element(element_from_score(ability_scores.get("Constitution").unwrap()).framed())
            .element(elements::Break::new(1.0))
            .element(element_from_score(ability_scores.get("Intelligence").unwrap()).framed())
            .element(elements::Break::new(1.0))
            .element(element_from_score(ability_scores.get("Wisdom").unwrap()).framed())
            .element(elements::Break::new(1.0))
            .element(element_from_score(ability_scores.get("Charisma").unwrap()).framed())
        )
        .element(elements::LinearLayout::vertical()
            .element(inspiration)
            .element(elements::Break::new(0.5))
            .element(prof_bonus)
            .element(elements::Break::new(0.5))
            .element(saving_throws.padded(2).framed())
            .element(elements::Break::new(0.5))
            .element(skill_element.padded(2).framed())
            .padded(1)
        )
        .push().expect("failed to add row");
    main_sheet
        .row()
        .element(left_bar)
        .element(elements::Paragraph::new(""))
        .element(elements::Paragraph::new(""))
        .push().expect("failed to add row");
    doc.push(main_sheet);
    println!("Rendering pdf");
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
fn element_from_score(score: &AbilityScore)->elements::LinearLayout{
    elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::new(score.get_name().to_uppercase())
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            elements::Paragraph::new(bns_translator(score.get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            elements::Paragraph::new(score.get_score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        )
}
fn element_from_skill(skill: &Skill, symb_fnt: &style::Style)->elements::StyledElement<elements::Paragraph>{
    let mut bns = bns_translator(skill.get_mod());
    if bns.bytes().count() == 2 {
        bns=String::from(" ")+&bns;
    }
    elements::Paragraph::default()
        .styled_string(format!("{}",proficiency_translator(skill.get_prof())),symb_fnt.clone())
        .string(format!(" {} {}",bns,skill.get_name()))
        .styled(style::Style::new().with_font_size(7))
}
fn proficiency_translator(prof: &Proficiency)->String{
    match prof{
        Proficiency::None => String::from("⭘"),
        Proficiency::Half => String::from("◐"),
        Proficiency::Profficient => String::from("⦿"),
        Proficiency::Expert => String::from("❂")
    }
}