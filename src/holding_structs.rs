use serde_json::Value;
use std::cmp::{PartialOrd,Ordering,Ord};
use std::collections::HashMap;
use owned_chars::OwnedChars;
use std::fmt::{self,Write};
///defines an ability score by the value(score) and name
pub struct AbilityScore{
    score: i64,
    name: String,
}
impl AbilityScore{
    pub fn get_score(&self)->i64{
        self.score
    }
    pub fn get_name(&self)->&String{
        &self.name
    }
    pub fn get_modifier(&self)->i64{
        self.score/2 -5
    }
    pub fn new(name: String, score: i64)->AbilityScore{
        AbilityScore { score, name}
    }
}
///Types of proficiency listed
#[derive(Debug, Eq, PartialEq,PartialOrd,Ord)]
pub enum Proficiency {
    None,
    Half,
    Profficient,
    Expert,
}
///A skill is a bonus, name, and prof
#[derive(Debug, Eq, PartialEq)]
pub struct Skill{
    bonus: i64,
    name: String,
    prof_rank: Proficiency
}
impl Skill{
    pub fn get_prof(&self)->&Proficiency{
        &self.prof_rank
    }
    pub fn get_mod(&self)->i64{
        self.bonus
    }
    pub fn get_name(&self)->&String{
        &self.name
    }
    pub fn new(name: String, bonus: i64, prof_rank: Proficiency)->Skill{
        Skill {bonus, name, prof_rank}
    }
}
impl PartialOrd for Skill{
    fn partial_cmp(&self, other: &Skill)->Option<Ordering>{
        if self.get_name()!=other.get_name(){
            return self.get_name().partial_cmp(other.get_name());
        }
        if self.get_prof()!=other.get_prof() {
            return self.get_prof().partial_cmp(other.get_prof());
        }
        self.get_mod().partial_cmp(&other.get_mod())
    }
}
impl Ord for Skill{
    fn cmp(&self, other: &Skill)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
///A class is a name and a level
pub struct Class{
    name: String,
    level: i64,
}
impl Class{
    pub fn get_name(&self)->&String{
        &self.name
    }
    pub fn get_level(&self)->i64{
        self.level
    }
    pub fn new(name: String, level: i64)->Class{
        Class { name, level}
    }
}
///a background is a name and a description
pub struct Background{
    name: String,
    description: String,
}
impl Background{
    pub fn get_name(&self)->&String{
        &self.name
    }
    pub fn get_description(&self)->&String{
        &self.description
    }
    pub fn new(name: String, description: String)-> Background{
        Background { name, description }
    }
}
///a dice has a size, and we include the number
pub struct Die{
    size: i64,
    num: i64,
}
impl Die{
    pub fn get_size(&self)->i64{
        self.size
    }
    pub fn get_num(&self)->i64{
        self.num
    }
    pub fn new(size: i64,num: i64)->Die{
        Die { size, num }
    }
}
impl fmt::Display for Die{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}d{}",self.num,self.size)
    }
}
///an attack bouns can be a regular bonus or DC
#[derive(Debug, Eq, PartialEq,Clone,PartialOrd,Ord)]
pub enum AtkBonus{
    Bonus(i64),
    DC(i64),
}
impl fmt::Display for AtkBonus{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            AtkBonus::Bonus(k)=>{
                if k>&0{
                    write!(f,"+{}",k)
                } else {
                    write!(f,"{}",k)
                }
            },
            AtkBonus::DC(k) =>{
                write!(f,"DC {}",k)
            }
        }
    }
}
///an attack is a string, AtkBonus, and damage
#[derive(Debug, Eq, PartialEq, Clone,PartialOrd,Ord)]
pub struct Attack{
    name: String,
    bonus: AtkBonus,
    damage: String
}
impl Attack{
    pub fn get_name(&self)->&String{
        &self.name
    }
    pub fn get_bonus(&self)->&AtkBonus{
        &self.bonus
    }
    pub fn get_bonus_as_string(&self)->String{
        self.bonus.to_string()
    }
    pub fn get_damage(&self)->&String{
        &self.damage
    }
    pub fn new(name: String,bonus: AtkBonus,damage: String)->Attack{
        Attack { name , bonus, damage }
    }
    pub fn add_dmg(&mut self, dmg: String){
        if !self.damage.is_empty(){
            let _= write!(self.damage,", {}",dmg);
        }
        else{
            self.damage=dmg;
        }
    }
}
///an item has a quantity and a name
pub struct Item{
    quantity: i64,
    name: String,
}
impl Item{
    pub fn get_quantity(&self)->i64{
        self.quantity
    }
    pub fn get_name(&self)->&String{
        &self.name
    }
    pub fn new(quantity: i64,name: String)->Item{
        Item { quantity, name}
    }
}
///We store all spells of the same level in the same SpellLevel struct
pub struct SpellLevel{
    level: i64,
    spells: Vec<String>
}
impl SpellLevel{
    pub fn get_lvl(&self)->i64{
        self.level
    }
    pub fn get_spells(&self)->&Vec<String>{
        &self.spells
    }
    pub fn new(level: i64, spells: Vec<String>)->SpellLevel{
        SpellLevel { level, spells }
    }
    pub fn add_spell(&mut self,spell: String){
        self.spells.push(spell);
    }
}
///a spell list has spells of several levels, but with a casting class, ability, save dc, and attack bonus
pub struct SpellList{
    pub levels: Vec<SpellLevel>,
    casting_class: String,
    casting_ability: String,
    save_dc: i64,
    atk_bonus: i64,
}
impl SpellList{
    pub fn get_levels(&self)->&Vec<SpellLevel>{
        &self.levels
    }
    pub fn get_class(&self)->&String{
        &self.casting_class
    }
    pub fn get_ability(&self)->&String{
        &self.casting_ability
    }
    pub fn get_dc(&self)->i64{
        self.save_dc
    }
    pub fn get_bonus(&self)->i64{
        self.atk_bonus
    }
    pub fn new(levels: Vec<SpellLevel>,casting_class: String,casting_ability: String,save_dc: i64, atk_bonus: i64)->SpellList{
        SpellList{levels, casting_class, casting_ability, save_dc, atk_bonus}
    }
}
///a damage multiplier has Immunity, Resistence, Vulnerability, each with a string damage type
pub enum DamageMult{
    Immune(String),
    Resist(String),
    Vuln(String),
}
///a struct for parsing the character into
pub struct Character{
    pub char_name: String,
    pub classes: Vec<Class>,
    pub background: Background,
    pub damage_mults: Vec<DamageMult>,
    pub race: String,
    pub alignment: String,
    pub xp: i64,
    pub ability_scores: Vec<AbilityScore>,
    pub prof_bonus: i64,
    pub saving_throws: Vec<Skill>,
    pub skills: Vec<Skill>,
    pub ac: i64,
    pub passive_bonus: i64,
    pub initiative: i64,
    pub speed: i64,
    pub hit_points: i64,
    pub hit_dice: Vec<Die>,
    pub attacks: Vec<Attack>,
    pub equipped: Vec<Item>,
    pub traits: (String, String, String, String),//Personality, Ideals, Bonds, Flaws
    pub features: Vec<String>,
    pub other_profs: (Vec<String>,Vec<String>,Vec<String>,Vec<String>),//armor,weapon,language, tool
    pub carried: Vec<Item>,
    pub coins: (i64,i64,i64,i64,i64),//cp,sp,ep,gp,pp
    pub spell_lists: Vec<SpellList>,
    pub spell_slots: (i64,i64,i64,i64,i64,i64,i64,i64,i64)//1st,2nd,...9th
}

