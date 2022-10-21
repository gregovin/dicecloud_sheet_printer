
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
struct Skill<'a>{
    base_score: &'a AbilityScore,
    name: String,
    prof_rank: Profficiency
}
impl Skill<'_>{
    pub fn get_prof(&self)->&Profficiency{
        &self.prof_rank
    }
    pub fn get_mod(&self,prof_bonus: i64)->i64{
        self.base_score.get_modifier()+match self.get_prof(){
            Profficiency::None => 0,
            Profficiency::Half => prof_bonus/2,
            Profficiency::Profficient => prof_bonus,
            Profficiency::Expert => 2*prof_bonus,
        }
    }
    pub fn new<'a>(name: String, base_score: &'a AbilityScore, prof_rank: Profficiency)->Skill<'a>{
        Skill {base_score, name, prof_rank}
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
type SpellList = Vec<SpellLevel>;