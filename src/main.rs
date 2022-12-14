use genpdf::{Element, Alignment};
use genpdf::{elements::{self,Paragraph},fonts, style};
use dicecloud_sheet_printer::{generate_pdf,get_token,get_character,get_char_url,bns_translator,get_img_from_url,holding_structs::*};
use std::collections::HashMap;
use std::convert::TryInto;

use std::{io,process,fs};
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
    let mut token =String::new();
    if username.trim() == String::new(){
        println!("No username detected, would you like to try accessing the character with no token?(y/n)");
        let mut ans= String::new();
        stdin.read_line(&mut ans).expect("failed to get answer");
        if !ans.to_lowercase().contains('y'){
            println!("Exiting to terminal");
            process::exit(0);
        }
        println!("continuing");
    } else {
        let mut psw = String::new();
        println!("Password:");
        stdin.read_line(&mut psw).expect("Fallied to get password");
        token = get_token(username, psw).await;
        if token.is_empty(){
            println!("Failed to login! Try accessing with no token?(y/n)");
            let mut ans= String::new();
            stdin.read_line(&mut ans).expect("failed to get answer");
            if !ans.to_lowercase().contains('y'){
                println!("Exiting to terminal");
                process::exit(0);
            }
            println!("continuing");
        } else {
            println!("Successfully logged in");
        }
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
    println!("Processing Character(this may take a while)...");
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
        if want_xp.to_lowercase().contains('y'){
            xp+=&character.xp.to_string();
        }
    }
    let mut detail_right = elements::TableLayout::new(vec![2,2,1]);
    detail_right.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let mut classes = character.classes;
    classes.sort();
    let class_str: String = if classes.len()==1{
        classes.iter().map(|class| format!("{} {}",class.name(),class.level())).collect::<Vec<String>>().join(" ")
    } else {
        classes.iter().map(|class| format!("{}. {}",class.name().chars().take(3).collect::<String>(),class.level())).collect::<Vec<String>>().join(" ")
    };
    detail_right
        .row()
        .element(
            Paragraph::new(class_str).styled(style::Style::new().with_line_spacing(0.5))
        )
        .element(
            Paragraph::new(character.background.name()).styled(style::Style::new().with_line_spacing(0.5))
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
        ability_scores.insert(score.name(),score);
    }
    main_sheet.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));    
    let mut left_bar = elements::TableLayout::new(vec![1,2]);
    left_bar.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let mut skills = character.skills;
    let mut saves: HashMap<String,&Skill> = HashMap::new();
    for save in &character.saving_throws{
        let nme = save.name().replace(" Save","");
        saves.insert(nme,save);
    }
    let saving_throws = elements::LinearLayout::vertical()
        .element(element_from_skill(saves.get("Strength").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Dexterity").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Constitution").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Intelligence").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Wisdom").unwrap(),&symbol))
        .element(element_from_skill(saves.get("Charisma").unwrap(),&symbol))
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
        if skill.name()=="Perception"{
            passive_bonus+=skill.modifier()+character.passive_bonus;
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
                .styled_string("???",symbol.with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("???",symbol.with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("??? ",symbol.with_font_size(7))
                .aligned(Alignment::Right)
            )
            .element(Paragraph::default()
                .styled_string("FAILURES ",style::Style::new().with_font_size(7))
                .styled_string("???",symbol.with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("???",symbol.with_font_size(7))
                .styled_string("-",style::Style::new().with_font_size(7))
                .styled_string("??? ",symbol.with_font_size(7))
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
        atk_dict.insert(atk.name().clone(),atk);
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
                println!("{}",atk_dict.iter().map(|atk| atk.1.name().clone()).collect::<Vec<_>>().join(", "));
            } else if &current_inst.trim().to_lowercase()=="selection"{
                println!("{} ({}/23)",to_display.iter().map(|atk| atk.name().clone()).collect::<Vec<_>>().join(", "),to_display.len());
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
        let nme = atk.name().to_string();
        let mut trunk = nme.replace(" (Two-Handed)","(2H)");
        trunk.truncate(13);
        attack_display.row()
            .element(Paragraph::new(trunk)
                .styled(style::Style::new().bold().with_font_size(10))
                )
            .element(
                Paragraph::new(atk.bonus_as_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(10))
            )
            .element(
                Paragraph::new(atk.damage())
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
            .styled(style::Style::new().with_font_size(7).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let ideal = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.1,28,2))
        .element(Paragraph::new("IDEALS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(7).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let bond = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.2,28,2))
        .element(Paragraph::new("BONDS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(7).bold()))
        .padded(1)
        .framed()
        .padded(1);
    let flaw = elements::LinearLayout::vertical()
        .element(vertical_pad(traits.3,28,2))
        .element(Paragraph::new("FLAWS")
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(7).bold()))
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
        .element(elements::Break::new(0.25));
    let mut features_elem= elements::LinearLayout::vertical();
    features_elem=features_elem.element(Paragraph::new("ACTIONS").aligned(Alignment::Center)
        .styled(style::Style::new().bold().with_font_size(7)));
    let mut actions = character.actions;
    let mut equipment = character.equipment;
    equipment.sort();
    actions.sort();
    actions.push(Action::default());
    let mut features = character.features;
    let mut dmg_mults = character.damage_mults;
    dmg_mults.sort();
    features.sort();
    let mut resources = character.resources;
    resources.sort();
    let re = regex::Regex::new(r"Pass (Dawn|Dusk|Midnight)").unwrap();
    let mut features = resources.into_iter().map(|r| r.to_string()).chain(dmg_mults.into_iter().map(|mul| mul.to_string()))
        .chain(features.into_iter().filter(|feat| !actions.iter().any(|x| feat==x.name())));
    let mut actions_itr= actions.iter().filter(|act| !re.is_match(act.name()))
        .filter(|act| !equipment.iter().any(|x| act.name()==x.name() && act.uses() !=-1))
        .map(|act| act.to_string());
    for _i in 0..26{
        if let Some(name)=actions_itr.next(){
            if name==Action::default().to_string(){
                features_elem=features_elem.element(Hline::new());
                features_elem=features_elem.element(Paragraph::new("FEATURES").aligned(Alignment::Center)
                    .styled(style::Style::new().bold().with_font_size(7)))
            } else {
                features_elem=features_elem.element(Paragraph::new(name).aligned(Alignment::Center)
                    .styled(style::Style::new().with_font_size(10)));
            }
        } else if let Some(name)=features.next(){
            features_elem=features_elem.element(Paragraph::new(name).aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(10)));
        }else {
            features_elem=features_elem.element(elements::Break::new(1.0).styled(style::Style::new().with_font_size(10)));
        }

    }
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
        .element(elements::LinearLayout::vertical()
            .element(traits_elemt)
            .element(elements::Break::new(0.25))
            .element(features_elem.padded(1).framed().padded(1))
            .element(elements::PageBreak::new())
        )
        .push().expect("failed to add row");
    doc.push(main_sheet);
    
    // page 2 starts
    let mut page_2 = elements::TableLayout::new(vec![3,1]);
    let mut equipment_elem = elements::LinearLayout::vertical()
        .element(Paragraph::new("EQUIPMENT").aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7)));
    for itm in equipment.iter(){
        let q = itm.quantity();
        let nme = if q==1{
            itm.name()
        } else {
            itm.plural_name()
        };
        equipment_elem = equipment_elem.element(Paragraph::default().styled_string(if itm.requires_attunement() {"??? "} else {""},symbol)
                .string(format!("{}{}",if q==1 {"".to_string()} else {q.to_string()+" "},nme))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(10))
            );
    }
    let mut features_elem2= elements::LinearLayout::vertical();
    features_elem2=features_elem2.element(Paragraph::new("OTHER FEATURES & TRAITS").aligned(Alignment::Center)
        .styled(style::Style::new().bold().with_font_size(7)));
    for name in actions_itr{
        if name==String::default(){
            features_elem2=features_elem2.element(Hline::new());
            features_elem2=features_elem2.element(Paragraph::new("FEATURES").aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)))
        } else {
            features_elem2=features_elem2.element(Paragraph::new(name).aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(10)));
        }
    }
    for name in features{
        features_elem2=features_elem2.element(Paragraph::new(name).aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(10)));
    }
    let coins = character.coins;
    let mut equiptable = elements::TableLayout::new(vec![1,9]);
    equiptable.row()
        .element(elements::LinearLayout::vertical()
            .element(elements::Break::new(1.0))
            .element(Paragraph::new("CP").aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)))
            .element(Paragraph::new(&coins.0.to_string()).aligned(Alignment::Center).padded(1).framed().padded(1))
            .element(elements::Break::new(0.5))
            .element(Paragraph::new("SP").aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)))
            .element(Paragraph::new(&coins.1.to_string()).aligned(Alignment::Center).padded(1).framed().padded(1))
            .element(elements::Break::new(0.5))
            .element(Paragraph::new("EP").aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)))
            .element(Paragraph::new(&coins.2.to_string()).aligned(Alignment::Center).padded(1).framed().padded(1))
            .element(elements::Break::new(0.5))
            .element(Paragraph::new("GP").aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)))
            .element(Paragraph::new(&coins.3.to_string()).aligned(Alignment::Center).padded(1).framed().padded(1))
            .element(elements::Break::new(0.5))
            .element(Paragraph::new("PP").aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(7)))
            .element(Paragraph::new(&coins.4.to_string()).aligned(Alignment::Center).padded(1).framed().padded(1))
            )
        .element(equipment_elem.padded(1))
        .push().expect("Failed to add row");
    let background = character.background;
    let mut img_elem = elements::LinearLayout::vertical();
    if !character.char_img.is_empty(){
        let img = get_img_from_url(character.char_img).await;
        img_elem=img_elem.element(elements::Image::from_dynamic_image(img).expect("Image fail")
            .with_scale(genpdf::Scale{x:0.9,y:0.9})
            .with_alignment(Alignment::Center));

    } else {
        img_elem=img_elem.element(elements::Break::new(6.0));
    }
    img_elem=img_elem.element(Paragraph::new("CHARACTER PORTRAIT").aligned(Alignment::Center)
        .styled(style::Style::new().bold().with_font_size(7)));
    page_2
        .row()
        .element(elements::LinearLayout::vertical()
            .element(equiptable.framed().padded(1))
            .element(features_elem2.padded(1).framed().padded(1))
        )
        .element(elements::LinearLayout::vertical()
                .element(img_elem
                    .padded(1)
                    .framed()
                    .padded(1)
                )
                .element(elements::LinearLayout::vertical()
                    .element(Paragraph::new("BACKGROUND").aligned(Alignment::Center)
                        .styled(style::Style::new().bold().with_font_size(7)))
                    .element(Paragraph::new(background.background_feature().name()).aligned(Alignment::Center)
                        .styled(style::Style::new().bold()))
                    .element(Paragraph::new(background.background_feature().description()).aligned(Alignment::Center)
                        .styled(style::Style::new().with_font_size(10)))
                    .padded(1)
                    .framed()
                    .padded(1)
                )
        )
        .push().expect("Failed to add row");
    doc.push(page_2);
    let mut spl_lists = character.spell_lists;
    let spl_slots = character.spell_slots;
    if !spl_lists.is_empty(){
        doc.push(elements::PageBreak::new());
        doc.push(elements::Paragraph::new("SPELLS").styled(style::Style::new().bold()));
        let mut spell_slots_table = elements::TableLayout::new(vec![1,1,1,1,1,1,1,1,1]);
        let slt_fmt = style::Style::new().bold().with_font_size(7);
        spell_slots_table.row()
            .element(spell_slot_elem(&spl_slots, 1,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 2,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 3,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 4,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 5,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 6,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 7,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 8,symbol,slt_fmt).padded(1).framed())
            .element(spell_slot_elem(&spl_slots, 9,symbol,slt_fmt).padded(1).framed())
            .push().expect("failed to add row");
        doc.push(spell_slots_table);
        spl_lists.sort();
        for ls in spl_lists{
            let mut spell_header = elements::TableLayout::new(vec![3,1,1,1]);
            spell_header.row()
                .element(Paragraph::new(&ls.name).aligned(Alignment::Center)
                    .styled(style::Style::new().bold().with_font_size(14)).padded(1).framed().padded(1))
                .element(elements::LinearLayout::vertical()
                    .element(Paragraph::new(bns_translator(ls.atk_bonus)).aligned(Alignment::Center)
                        .styled(style::Style::new().with_font_size(10)))
                    .element(Paragraph::new("ATTACK BONUS").aligned(Alignment::Center)
                        .styled(slt_fmt))
                    .padded(1).framed().padded(1)
                )
                .element(elements::LinearLayout::vertical()
                    .element(Paragraph::new(format!("DC {}",ls.save_dc)).aligned(Alignment::Center)
                        .styled(style::Style::new().with_font_size(10)))
                    .element(Paragraph::new("SAVE DC").aligned(Alignment::Center)
                        .styled(slt_fmt))
                    .padded(1).framed().padded(1)
                )
                .element(elements::LinearLayout::vertical()
                    .element(Paragraph::new(format!("/{}",ls.max_prepared)).aligned(Alignment::Right)
                        .styled(style::Style::new().with_font_size(10)))
                    .element(Paragraph::new("PREPARED").aligned(Alignment::Center)
                        .styled(slt_fmt))
                    .padded(1).framed().padded(1)
                )
                .push().expect("failed to build row");
            doc.push(spell_header);
            let mut spell_column_specifier =elements::TableLayout::new(vec![1,11,3,4,5,2,4,11]);
            spell_column_specifier.row()
                .element(Paragraph::new("P").styled(slt_fmt))
                .element(Paragraph::new("NAME").styled(slt_fmt))
                .element(Paragraph::new("SCHOOL").styled(slt_fmt))
                .element(Paragraph::new("CAST TIME").styled(slt_fmt))
                .element(Paragraph::new("RANGE").styled(slt_fmt))
                .element(Paragraph::new("VSCR").styled(slt_fmt))
                .element(Paragraph::new("DUR").styled(slt_fmt))
                .element(Paragraph::new("MATERIAL").styled(slt_fmt))
                .push().expect("failed to add row");
            doc.push(spell_column_specifier);
            let mxlvl = ls.max_lvl();
            for i in 0..=mxlvl{
                if i==0{
                    doc.push(Paragraph::new("CANTRIPS").styled(style::Style::new().bold().with_font_size(11)));
                } else {
                    let ord = if i==1{"ST"} else if i==2{"ND"} else if i==3{"RD"} else {"TH"};
                    doc.push(Paragraph::new(format!("{}{} LEVEL SPELLS",i,ord))
                        .styled(style::Style::new().bold().with_font_size(11)));
                }
                if let Some(lvl)=ls.levels.get(&i){
                    let mut spells = lvl.spells().clone();
                    spells.sort();
                    let mut spl_table= elements::TableLayout::new(vec![1,11,3,4,5,2,4,11]);
                    for spl in spells{
                        row_from_spell(&mut spl_table, &spl,symbol);
                    }
                    doc.push(spl_table);
                } else {
                    doc.push(elements::Break::new(1.0).styled(style::Style::new().with_font_size(7)));
                }
            }
        }
    }
    let mut out_path = String::new();
    println!("What would you like the output file to be?");
    stdin.read_line(&mut out_path).expect("Failed to get output path");
    let mut out_path = out_path.trim().to_string();
    if !out_path.ends_with(".pdf"){
        out_path+=".pdf";
    }
    out_path = "./sheet_outputs/".to_string()+&out_path;
    println!("Rendering pdf...(this may take a moment)");
    doc.render_to_file(out_path).expect("Failed to write output file");
}
fn row_from_spell(spell_table: &mut elements::TableLayout, spl: &Spell, symb: style::Style){
    let scl: String = spl.school().chars().take(4).collect();
    let material: String = spl.material().chars().take(30).collect();
    let sty = style::Style::new().with_font_size(10);
    let prpd = match spl.prepd(){
        SpellPrep::AlwaysPrepared => Paragraph::new("A ").styled(sty),
        _=> Paragraph::default().styled_string("???",symb.with_font_size(10)).string(" ").styled(sty)
    };
    spell_table
        .row()
        .element(prpd)
        .element(Paragraph::new(spl.name()).styled(sty))
        .element(Paragraph::new(format!("{}.",scl)).styled(sty))
        .element(Paragraph::new(&spl.casting_time().to_string()).styled(sty))
        .element(Paragraph::new(spl.range()).styled(sty))
        .element(Paragraph::new(&spl.vscr_to_string()).styled(sty))
        .element(Paragraph::new(spl.duration()).styled(sty))
        .element(Paragraph::new(material).styled(sty))
        .push().expect("failed to add row");
}
fn element_from_score(score: &AbilityScore)->elements::LinearLayout{
    elements::LinearLayout::vertical()
        .element(
            Paragraph::new(score.name().to_uppercase())
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(7))
        )
        .element(
            Paragraph::new(bns_translator(score.modifier()))
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(20))
        )
        .element(
            Paragraph::new(score.score().to_string())
                .aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(7))
                .framed()
        )
}
fn element_from_skill(skill: &Skill, symb_fnt: &style::Style)->elements::StyledElement<Paragraph>{
    let mut bns = bns_translator(skill.modifier());
    if bns.len() == 2 {
        bns=String::from(" ")+&bns;
    }
    Paragraph::default()
        .styled_string(proficiency_translator(skill.prof()),*symb_fnt)
        .string(format!(" {}  {}",bns,skill.name()))
        .styled(style::Style::new().with_font_size(8))
}
fn proficiency_translator(prof: &Proficiency)->String{
    match prof{
        Proficiency::None => String::from("???"),
        Proficiency::Half => String::from("???"),
        Proficiency::Profficient => String::from("???"),
        Proficiency::Expert => String::from("???")
    }
}
/// pads a String vertically(ie with new lines) to lines long, assuming a line width of `width` characters(under worst case)
fn vertical_pad(txt: String, width: usize, lines: usize)->elements::LinearLayout{
    let mut wrapped= textwrap::wrap(&txt,width).into_iter();
    let mut out = elements::LinearLayout::vertical();
    for _idx in 0..lines{
        if let Some(mut thing)=wrapped.next(){
            out=out.element(Paragraph::new(thing.to_mut().as_str()).aligned(Alignment::Center)
                .styled(style::Style::new().with_font_size(10)));
        } else {
            out=out.element(elements::Break::new(1.0).styled(style::Style::new().with_font_size(10)));
        }
    }
    out
}
fn spell_slot_elem(spell_slots: &[i64],level: i64, symbol: style::Style, slt: style::Style)-> elements::LinearLayout{
    let ordinal = if level ==1{"ST"} else if level==2{"ND"} else if level == 3{"RD"} else {"TH"};
    elements::LinearLayout::vertical()
        .element(Paragraph::new(format!("{}{} LEVEL",level, ordinal)).aligned(Alignment::Center).styled(slt))
        .element(Paragraph::new(vec!["???"; spell_slots[(level-1) as usize].try_into().unwrap()].into_iter().collect::<String>())
            .aligned(Alignment::Center).styled(symbol.with_font_size(10)))
}