impl Character{
    pub fn new(char_json: Value,race_decoder: Value)->Character{
        let char_name=&char_json["creatures"][0]["name"];
        if char_name == &Value::Null{
            panic!("cannot find char name, probably because the api is wrong");
        }
        let char_name = char_name.as_str().unwrap().to_string();
        let alignment=char_json["creatures"][0]["alignment"].as_str().unwrap().to_string();
        let xp: i64=char_json["creatures"][0]["denormalizedStats"]["xp"].as_i64().unwrap();
        let mut ability_scores: Vec<AbilityScore> = vec![];
        let mut skills: Vec<Skill> = vec![];
        let mut saving_throws: Vec<Skill> = vec![];
        let mut initiative: i64=0;
        let mut prof_bonus: i64=0;
        let mut damage_mults: Vec<DamageMult> = vec![];
        let mut passive_bonus: i64 =0;
        let mut speed: i64=0;
        let mut hit_points: i64=0;
        let mut ac: i64=0;
        let mut traits = (String::new(),String::new(),String::new(),String::new());
        let mut attacks_dict: HashMap<String,Attack> =HashMap::new();
        let mut attacks: Vec<Attack>=vec![];
        let mut classes: Vec<Class> = vec![];
        let props: &Value = &char_json["creatureProperties"];
        let mut features: Vec<String> = vec![];
        let mut equipped: Vec<Item> = vec![];
        let mut carried: Vec<Item> = vec![];
        let mut hit_dice: Vec<Die> = vec![];
        let mut idx =0;
        let mut background: Background=Background::new(String::new(),String::new());
        let mut race: String = String::new();
        let mut coins = (0,0,0,0,0);
        let mut other_profs: (Vec<String>,Vec<String>,Vec<String>,Vec<String>) = (vec![],vec![],vec![],vec![]);
        while props[idx] != Value::Null{
            let val = &props[idx];
            if val["type"].as_str()==Some("attribute") && val["attributeType"].as_str()==Some("ability"){
                ability_scores.push(AbilityScore::new(val["name"].as_str().unwrap().to_string(),
                    val["total"].as_i64().unwrap()));
            } else if val["type"].as_str()==Some("skill"){
                if val["name"].as_str()==Some("Initiative"){
                    initiative=val["value"].as_i64().unwrap();
                } else if val["skillType"].as_str()==Some("save"){
                    let prf=val["proficiency"].as_f64();
                    let prof = if prf ==Some(0.5){
                        Proficiency::Half
                    } else if prf == Some(1.0){
                        Proficiency::Profficient
                    } else if prf == Some(2.0){
                        Proficiency::Expert
                    } else {
                        Proficiency::None
                    };
                    saving_throws.push(Skill::new(val["name"].as_str().unwrap().to_string(),
                        val["value"].as_i64().unwrap(),prof));
                } else if val["skillType"].as_str()==Some("skill"){
                    let prf=val["proficiency"].as_f64();
                    let prof = if prf ==Some(0.5){
                        Proficiency::Half
                    } else if prf == Some(1.0){
                        Proficiency::Profficient
                    } else if prf == Some(2.0){
                        Proficiency::Expert
                    } else {
                        Proficiency::None
                    };
                    skills.push(Skill::new(val["name"].as_str().unwrap().to_string(),
                        val["value"].as_i64().unwrap(),prof));
                    if val["name"].as_str()==Some("Perception"){
                        passive_bonus = val["passiveBonus"].as_i64().unwrap();
                    }
                } else if val["skillType"].as_str()==Some("armor"){
                    other_profs.0.push(val["name"].as_str().unwrap().to_string());
                } else if val["skillType"].as_str()==Some("weapon"){
                    other_profs.1.push(val["name"].as_str().unwrap().to_string());
                } else if val["skillType"].as_str()==Some("language"){
                    other_profs.2.push(val["name"].as_str().unwrap().to_string());
                } else if val["skillType"].as_str()==Some("tool"){
                    other_profs.3.push(val["name"].as_str().unwrap().to_string());
                }
            }else if val["type"].as_str()==Some("feature"){
                features.push(val["name"].as_str().unwrap().to_string());
            }else if val["type"].as_str()==Some("note"){
                let failsafe = String::new();
                if val["name"].as_str()==Some("Flaws"){
                    traits.3 = match val["summary"]["text"].as_str(){
                        Some(s)=>s.to_string(),
                        None=>failsafe
                    };
                } else if val["name"].as_str()==Some("Ideals"){
                    traits.1 = match val["summary"]["text"].as_str(){
                        Some(s)=>s.to_string(),
                        None=>failsafe
                    };
                } else if val["name"].as_str()==Some("Personality Traits"){
                    traits.0 =match val["summary"]["text"].as_str(){
                        Some(s)=>s.to_string(),
                        None=>failsafe
                    };
                } else if val["name"].as_str()==Some("Bonds"){
                    traits.2 = match val["summary"]["text"].as_str(){
                        Some(s)=>s.to_string(),
                        None=>failsafe
                    };
                }
            }else if val["type"].as_str()==Some("attribute") && val["attributeType"].as_str()==Some("hitDice"){
                let total: i64 = val["total"].as_i64().unwrap();
                if total>0{
                    let ds: String = val["hitDiceSize"].as_str().unwrap().to_string();
                    let size: i64=ds.split('d').collect::<Vec<_>>()[1].parse().unwrap();
                    hit_dice.push(Die::new(size,total));
                }
            }else if val["type"].as_str()==Some("action") && val["actionType"].as_str()==Some("attack"){
                let bns = AtkBonus::Bonus(val["attackRoll"]["value"].as_i64().unwrap());
                let id = val["_id"].as_str().unwrap().to_string();
                let dmg = match attacks_dict.get(&id){
                    Some(atk)=>atk.get_damage(),
                    None=>""
                };
                attacks_dict.insert(id,Attack::new(val["name"].as_str().unwrap().to_string(),bns,dmg.to_string()));
            }else if val["type"].as_str()==Some("damage"){
                let par_id = val["parent"]["id"].as_str().unwrap().to_string();
                let dmg_die = val["amount"]["calculation"].as_str().unwrap();
                let dmg_bonus = val["amount"]["effects"][0]["amount"]["value"].as_i64().unwrap_or(0);
                let dmg_type = val["damageType"].as_str().unwrap().to_string();
                let dmg_string = format!("{}{}{}[{}]",dmg_die,if dmg_bonus>=0 {"+"} else {""},
                dmg_bonus,damage_type_abreviator(dmg_type));
                match attacks_dict.get_mut(&par_id){
                    Some(atk)=>atk.add_dmg(dmg_string),
                    None=>{attacks_dict.insert(par_id,Attack::new(String::new(),AtkBonus::Bonus(0),dmg_string));},
                };
            }else if val["type"].as_str()==Some("class"){
                classes.push(Class::new(val["name"].as_str().unwrap().to_string(),
                    val["level"].as_i64().unwrap()));
            }else if val["type"].as_str()==Some("Item"){
                if val["equipped"].as_bool().unwrap(){
                    equipped.push(Item::new(val["quantity"].as_i64().unwrap(),
                        val["name"].as_str().unwrap().to_string()));
                } else if val["name"].as_str().unwrap().contains("piece"){
                    let coin_type=val["name"].as_str().unwrap().to_string();
                    if coin_type.contains("Platinum"){
                        coins.4 = val["quantity"].as_i64().unwrap();
                    } else if coin_type.contains("Gold"){
                        coins.3 = val["quantity"].as_i64().unwrap();
                        //electrum doesn't exist in dc v2
                    } else if coin_type.contains("Silver"){
                        coins.1 = val["quantity"].as_i64().unwrap();
                    } else if coin_type.contains("Copper"){
                        coins.0 = val["quantity"].as_i64().unwrap();
                    }
                }else {
                    let q: i64 = match val["quantity"].as_i64(){
                        Some(k) => k,
                        None=>continue,
                    };
                    carried.push(Item::new(q,
                    val["name"].as_str().unwrap().to_string()));
                }
            }else if val["name"].as_str()==Some("Proficiency Bonus"){
                prof_bonus=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()==Some("Speed") && val["type"].as_str()==Some("attribute"){
                speed=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()==Some("Hit Points") && val["type"].as_str()==Some("attribute"){
                hit_points=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()==Some("Armor Class") && val["type"].as_str()==Some("attribute"){
                ac=val["total"].as_i64().unwrap();
            } else if !val["tags"].as_array().unwrap().is_empty() && val["tags"].as_array().unwrap()[0].as_str()==Some("background"){
                background=Background::new(val["name"].as_str().unwrap().to_string(),
                    val["description"].as_str().unwrap().to_string());
            } else if val["type"].as_str()==Some("constant")&&val["variableName"].as_str()==Some("race"){
                if race==String::new(){
                    race = val["calculation"].as_str().unwrap().to_string().replace('\"',"");
                }
            } else if val["type"].as_str()==Some("constant") && val["variableName"].as_str()==Some("subRace"){
                race = val["calculation"].as_str().unwrap().to_string().replace('\"',"");
            }
            idx +=1;
            if idx % 100 == 0{
                println!("Proccessed {} properties",idx);
            }
        }
        let race = race_translator(race,race_decoder);
        for pair in attacks_dict.iter(){
            if !pair.1.get_name().is_empty(){
                attacks.push(pair.1.clone());
            }
        }
        Character{
            char_name,
            classes,
            background,
            damage_mults,
            race,
            alignment,
            xp,
            ability_scores,
            prof_bonus,
            saving_throws,
            skills,
            ac,
            passive_bonus,
            initiative,
            speed,
            hit_points,
            hit_dice,
            attacks,
            equipped,
            traits,
            features,
            other_profs,
            carried,
            coins,
            spell_lists: vec![],
            spell_slots: (0,0,0,0,0,0,0,0,0)
        }
    }
}
fn damage_type_abreviator(typ: String)->String{
    if typ.len()<5{
        return typ;
    }else if &typ == "piercing"{
        return "pir.".to_string();
    }
    let mut typ_bits=typ.into_bytes();
    typ_bits.truncate(3);
    String::from_utf8(typ_bits).expect("should never happen by design")+"."
}
fn race_translator(race: String,race_decoder: Value)-> String{
    if race.is_empty(){return race};//deal with this nasty edge case
    //if the race is one of the special ones in the decoder, do that
    if let Some(out)=race_decoder[&race].as_str(){
        return out.to_string();
    }
    //if it allready has a space, it is probably formated right
    if race.contains(' '){
        return race;
    }
    //otherwise assume lowerCammelCase
    let mut race_chars = OwnedChars::from_string(race);
    let mut out: Vec<char>= vec![];
    //make the first character upper case(unicode is cursed)
    for ch in race_chars.next().unwrap().to_uppercase(){
        out.push(ch);
    }
    //loop over the other characters, if they are uppercase add a space. The upper case check is that way because unicode
    for ch in race_chars{
        if ch.is_uppercase() && !ch.is_lowercase(){
            out.push(' ');
        }
        out.push(ch);
    }
    out.into_iter().collect()
}