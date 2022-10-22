
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

pub struct Character{
    pub char_name: String,
    pub classes: Vec<Class>,
    pub background: String,
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
    pub fn new()->Character{
        Character{
            char_name: String::new(),
            classes: vec![],
            background: String::new(),
            race: String::new(),
            alignment: String::new(),
            xp: 0,
            ability_scores: vec![],
            prof_bonus: 0,
            saving_throws: vec![],
            skills: vec![],
            ac: 0,
            initiative: 0,
            speed: 0,
            hit_points: 0,
            attacks: vec![],
            equipped: vec![],
            traits: (String::new(),String::new(),String::new(),String::new()),
            features: vec![],
            other_profs: vec![],
            carried: vec![],
            coins: (0,0,0,0,0),
            spell_lists: vec![],
            spell_slots: (0,0,0,0,0,0,0,0,0)
        }
    }
}