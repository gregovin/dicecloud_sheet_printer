use std::env;
use std::hash::Hash;
use genpdf::{Element, Alignment};
use genpdf::{elements::{self,Paragraph},fonts, style};
use dicecloud_sheet_printer::{generate_pdf,get_token,get_character,get_char_url,bns_translator,holding_structs::*};
use serde_json::Value;
use std::collections::HashMap;
use tokio;
use std::{io,process,fs};
use textwrap;

#[tokio::main]
async fn main() {
    let mut doc = generate_pdf();
    let symbol_font = doc.add_font_family(fonts::from_files("./fonts/Noto_Sans_Symbols_2","NotoSansSymbols2",None)
        .expect("Failed to load symbol font"));
    let symbol = style::Style::from(symbol_font);
    let race_decoder= serde_json::from_str(&fs::read_to_string("race_decoder.json").expect("Failed to read file race_decoder.json")).expect("Failed to parse race decoder");
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
        .element(Paragraph::new("DUNGEONS AND DRAGONS")
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(11)))
        .push().expect("Invalid row");
    doc.push(header);
    let mut detail = elements::TableLayout::new(vec![1,2]);
    println!("Processing Character...");
    let character = Character::new(char_json.await,race_decoder);
    println!("Setting up document...");
    detail.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let detail_left = elements::LinearLayout::vertical()
        .element(elements::Break::new(1.0))
        .element(Paragraph::new(&character.char_name)
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
            Paragraph::new(class_str).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            Paragraph::new(character.background.get_name()).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            Paragraph::new("").styled(style::Style::new().with_line_spacing(0.5))
        )
        .push().expect("Invalid row");
    detail_right
        .row()
        .element(
            Paragraph::new("Class").styled(style::Style::new().with_font_size(7))
        )
        .element(
            Paragraph::new("Background").styled(style::Style::new().with_font_size(7))
        )
        .element(
            Paragraph::new("Player Name").styled(style::Style::new().with_font_size(7))
        )
        .push().expect("Invalid row");
    detail_right
        .row()
        .element(
            Paragraph::new(character.race).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            Paragraph::new(&character.alignment).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            Paragraph::new(xp).styled(style::Style::new().with_line_spacing(0.5))
        )
        .push().expect("Invalid row");
    detail_right
        .row()
        .element(
            Paragraph::new("Race")
                .styled(style::Style::new().with_font_size(7))
        )
        .element(
            Paragraph::new("Alignment")
                .styled(style::Style::new().with_font_size(7))
        )
        .element(
            Paragraph::new("Experience Points")
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
        .element(Paragraph::new("SAVING THROWS")
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
            Paragraph::new("SKILLS")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(8))
        );
    let mut inspiration = elements::TableLayout::new(vec![2,9]);
    inspiration.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    inspiration.row()
        .element(Paragraph::new(""))
        .element(Paragraph::new("INSPIRATION")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
            .padded(2)
        ).push().expect("Failed to add row");
    let mut prof_bonus = elements::TableLayout::new(vec![2,9]);
    prof_bonus.set_cell_decorator(elements::FrameCellDecorator::new(true,true,false));
    prof_bonus.row()
        .element(Paragraph::new(bns_translator(character.prof_bonus))
            .aligned(Alignment::Center))
        .element(Paragraph::new("PROFICIENCY BONUS")
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
        .element(Paragraph::new(passive_bonus.to_string())
            .aligned(Alignment::Center))
        .element(Paragraph::new("PASSIVE PERCEPTION")
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
                    Paragraph::new(character.ac.to_string())
                        .aligned(Alignment::Center)
                )
                .element(
                    Paragraph::new("ARMOR CLASS")
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
                    Paragraph::new(bns_translator(character.initiative))
                        .aligned(Alignment::Center)
                )
                .element(
                    Paragraph::new("INITIATIVE")
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
                    Paragraph::new(character.speed.to_string())
                        .aligned(Alignment::Center)
                )
                .element(
                    Paragraph::new("SPEED")
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
            .element(Paragraph::new(format!("Hit Point Maximum: {}",character.hit_points))
                .styled(style::Style::new().with_font_size(7))
                .padded(2)
                .framed())
            .element(elements::Break::new(2.25))
            .element(Paragraph::new("CURRENT HIT POINTS")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)));
    middle_column=middle_column.element(elements::Break::new(0.25))
            .element(hit_point_detail.framed().padded(2))
            .element(elements::Break::new(0.1))
            .element(elements::LinearLayout::vertical()
                .element(elements::Break::new(2.25))
                .element(Paragraph::new("TEMPORARY HIT POINTS")
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
            .element(Paragraph::new(format!("Total: {}",hd_str))
                .styled(style::Style::new().with_font_size(7))
            )
            .element(elements::Break::new(1))
            .element(Paragraph::new("HIT DICE")
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
            )
            .padded(1)
            .framed()
            .padded(1)
        )
        .element(elements::LinearLayout::vertical()
            .element(Paragraph::default()
                .styled_string("SUCCESSES ",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘ ",symbol.clone().with_font_size(7))
                .aligned(Alignment::Right)
            )
            .element(Paragraph::default()
                .styled_string("FAILURES ",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘",symbol.clone().with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("⭘ ",symbol.clone().with_font_size(7))
                .aligned(Alignment::Right)
            )
            .element(Paragraph::new("DEATH SAVES")
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7)))
            .framed()
            .padded(1)
        )
        .push().expect("Invalid Row");
    middle_column=middle_column.element(mid_tbl);
    let atks = character.attacks;
    let mut atk_dict: HashMap<String,Attack> = HashMap::new();
    for atk in atks{
        atk_dict.insert(atk.get_name().clone(),atk);
    }
    let mut to_display: Vec<Attack>=vec![];
    //needs to be rethought
    if atk_dict.len()>24{
        println!("You have more attacks than you have space for! Select up to 23 attacks");
        println!("Type \"list\" to list all attacks, \"selection\" to show selection, \"instructions\" to print this again, \"remove <name>\" to remove an attack by name, \"add <name>\" to add an attack by name, or \"done\" to finish selection");
        let mut done: bool=false;
        while !done{
            let mut current_inst: String=String::new();
            stdin.read_line(&mut current_inst).expect("failed to read line");
            if &current_inst.trim().to_lowercase()=="list"{
                println!("{}",atk_dict.iter().map(|atk| atk.1.get_name().clone()).collect::<Vec<_>>().join(", "));
            } else if &current_inst.trim().to_lowercase()=="selection"{
                println!("{} ({}/23)",to_display.iter().map(|atk| atk.get_name().clone()).collect::<Vec<_>>().join(", "),to_display.len());
            } else if &current_inst.trim().to_lowercase()=="instructions"{
                println!("Type \"list\" to list all attacks, \"selection\" to show selection, \"instructions\" to print this again, \"remove <name>\" to remove an attack by name, \"add <name>\" to add an attack by name, or \"done\" to finish selection");
            } else if current_inst.trim().to_lowercase().contains("remove"){
                let atk_name = current_inst.replace("remove ","").trim().to_string();
                match atk_dict.get(&atk_name) {
                    Some(atk)=>{to_display.retain(|x| x !=atk);println!("Removed attack");},
                    None=>println!("Attack does not exist"),
                };
            } else if current_inst.trim().to_lowercase().contains("add"){
                let atk_name = current_inst.replace("add ","").trim().to_string();
                if atk_dict.contains_key(&atk_name){
                    if to_display.len() < 23{
                        to_display.push(atk_dict.get(&atk_name).unwrap().clone());
                        println!("Added attack ({}/23)",to_display.len());
                    } else {
                        println!("You allready have too many attacks!");
                    }
                } else {
                    println!("Specified attack does not exist");
                }
            } else if &current_inst.trim().to_lowercase()=="done"{
                done = true;
            } else{
                println!("Invalid instruction. Type \"instructions\" to see all options");
            }
        }
    } else {
        for atk in atk_dict.iter(){
            to_display.push(atk.1.clone());
        }
    }
    let num_atks = to_display.len();
    to_display.sort();
    let mut attack_display=elements::TableLayout::new(vec![2,1,2]);
    attack_display.row()
        .element(Paragraph::new("NAME").styled(style::Style::new().with_font_size(7)))
        .element(Paragraph::new("ATK BONUS").styled(style::Style::new().with_font_size(7)))
        .element(Paragraph::new("DAMAGE/TYPE").styled(style::Style::new().with_font_size(7)))
        .push().expect("failed to add row");
    for atk in to_display{
        let nme = atk.get_name().to_string();
        let mut trunk = nme.replace(" (Two-Handed)","(2H)");
        trunk.truncate(13);
        attack_display.row()
            .element(Paragraph::new(trunk)
                .styled(style::Style::new().bold().with_font_size(10))
                )
            .element(
                Paragraph::new(atk.get_bonus_as_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(10))
            )
            .element(
                Paragraph::new(atk.get_damage())
                .styled(style::Style::new().with_font_size(10))
            )
            .push().expect("failed to add row");
    }
    if num_atks<23{
        let needed=23-num_atks;
        for _ in 0..needed{
            attack_display.row()
                .element(Paragraph::new("")
                    .styled(style::Style::new().bold().with_font_size(10))
                )
                .element(
                    Paragraph::new("")
                    .aligned(Alignment::Center)
                    .styled(style::Style::new().with_font_size(10))
                )
                .element(
                    Paragraph::new("")
                    .styled(style::Style::new().with_font_size(10))
                )
            .push().expect("failed to add row");
        }
    }
    middle_column=middle_column.element(attack_display.padded(1).framed().padded(1));
    let traits = character.traits;
    let personality = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.0,28,3))
        .element(Paragraph::new("PERSONALITY TRAITS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(10).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let ideal = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.1,28,2))
        .element(Paragraph::new("IDEALS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(10).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let bond = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.2,28,2))
        .element(Paragraph::new("BONDS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(10).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let flaw = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.3,28,2))
        .element(Paragraph::new("FLAWS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(10).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let traits_elemt= elements::LinearLayout::vertical()
        .element(personality)
        .element(elements::Break::new(0.5))
        .element(ideal)
        .element(elements::Break::new(0.5))
        .element(bond)
        .element(elements::Break::new(0.5))
        .element(flaw)
        .element(elements::Break::new(0.5));
    main_sheet
        .row()
        .element(elements::LinearLayout::vertical()
            .element(left_bar)
            .element(elements::Break::new(0.5))
            .element(passive_perception.padded(1))
            .element(elements::Break::new(1.0))
            .element(elements::LinearLayout::vertical()
                .element(Paragraph::default()
                    .styled_string("Armor: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.0,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(Paragraph::default()
                    .styled_string("Weapons: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.1,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(Paragraph::default()
                    .styled_string("Languages: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.2,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(Paragraph::default()
                    .styled_string("Tools: ",style::Style::new().bold().with_font_size(7))
                    .styled_string(other_profs.3,style::Style::new().with_font_size(7))
                    .aligned(Alignment::Center)
                    .padded(1)
                )
                .element(Paragraph::new("OTHER PROFICIENCIES & LANGUAGES")
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
        .element(traits_elemt)
        .push().expect("failed to add row");
    doc.push(main_sheet);
    println!("Rendering pdf...(this may take a moment)");
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
fn element_from_score(score: &AbilityScore)->elements::LinearLayout{
    elements::LinearLayout::vertical()
        .element(
            Paragraph::new(score.get_name().to_uppercase())
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            Paragraph::new(bns_translator(score.get_modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            Paragraph::new(score.get_score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        )
}
fn element_from_skill(skill: &Skill, symb_fnt: &style::Style)->elements::StyledElement<Paragraph>{
    let mut bns = bns_translator(skill.get_mod());
    if bns.bytes().count() == 2 {
        bns=String::from(" ")+&bns;
    }
    Paragraph::default()
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
/// pads a String vertically(ie with new lines) to lines long, assuming a line width of `width` characters(under worst case)
fn vertical_pad(txt: String, width: usize, lines: usize)->elements::LinearLayout{
    let mut wrapped= textwrap::wrap(&txt,width).into_iter();
    let mut out = elements::LinearLayout::vertical();
    for _idx in 0..lines{
        if let Some(mut thing)=wrapped.next(){
            out=out.element(Paragraph::new(thing.to_mut().as_str()).aligned(Alignment::Center));
        } else {
            out=out.element(elements::Break::new(1.0));
        }
    }
    out
}