use serde_json::Value;

use std::str::FromStr;
struct AbilityScore{
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
enum Profficiency {
    None,
    Half,
    Profficient,
    Expert,
}
struct Skill{
    bonus: i64,
    name: String,
    prof_rank: Profficiency
}
impl Skill{
    pub fn get_prof(&self)->&Profficiency{
        &self.prof_rank
    }
    pub fn get_mod(&self,prof_bonus: i64)->i64{
        self.bonus
    }
    pub fn new(name: String, bonus: i64, prof_rank: Profficiency)->Skill{
        Skill {bonus, name, prof_rank}
    }
}
struct Class{
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
struct Background{
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
struct Die{
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
enum AtkBonus{
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
struct Attack{
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
struct Item{
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
struct SpellLevel{
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
}
struct SpellList{
    levels: Vec<SpellLevel>,
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
        SpellList{levels, casting class, casting_ability, save_dc, atk_bonus}
    }
}
pub enum DamageMult{
    Immune(String),
    Resist(String),
    Vuln(String),
}

pub struct Character{
    pub char_name: String,
    pub classes: Vec<Class>,
    pub background: String,
    pub damage_mults: Vec<DamageMult>
    pub race: String,
    pub alignment: String,
    pub xp: i64,
    pub ability_scores: Vec<AbilityScore>,
    pub prof_bonus: i64,
    pub saving_throws: Vec<Skill>
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
    pub coins: (i64,i64,i64,i64,i64)
    pub spell_lists: Vec<SpellList>
    pub spell_slots: (i64,i64,i64,i64,i64,i64,i64,i64,i64)
}
impl Character{
    pub fn new(char_json: Value)->Character{
        let char_name=char_json["creatures"][0]["name"];
        if char_name == Value::Null{
            panic!("cannot find char name, probably because the api is wrong");
        }
        let char_name = char_name.as_str().to_string();
        let alignment=char_json["creatures"][0]["alignment"].as_str().to_string().unwrap();
        let xp: i64=char_json["creatures"][0]["denormalizedStats"]["xp"].as_i64().unwrap();
        let mut ability_scores: Vec<AbilityScore> = vec![];
        let mut skills: Vec<Skill> = vec![];
        let mut saves: Vec<Skill> = vec![];
        let initiative: i64;
        let prof_bonus: i64;
        let speed: i64;
        let hit_points: i64;
        let ac: i64;
        let mut traits = (String::new(),String::new(),String::new(),String::new());
        let mut attacks: Vec<Attack> =vec![];
        let mut classes: Vec<Class> = vec![];
        let props: Value = char_json["creatureProperties"];
        let mut features: Vec<String> = vec![];
        let mut hit_dice: Vec<Die> = vec![];
        let mut idx =0;
        while props[idx] != Value::Null{
            let val = props[idx]
            if val["type"].as_str==Some("attribute") && val["attributeType"]==Some("ability"){
                ability_scores.push(AbilityScore::new(val["name"].as_str().to_string(),
                    val["total"].as_i64().unwrap()));
            } else if val["type"].as_str()==Some("skill"){
                if val["name"].as_str()==Some("Initiative"){
                    initiative=val["value"].as_i64().unwrap();
                } else if val["skillType"].as_str()==Some("save"){
                    let prf=val["proficiency"].as_str().unwrap();
                    let prof = if prf =="0.5"{
                        Profficiency::Half
                    } else if prf == "1"{
                        Profficiency::Profficient
                    } else if prf == "2"{
                        Proficiency::Expert;
                    } else {
                        Profficiency::None;
                    };
                    saves.push(Skill::new(val["value"].as_i64().unwrap(),
                        val["name"].as_str().to_string(),prof));
                } else if val["skillType"].as_str()==Some("skill"){
                    let prf=val["proficiency"].as_str().unwrap();
                    let prof = if prf =="0.5"{
                        Profficiency::Half
                    } else if prf == "1"{
                        Profficiency::Profficient
                    } else if prf == "2"{
                        Proficiency::Expert;
                    } else {
                        Profficiency::None;
                    };
                    skills.push(Skill::new(val["value"].as_i64().unwrap(),
                        val["name"].as_str().unwrap().to_string(),prof));
                }
            }else if val["type"].as_str()==Some("feature"){
                features.push(val["name"].as_str().unwrap().to_string());
            }else if val["type"].as_str()==Some("note"){
                if val["name"].as_str()==Some("Flaws"){
                    
                } else if val["name"].as_str()==Some("Ideals"){

                } else if val["name"].as_str()==Some("Personality Traits"){

                } else if val["name"].as_str()==Some("Bonds"){

                }
            }else if val["type"].as_str()==Some("attribute") && val["attributeType"].as_str()=Some("hitDice"){
                let total: i64 = val["total"].as_i64().unwrap()>0;
                if total>0{
                    let ds: String = val["hitDiceSize"].as_str().unwrap().to_string();
                    let size: i64=ds.split("d").collect()[1].parse().unwrap();
                    hit_dice.push(Die::new(size,total));
                }
            }else if val["type"].as_str()==Some("action"){
                
            }else if val["type"].as_str()==Some("class"){
                classes.push(Class::new(val["name"].as_str().unwrap().to_string(),
                    val["level"].as_i64().unwrap()));
            }else if val["name"].as_str()==Some("Proficiency Bonus"){
                prof_bonus=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()=="Speed"{
                speed=val["value"].as_i64().unwrap();
            } else if val["name"].as_str()=="Hit Points"{
                hit_points=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()=="Armor Class"{
                ac=val["total"].as_i64().unwrap();
            }
            idx +=1;
        }
        Character{
            char_name,
            classes,
            background: String::new(),
            damage_mults: vec![],
            race: String::new(),
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
            attacks: vec![],
            equipped: vec![],
            traits,
            features: vec![],
            other_profs: vec![],
            carried: vec![],
            coins: (0,0,0,0,0),
            spell_lists: vec![],
            spell_slots: (0,0,0,0,0,0,0,0,0)
        }
    }
}