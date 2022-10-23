use serde_json::Value;

use std::str::FromStr;

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
pub enum Proficiency {
    None,
    Half,
    Profficient,
    Expert,
}
///A skill is a bonus, name, and prof
pub struct Skill{
    bonus: i64,
    name: String,
    prof_rank: Proficiency
}
impl Skill{
    pub fn get_prof(&self)->&Proficiency{
        &self.prof_rank
    }
    pub fn get_mod(&self,prof_bonus: i64)->i64{
        self.bonus
    }
    pub fn new(name: String, bonus: i64, prof_rank: Proficiency)->Skill{
        Skill {bonus, name, prof_rank}
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
    pub fn to_string(&self)->String{
        format!("{}d{}",self.num,self.size).to_string()
    }
    pub fn new(size: i64,num: i64)->Die{
        Die { size, num }
    }
}
///an attack bouns can be a regular bonus or DC
pub enum AtkBonus{
    Bonus(i64),
    DC(i64),
}
impl AtkBonus{
    pub fn to_string(&self)->String{
        match self{
            AtkBonus::Bonus(k)=>{
                if k>&0{
                    format!("+{}",k).to_string()
                } else {
                    format!("{}",k).to_string()
                }
            },
            AtkBonus::DC(k) =>{
                format!("DC {}",k).to_string()
            }
        }
    }
}
///an attack is a string, AtkBonus, and damage
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
    pub fn add_spell(&mut self,String spell){
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
    pub initiative: i64,
    pub speed: i64,
    pub hit_points: i64,
    pub hit_dice: Vec<Die>,
    pub attacks: Vec<Attack>,
    pub equipped: Vec<Item>,
    pub traits: (String, String, String, String),
    pub features: Vec<String>,
    pub other_profs: Vec<String>,
    pub carried: Vec<Item>,
    pub coins: (i64,i64,i64,i64,i64),
    pub spell_lists: Vec<SpellList>,
    pub spell_slots: (i64,i64,i64,i64,i64,i64,i64,i64,i64)
}

impl Character{
    fn metaSearch(field: &Vec<Value>,targets:&Vec<&str>)->bool{
        let mut idx =0;
        for val in field{
            for target in targets{
                if val.as_str().unwrap().contains(target){
                    return true;
                }
            }
            idx +=1;
        }
        return false;
    }
    pub fn new(char_json: Value)->Character{
        let char_name=&char_json["creatures"][0]["name"];
        let subclass_names: Vec<&str>=vec!["Subrace","Season","Type","Creed"];
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
        let mut speed: i64=0;
        let mut hit_points: i64=0;
        let mut ac: i64=0;
        let mut traits = (String::new(),String::new(),String::new(),String::new());
        let mut attacks: Vec<Attack> =vec![];
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
        let mut other_profs: Vec<String> = vec![];
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
                } else {
                    other_profs.push(val["name"].as_str().unwrap().to_string());
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
                    let size: i64=ds.split("d").collect::<Vec<_>>()[1].parse().unwrap();
                    hit_dice.push(Die::new(size,total));
                }
            }else if val["type"].as_str()==Some("action") && val["actionType"].as_str()==Some("attack"){
                let bns = AtkBonus::Bonus(val["attackRoll"]["value"].as_i64().unwrap());
                let id = val["_id"].as_str();
                let mut delta = 1;
                let mut damage = String::new();
                let mut dmg_die= String::new();
                let mut dmg_type= String::new();
                let mut dmg_bonus: i64=0;
                while props[idx+delta]["parent"]["id"].as_str()==id{
                    let valNxt = &props[idx+1];
                    if valNxt["type"].as_str()==Some("damage"){
                        dmg_die = valNxt["amount"]["value"].as_str().unwrap().to_string();
                        dmg_type = valNxt["damageType"].as_str().unwrap().to_string();
                        dmg_bonus = match valNxt["effects"][0]["amount"]["value"].as_i64(){
                            Some(k)=>k,
                            None=>0
                        };
                    }
                    damage += &format!("{}+{}[{}]",dmg_die,dmg_bonus,dmg_type);
                    delta +=1;
                }
                attacks.push(Attack::new(val["name"].as_str().unwrap().to_string(),
                    bns,damage));

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
            } else if val["tags"].as_array().unwrap().len()>0 && val["tags"].as_array().unwrap()[0].as_str()==Some("background"){
                background=Background::new(val["name"].as_str().unwrap().to_string(),
                    val["description"].as_str().unwrap().to_string());
            } else if val["tags"].as_array().unwrap().len()>0 && val["tags"].as_array().unwrap()[0].as_str()==Some("race"){
                if race==String::new(){
                    race = val["name"].as_str().unwrap().to_string();
                }
            } else if Self::metaSearch(val["tags"].as_array().unwrap(),&subclass_names){
                race = val["name"].as_str().unwrap().to_string();
            }
            idx +=1;
            if idx % 100 == 0{
                println!("Proccessed {} properties",idx);
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