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
        .element(elements::Image::from_path("./images/dicecloud_favicon.jpg")
            .expect("failed to load image")
            .with_scale(genpdf::Scale::new(2,2))
            .with_position(genpdf::Position::new(13,1))
            )
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
        .element(elements::Break::new(0.5))
        .element(elements::Paragraph::new("SAVING THROWS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        );
    let mut skill_element = elements::LinearLayout::vertical();
    
    skills.sort();
    let mut passive_bonus: i64 = 10;
    for skill in skills{
        skill_element=skill_element.element(element_from_skill(&skill,&symbol));
        if skill.get_name()=="Perception"{
            passive_bonus+=skill.get_mod()+character.passive_bonus;
        }
    }
    skill_element=skill_element.element(elements::Break::new(0.5))
        .element(
            elements::Paragraph::new("SKILLS")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(8))
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
        .element(elements::Paragraph::new(bns_translator(character.prof_bonus))
            .aligned(Alignment::Center))
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
            .element(elements::Break::new(1.25))
            .element(element_from_score(ability_scores.get("Dexterity").unwrap()).framed())
            .element(elements::Break::new(1.25))
            .element(element_from_score(ability_scores.get("Constitution").unwrap()).framed())
            .element(elements::Break::new(1.25))
            .element(element_from_score(ability_scores.get("Intelligence").unwrap()).framed())
            .element(elements::Break::new(1.25))
            .element(element_from_score(ability_scores.get("Wisdom").unwrap()).framed())
            .element(elements::Break::new(1.25))
            .element(element_from_score(ability_scores.get("Charisma").unwrap()).framed())
            .padded(1)
        )
        .element(elements::LinearLayout::vertical()
            .element(inspiration)
            .element(elements::Break::new(0.75))
            .element(prof_bonus)
            .element(elements::Break::new(0.75))
            .element(saving_throws.padded(3).framed())
            .element(elements::Break::new(0.75))
            .element(skill_element.padded(3).framed())
            .padded(1)
        )
        .push().expect("failed to add row");
    let other_profs = (character.other_profs.0.join(", "),
        character.other_profs.1.join(", "),character.other_profs.2.join(", "),
        character.other_profs.3.join(", "));
    
    let mut passive_perception = elements::TableLayout::new(vec![2,9]);
    passive_perception.set_cell_decorator(elements::FrameCellDecorator::new(true,true,false));
    passive_perception
        .row()
        .element(elements::Paragraph::new(passive_bonus.to_string())
            .aligned(Alignment::Center))
        .element(elements::Paragraph::new("PASSIVE PERCEPTION")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(7))
            .padded(2)
        )
        .push().expect("Failed to add rows");
    let mut middle_column = elements::LinearLayout::vertical();
    let mut top_middle= elements::TableLayout::new(vec![1,1,1]);
    top_middle.set_cell_decorator(elements::FrameCellDecorator::new(false,false,false));
    top_middle.row()
        .element(
            elements::LinearLayout::vertical()
                .element(
                    elements::Paragraph::new(character.ac.to_string())
                        .aligned(Alignment::Center)
                )
                .element(
                    elements::Paragraph::new("ARMOR CLASS")
                        .aligned(Alignment::Center)
                        .styled(style::Style::new().bold().with_font_size(7))
                )
                .padded(2)
                .framed()
                .padded(1)
        )
        .element(
            elements::LinearLayout::vertical()
                .element(elements::Break::new(0.5))
                .element(
                    elements::Paragraph::new(bns_translator(character.initiative))
                        .aligned(Alignment::Center)
                )
                .element(
                    elements::Paragraph::new("INITIATIVE")
                        .aligned(Alignment::Center)
                        .styled(style::Style::new().bold().with_font_size(7))
                )
                .element(elements::Break::new(0.4))
                .padded(1)
                .framed()
                .padded(1)
        )
        .element(
            elements::LinearLayout::vertical()
                .element(elements::Break::new(0.5))
                .element(
                    elements::Paragraph::new(character.speed.to_string())
                        .aligned(Alignment::Center)
                )
                .element(
                    elements::Paragraph::new("SPEED")
                        .aligned(Alignment::Center)
                        .styled(style::Style::new().bold().with_font_size(7))
                )
                .element(elements::Break::new(0.4))
                .padded(1)
                .framed()
                .padded(1)
            )
        .push().expect("Failed to add row");
    middle_column=middle_column.element(top_middle);
    let hit_point_detail=elements::LinearLayout::vertical()
            .element(elements::Paragraph::new(format!("Hit Point Maximum: {}",character.hit_points))
                .styled(style::Style::new().with_font_size(7))
                .padded(2)
                .framed())
            .element(elements::Break::new(2.25))
            .element(elements::Paragraph::new("CURRENT HIT POINTS")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)));
    middle_column=middle_column.element(elements::Break::new(0.25))
            .element(hit_point_detail.framed().padded(2))
            .element(elements::Break::new(0.1))
            .element(elements::LinearLayout::vertical()
                .element(elements::Break::new(2.25))
                .element(elements::Paragraph::new("TEMPORARY HIT POINTS")
                    .aligned(Alignment::Center)
                    .styled(style::Style::new().bold().with_font_size(7))
                )
                .framed()
                .padded(2)
            )
            .element(elements::Break::new(0.25));
    let mut mid_tbl = elements::TableLayout::new(vec![1,1]);
    mid_tbl.set_cell_decorator(elements::FrameCellDecorator::new(false,false,false));
    let hd_str: String = character.hit_dice.iter().map(|die| die.to_string()).collect::<Vec<_>>().join(", ");
    mid_tbl.row()
        .element(elements::LinearLayout::vertical()
            .element(elements::Paragraph::new(format!("Total: {}",hd_str))
                .styled(style::Style::new().with_font_size(7))
            )
            .element(elements::Break::new(1))
            .element(elements::Paragraph::new("HIT DICE")
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
            )
            .padded(1)
            .framed()
            .padded(1)
        )
        .element(elements::LinearLayout::vertical()
            .element(elements::Paragraph::default()
                .styled_string("SUCCESSES ",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .aligned(Alignment::Right)
            )
            .element(elements::Paragraph::default()
                .styled_string("FAILURES ",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .aligned(Alignment::Right)
            )
            .element(elements::Paragraph::new("DEATH SAVES")
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7)))
            .padded(1)
            .framed()
            .padded(1)
        )
        .push().expect("Invalid Row");
    middle_column=middle_column.element(mid_tbl);
    main_sheet
        .row()
        .element(elements::LinearLayout::vertical()
            .element(left_bar)
            .element(elements::Break::new(0.5))
            .element(passive_perception.padded(1))
            .element(elements::Break::new(1.0))
            .element(elements::LinearLayout::vertical()
                .element(elements::Paragraph::default()
                    .styled_string("Armor: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.0,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(elements::Paragraph::default()
                    .styled_string("Weapons: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.1,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(elements::Paragraph::default()
                    .styled_string("Languages: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.2,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(elements::Paragraph::default()
                    .styled_string("Tools: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.3,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(elements::Paragraph::new("OTHER PROFICIENCIES & LANGUAGES")
                    .aligned(Alignment::Center)
                    .styled(style::Style::new().bold().with_font_size(7)))
                .framed()
                .padded(1)
            )
        )
        .element(elements::LinearLayout::vertical()
            .element(elements::Break::new(0.25))
            .element(middle_column)
        )
        .element(elements::Paragraph::new(""))
        .push().expect("failed to add row");
    doc.push(main_sheet);
    println!("Rendering pdf...(this may take a moment)");
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
        .string(format!(" {}  {}",bns,skill.get_name()))
        .styled(style::Style::new().with_font_size(8))
}
fn proficiency_translator(prof: &Proficiency)->String{
    match prof{
        Proficiency::None => String::from("⭘"),
        Proficiency::Half => String::from("◐"),
        Proficiency::Profficient => String::from("⦿"),
        Proficiency::Expert => String::from("❂")
    }
}