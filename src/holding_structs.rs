use serde_json::Value;
use std::cmp::{PartialOrd,Ordering,Ord};
use std::collections::HashMap;
use genpdf::{RenderResult,Element,Context,render::Area,style::Style,error::Error,Mm,Size};
use owned_chars::OwnedChars;
use std::fmt::{self,Write};
///defines an ability score by the value(score) and name
#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbilityScore{
    score: i64,
    name: String,
}
impl AbilityScore{
    pub fn score(&self)->i64{
        self.score
    }
    pub fn name(&self)->&String{
        &self.name
    }
    /// get an ability score's modifier
    /// #example
    /// ```
    /// use dicecloud_sheet_printer::holding_structs::{AbilityScore};
    ///
    /// let sten = AbilityScore::new("Strength".to_string(),14);
    /// let con = AbilityScore::new("Constitution".to_string(),11);
    /// let dex = AbilityScore::new("Dexterity".to_string(),9);
    /// assert_eq!(sten.modifier(),2);
    /// assert_eq!(con.modifier(),0);
    /// assert_eq!(dex.modifier(),-1);
    /// ```
    pub fn modifier(&self)->i64{
        self.score/2 -5
    }
    pub fn new(name: String, score: i64)->AbilityScore{
        AbilityScore { score, name}
    }
}
///Types of proficiency listed
#[derive(Debug, Eq, PartialEq,PartialOrd,Ord,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Proficiency {
    #[default]
    None,
    Half,
    Profficient,
    Expert,
}
///A skill is a bonus, name, and prof
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Skill{
    bonus: i64,
    name: String,
    prof_rank: Proficiency
}
impl Skill{
    pub fn prof(&self)->&Proficiency{
        &self.prof_rank
    }
    pub fn modifier(&self)->i64{
        self.bonus
    }
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn new(name: String, bonus: i64, prof_rank: Proficiency)->Skill{
        Skill {bonus, name, prof_rank}
    }
}
impl PartialOrd for Skill{
    fn partial_cmp(&self, other: &Skill)->Option<Ordering>{
        if self.name()!=other.name(){
            return self.name().partial_cmp(other.name());
        }
        if self.prof()!=other.prof() {
            return self.prof().partial_cmp(other.prof());
        }
        self.modifier().partial_cmp(&other.modifier())
    }
}
impl Ord for Skill{
    fn cmp(&self, other: &Skill)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
///A class is a name and a level
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class{
    name: String,
    level: i64,
    pub start_class: bool,
}
impl Class{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn level(&self)->i64{
        self.level
    }
    pub fn new(name: String, level: i64)->Class{
        Class { name, level, start_class: false}
    }
}
impl PartialOrd for Class{
    fn partial_cmp(&self, other: &Class) -> Option<Ordering> {
        if self.start_class != other.start_class {
            if self.start_class && !other.start_class {Some(Ordering::Less)} else {Some(Ordering::Greater)}
        } else if self.level!=other.level(){
            other.level().partial_cmp(&self.level)
        } else {
            self.name.partial_cmp(other.name())
        }
    }
}
impl Ord for Class{
    fn cmp(&self, other: &Class) ->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Feature{
    name: String,
    description: String,
}
impl Feature{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn description(&self)->&String{
        &self.description
    }
    pub fn new(name: String, description: String)->Feature{
        Feature { name, description}
    }
}
///a background is a name and a description
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Background{
    name: String,
    background_feature: Feature,
}
impl Background{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn background_feature(&self)->&Feature{
        &self.background_feature
    }
    pub fn set_background_feature(&mut self,feat: Feature){
        self.background_feature=feat;
    }
    pub fn new(name: String)-> Background{
        Background { name, background_feature: Feature::default() }
    }
}
///a dice has a size, and we include the number
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Die{
    size: i64,
    num: i64,
}
impl Die{
    pub fn size(&self)->i64{
        self.size
    }
    pub fn num(&self)->i64{
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
#[derive(Debug, Eq, PartialEq,Clone,PartialOrd,Ord,Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
impl Default for AtkBonus{
    fn default()->Self{AtkBonus::Bonus(0)}
}
///an attack is a string, AtkBonus, and damage
#[derive(Debug, Eq, PartialEq, Clone,PartialOrd,Ord,Default,Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attack{
    name: String,
    bonus: AtkBonus,
    damage: String
}
impl Attack{
    /// returns the name of the attack
    pub fn name(&self)->&String{
        &self.name
    }
    /// returns the attack's bonus
    pub fn bonus(&self)->&AtkBonus{
        &self.bonus
    }
    /// returns the bonus as a string
    pub fn bonus_as_string(&self)->String{
        self.bonus.to_string()
    }
    /// returns the damage
    pub fn damage(&self)->&String{
        &self.damage
    }
    pub fn new(name: String,bonus: AtkBonus,damage: String)->Attack{
        Attack { name , bonus, damage }
    }
    ///adds damage to the attack
    /// #Example
    /// ```
    /// use dicecloud_sheet_printer::holding_structs::{Attack, AtkBonus};
    /// let mut atk1 = Attack::new("test".to_string(),AtkBonus::Bonus(0),"1d8+3 [fire]".to_string());
    /// let mut atk2 = Attack::new("test".to_string(),AtkBonus::Bonus(0),String::new());
    /// atk1.add_dmg("4 [pir]".to_string());
    /// atk2.add_dmg("4 [pir]".to_string());
    ///
    /// assert_eq!(atk1.damage(),"1d8+3 [fire] 4 [pir]");
    /// assert_eq!(atk2.damage(),"4 [pir]");
    /// ```
    pub fn add_dmg(&mut self, dmg: String){
        if !self.damage.is_empty(){
            let _= write!(self.damage," {}",dmg);
        }
        else{
            self.damage=dmg;
        }
    }
}
///an item has a quantity and a name
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Item{
    quantity: i64,
    name: String,
    plural_name: String,
    requires_attunement: bool,
}
impl Item{
    pub fn quantity(&self)->i64{
        self.quantity
    }
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn plural_name(&self)->&String{
        &self.plural_name
    }
    pub fn requires_attunement(&self)->bool{
        self.requires_attunement
    }
    pub fn new(quantity: i64,name: String,plural_name: String)->Item{
        Item { quantity, name,plural_name,requires_attunement: false}
    }
    pub fn needs_attuned(&mut self){
        self.requires_attunement=true;
    }
}
impl PartialOrd for Item{
    fn partial_cmp(&self,other: &Item)->Option<Ordering>{
        if &self.name != other.name(){
            return self.name.partial_cmp(other.name());
        }
        self.quantity.partial_cmp(&other.quantity())
    }
}
impl Ord for Item{
    fn cmp(&self,other: &Item)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
impl fmt::Display for Item{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let atn = if self.requires_attunement{"❂ "} else {""};
        let nme = if self.quantity==1 {&self.name} else {&self.plural_name};
        write!(f,"{}{} {}",atn,self.quantity,nme)
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord,Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpellPrep{
    AlwaysPrepared,
    Prepared,
    #[default]
    NotPrepared,
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Spell{
    name: String,
    level: i64,
    casting_time: ActionType,
    duration: String,
    school: String,
    range: String,
    vscr: (bool,bool,bool,bool),
    material: String,
    prepd: SpellPrep
}
impl Spell{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn level(&self)->i64{
        self.level
    }
    pub fn casting_time(&self)->&ActionType{
        &self.casting_time
    }
    pub fn duration(&self)->&String{
        &self.duration
    }
    pub fn school(&self)->&String{
        &self.school
    }
    pub fn range(&self)->&String{
        &self.range
    }
    pub fn vscr(&self)->(bool,bool,bool,bool){
        self.vscr
    }
    pub fn vscr_to_string(&self)->String{
        let v = if self.vscr.0{"v"} else {""};
        let s = if self.vscr.1{"s"} else {""};
        let c = if self.vscr.2{"c"} else {""};
        let r = if self.vscr.3{"r"} else {""};
        format!("{}{}{}{}",v,s,c,r)
    }
    pub fn material(&self)->&String{
        &self.material
    }
    pub fn prepd(&self)->SpellPrep{
        self.prepd
    }
    pub fn new(name: String, level: i64, casting_time: ActionType, duration: String, school: String, range: String, vscr: (bool,bool,bool,bool),material: String)->Spell{
        Spell{name,level,casting_time,duration,school,range,vscr,material,prepd:SpellPrep::NotPrepared}
    }
    pub fn prepare(&mut self){
        self.prepd = SpellPrep::Prepared;
    }
    pub fn always_prepare(&mut self){
        self.prepd = SpellPrep::AlwaysPrepared;
    }
    pub fn unprepare(&mut self){
        self.prepd = SpellPrep::NotPrepared;
    }
}
///We store all spells of the same level in the same SpellLevel struct
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpellLevel{
    level: i64,
    spells: Vec<Spell>
}
impl SpellLevel{
    pub fn lvl(&self)->i64{
        self.level
    }
    pub fn spells(&self)->&Vec<Spell>{
        &self.spells
    }
    pub fn new(level: i64, spells: Vec<Spell>)->SpellLevel{
        SpellLevel { level, spells }
    }
    pub fn add_spell(&mut self,spell: Spell){
        self.spells.push(spell);
    }
}
///a spell list has spells of several levels, but with a casting class, ability, save dc, and attack bonus
#[derive(Debug, Eq, PartialEq,Clone,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpellList{
    pub levels: HashMap<i64,SpellLevel>,
    pub name: String,
    pub save_dc: i64,
    pub atk_bonus: i64,
    pub max_prepared: i64
}
impl SpellList{
    pub fn new(levels: HashMap<i64,SpellLevel>,name: String,save_dc: i64, atk_bonus: i64, max_prepared: i64)->SpellList{
        SpellList{levels, name, save_dc, atk_bonus,max_prepared}
    }
    pub fn max_lvl(&self)->i64{
        self.levels.iter().fold(0,|mx,val| if val.1.lvl()>mx {val.1.lvl()} else {mx})
    }
}
impl PartialOrd for SpellList{
    fn partial_cmp(&self,other: &SpellList)->Option<Ordering>{
        let own_lvl = self.max_lvl();
        let other_lvl = other.max_lvl();
        if self.max_prepared != other.max_prepared{
            other.max_prepared.partial_cmp(&self.max_prepared)
        } else if own_lvl != other_lvl{
            other_lvl.partial_cmp(&own_lvl)
        } else if self.name != other.name {
            self.name.partial_cmp(&other.name)
        } else if self.save_dc != other.save_dc{
            self.save_dc.partial_cmp(&other.save_dc)
        } else {
            self.atk_bonus.partial_cmp(&other.atk_bonus)
        }
    }
}
impl Ord for SpellList{
    fn cmp(&self,other: &SpellList)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
///a damage multiplier has Immunity, Resistence, Vulnerability, each with a string damage type
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DamageMult{
    Immune(String),
    Resist(String),
    Vuln(String),
}
impl fmt::Display for DamageMult{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (typ, str) = match self{
            DamageMult::Immune(s)=> (s, "immunity"),
            DamageMult::Resist(s)=> (s, "resistance"),
            DamageMult::Vuln(s)=> (s, "vulnerability")
        };
        write!(f,"{} {}",typ,str)
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ActionType{
    Free,
    Reaction,
    Bonus,
    #[default]
    Action,
    Long(String)
}
impl fmt::Display for ActionType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self{
            ActionType::Free => "fr.".to_string(),
            ActionType::Reaction => "rxn".to_string(),
            ActionType::Bonus => "bns".to_string(),
            ActionType::Action =>"a".to_string(),
            ActionType::Long(time) =>time.to_string(),
        };
        write!(f, "{}", &out)
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Action{
    name: String,
    typ: ActionType,
    uses: i64 //-1=infty
}
impl Action{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn uses(&self)->i64{
        self.uses
    }
    pub fn typ(&self)->&ActionType{
        &self.typ
    }
    pub fn new(name: String,uses: i64,typ: ActionType)->Action{
        Action{name, uses,typ}
    }
}
impl fmt::Display for Action{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nl = format!("{}",self.uses).len();
        let resources = if self.uses==-1{
            String::new()
        } else {
            let blank = String::from_utf8(vec![b'_'; nl]).expect("never fails");
            format!("({}/{})",blank,self.uses)
        };
        write!(f,"({}) {}{}",self.typ,self.name,resources)
    }
}
impl Default for Action{
    fn default()->Action{
        Action{name:String::default(),typ: ActionType::default(), uses:-1}
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resource {
    name: String,
    total: i64,
}
impl Resource{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn total(&self)->i64{
        self.total
    }
    pub fn new(name: String, total: i64)->Resource{
        Resource{name, total}
    }
}
impl fmt::Display for Resource{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        let nl = format!("{}",self.total).len();
        let resources = if self.total==-1{
            String::new()
        } else {
            let blank = String::from_utf8(vec![b'_'; nl]).expect("never fails");
            format!("({}/{})",blank,self.total)
        };
        write!(f,"{} {}",self.name,resources)
    }
}
///a struct for parsing the character into
#[derive(Debug, Eq, PartialEq,Clone,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub actions: Vec<Action>,
    pub resources: Vec<Resource>,
    pub equipment: Vec<Item>,
    pub traits: (String, String, String, String),//Personality, Ideals, Bonds, Flaws
    pub features: Vec<String>,
    pub other_profs: (Vec<String>,Vec<String>,Vec<String>,Vec<String>),//armor,weapon,language, tool
    pub coins: (i64,i64,i64,i64,i64),//cp,sp,ep,gp,pp
    pub spell_lists: Vec<SpellList>,
    pub spell_slots: [i64;9],//1st,2nd,...9th
    pub char_img: String,
}

impl Character{
    /// #Panics
    /// when properties do not follow the expected structure, (ie a core stat can't be found, or a property does not have an expected entry), the function will panic
    pub fn new(mut char_json: Value,race_decoder: Value)->Character{
        let char_name=&char_json["creatures"][0]["name"];
        if char_name == &Value::Null{
            panic!("cannot find char name, probably because the api is wrong");
        }
        let char_name = char_name.as_str().unwrap().to_string();
        let alignment=char_json["creatures"][0]["alignment"].as_str().unwrap_or("").to_string();
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
        let mut actions: Vec<Action>=vec![];
        let mut classes: Vec<Class> = vec![];
        let mut features: Vec<String> = vec![];
        let mut starting_class = String::new();
        let mut resources: Vec<Resource> = vec![];
        let mut equipment: Vec<Item> = vec![];
        let mut hit_dice: Vec<Die> = vec![];
        let mut background: Background=Background::default();
        let mut race: String = String::new();
        let mut coins = (0,0,0,0,0);
        let mut other_profs: (Vec<String>,Vec<String>,Vec<String>,Vec<String>) = (vec![],vec![],vec![],vec![]);
        let mut char_img = String::new();
        let mut spell_ls_dict: HashMap<String,SpellList> =HashMap::new();
        let mut spell_slots: [i64;9] = [0;9];
        if let Some(url) =char_json["creatures"][0]["avatarPicture"].as_str(){
            char_img = url.to_string();
        } else if let Some(url) = char_json["creatures"][0]["picture"].as_str(){
            char_img = url.to_string();
        }
        let props = char_json["creatureProperties"].as_array_mut().unwrap();
        props.sort_by(|a,b| a["order"].as_i64().unwrap().cmp(&b["order"].as_i64().unwrap()));
        for val in props{
            if val["removed"].as_bool()==Some(true){
                continue;
            }
            if val["type"].as_str()==Some("attribute") && val["attributeType"].as_str()==Some("ability"){
                ability_scores.push(AbilityScore::new(val["name"].as_str().unwrap().to_string(),
                    val["total"].as_i64().unwrap()));
            } else if val["type"].as_str()==Some("skill"){
                if val["name"].as_str()==Some("Initiative"){
                    initiative=val["value"].as_i64().unwrap();
                } else if val["skillType"].as_str()==Some("save"){
                    let prf=val["proficiency"].as_f64();
                    let prof = if prf ==Some(0.49) || prf==Some(0.5){
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
                    let prof = if prf.is_some() && prf.unwrap() >= 0.48 && prf.unwrap() <= 0.52{
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
            } else if val["type"].as_str()==Some("feature") && 
                val["tags"].as_array().unwrap().iter().any(|tag| tag.as_str().unwrap().contains("background")){
                    background.set_background_feature(Feature::new(val["name"].as_str().unwrap().to_string(),
                    val["summary"]["value"].as_str().unwrap().to_string()));
            }else if val["type"].as_str()==Some("feature"){
                features.push(val["name"].as_str().unwrap().to_string());
            }else if val["type"].as_str()==Some("spellList"){
                let id = val["_id"].as_str().unwrap();
                let max_prepared = val["maxPrepared"]["value"].as_i64().unwrap_or(0);
                let dc = val["dc"]["value"].as_i64().unwrap_or(10);
                let attack_bonus = val["attackRollBonus"]["value"].as_i64().unwrap_or(0);
                let name = val["name"].as_str().unwrap();
                spell_ls_dict.entry(id.to_string())
                    .or_insert_with(|| SpellList::new(HashMap::new(),name.to_string(),dc,attack_bonus,max_prepared));
            }else if val["type"].as_str()==Some("spell")&&val["deactivatedByToggle"].as_bool()!=Some(true){
                let ancestors = val["ancestors"].as_array().unwrap();
                let mut spl_list_id: String = String::new();
                for anc in ancestors.iter().rev(){
                    if spell_ls_dict.contains_key(anc["id"].as_str().unwrap_or("")){
                        spl_list_id=anc["id"].as_str().unwrap().to_string();
                        break;
                    }
                }
                // assume this always works
                let name = val["name"].as_str().unwrap_or("").to_string();
                let lvl = val["level"].as_i64().unwrap_or(0);
                let casting_time = val["actionType"].as_str().unwrap_or("").to_string();
                let duration = val["duration"].as_str().unwrap_or("").to_string();
                let school = val["school"].as_str().unwrap_or("").to_string();
                let range = val["range"].as_str().unwrap_or("").to_string();
                let vscr =(val["verbal"].as_bool()==Some(true),val["somatic"].as_bool()==Some(true),
                    val["concentration"].as_bool()==Some(true),val["ritual"].as_bool()==Some(true));
                let material = match val["material"].as_str(){
                    Some(s)=>s.to_string(),
                    None=>String::new()
                };
                let casting_time = if &casting_time=="action"{
                    ActionType::Action
                } else if &casting_time=="bonus"{
                    ActionType::Bonus
                } else if casting_time.contains("reaction"){
                    ActionType::Reaction
                } else if &casting_time=="free"{
                    ActionType::Free
                } else {
                    ActionType::Long(val["castingTime"].as_str().unwrap_or("long").replace("round","rnd").replace("minute","min").replace("hour","hr"))
                };
                let duration = duration.to_lowercase().replace("up to ","").replace("round","rnd").replace("minute","min").replace("hour","hr");
                let range = range.replace("feet","ft").replace("miles","mi").replace("mile","mi").replace("slotLevel","sl")
                    .replace("foot","ft").replace("radius","rad").replace(" * (1 + spellSniper)","");
                let mut spl = Spell::new(name,lvl,casting_time,duration, school, range, vscr, material);
                if val["alwaysPrepared"].as_bool() == Some(true){
                    spl.always_prepare();
                } else if val["prepared"].as_bool() == Some(true){
                    spl.prepare();
                }
                let _=spell_ls_dict.entry(spl_list_id.to_string()).and_modify(|ls|{
                    ls.levels.entry(lvl).and_modify(|splvl| {splvl.add_spell(spl.clone());}).or_insert_with(|| SpellLevel::new(lvl,vec![spl]));}
                );
            }else if val["type"].as_str()==Some("damageMultiplier") && val["inactive"].as_bool()!=Some(true){
                let mult = val["value"].as_f64().unwrap();
                if mult == 0.0 {
                    for typ in val["damageTypes"].as_array().unwrap_or(&vec![]){
                        damage_mults.push(DamageMult::Immune(typ.as_str().unwrap().to_string()));
                    }
                } else if mult == 0.5 {
                    for typ in val["damageTypes"].as_array().unwrap_or(&vec![]){
                        damage_mults.push(DamageMult::Resist(typ.as_str().unwrap().to_string()));
                    }
                } else if mult == 2.0 {
                    for typ in val["damageTypes"].as_array().unwrap_or(&vec![]){
                        damage_mults.push(DamageMult::Vuln(typ.as_str().unwrap().to_string()));
                    }
                }
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
            }else if val["type"].as_str()==Some("action"){
                if val["actionType"].as_str()==Some("attack") && val["inactive"].as_bool()!=Some(true){
                    let bns = AtkBonus::Bonus(val["attackRoll"]["value"].as_i64().unwrap_or(0));
                    let id = val["_id"].as_str().unwrap().to_string();
                    let dmg = match attacks_dict.get(&id){
                        Some(atk)=>atk.damage(),
                        None=>""
                    };
                    attacks_dict.insert(id,Attack::new(val["name"].as_str().unwrap().to_string(),bns,dmg.to_string()));
                } else if val["inactive"].as_bool()!=Some(true){
                    let typ = val["actionType"].as_str();
                    let name = val["name"].as_str().unwrap().to_string();
                    let mut uses = -1;
                    if let Some(k)=val["uses"]["value"].as_i64(){
                        uses=k;
                    }
                    let typ = if typ==Some("free"){
                        ActionType::Free
                    } else if typ==Some("bonus"){
                        ActionType::Bonus
                    } else if typ==Some("reaction"){
                        ActionType::Reaction
                    } else if typ==Some("action"){
                        ActionType::Action
                    } else if typ==Some("event"){
                        continue
                    }else if typ==Some("long"){
                        ActionType::Long("lng".to_string())
                    } else{
                        ActionType::default()
                    };
                    actions.push(Action{name,uses,typ});
                }
            }else if val["type"].as_str()==Some("damage"){
                let par_id = val["parent"]["id"].as_str().unwrap().to_string();
                let dmg_die = val["amount"]["calculation"].as_str().unwrap_or("0d0");
                let dmg_bonus = val["amount"]["effects"][0]["amount"]["value"].as_i64().unwrap_or(0);
                let dmg_type = val["damageType"].as_str().unwrap().to_string();
                let dmg_string = format!("{}{}{}[{}]",dmg_die,if dmg_bonus>=0 {"+"} else {""},
                dmg_bonus,damage_type_abreviator(dmg_type));
                if let Some(atk)=attacks_dict.get_mut(&par_id){
                    atk.add_dmg(dmg_string);
                }
            }else if val["type"].as_str()==Some("class"){
                classes.push(Class::new(val["name"].as_str().unwrap().to_string(),
                    val["level"].as_i64().unwrap()));
            }else if val["type"].as_str()==Some("item"){
                if val["name"].as_str().unwrap().to_lowercase().contains("piece"){
                    let coin_type=val["name"].as_str().unwrap().to_string().to_lowercase();
                    if coin_type.contains("platinum"){
                        coins.4 = val["quantity"].as_i64().unwrap();
                    } else if coin_type.contains("gold"){
                        coins.3 = val["quantity"].as_i64().unwrap();
                    } else if coin_type.contains("electrum"){
                        coins.2 = val["quantity"].as_i64().unwrap();
                    }else if coin_type.contains("silver"){
                        coins.1 = val["quantity"].as_i64().unwrap();
                    } else if coin_type.contains("copper"){
                        coins.0 = val["quantity"].as_i64().unwrap();
                    }
                }else{
                    let nme = val["name"].as_str().unwrap().to_string();
                    let mut itme = Item::new(val["quantity"].as_i64().unwrap_or(0),
                        val["name"].as_str().unwrap().to_string(),
                        val["plural"].as_str().unwrap_or(&nme).to_string());
                    if val["requiresAttunement"].as_bool()==Some(true){
                        itme.needs_attuned();
                    }
                    equipment.push(itme);
                }
            }else if val["type"].as_str()==Some("attribute") && val["attributeType"].as_str()==Some("spellSlot") {
                if val["inactive"].as_bool() !=Some(true){
                    let lvl = val["spellSlotLevel"]["value"].as_i64().unwrap();
                    let num = val["value"].as_i64().unwrap();
                    spell_slots[(lvl-1) as usize]+=num;
                }
            }else if val["type"].as_str()==Some("attribute") && val["attributeType"].as_str()==Some("resource") {
                if val["inactive"].as_bool() !=Some(true){
                    resources.push(Resource::new(val["name"].as_str().unwrap().to_string(),val["total"].as_i64().unwrap_or(0)));
                }
            }else if val["name"].as_str()==Some("Proficiency Bonus"){
                prof_bonus=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()==Some("Speed") && val["type"].as_str()==Some("attribute"){
                speed=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()==Some("Hit Points") && val["type"].as_str()==Some("attribute"){
                hit_points=val["total"].as_i64().unwrap();
            } else if val["name"].as_str()==Some("Armor Class") && val["type"].as_str()==Some("attribute"){
                ac=val["total"].as_i64().unwrap();
            } else if val["tags"].as_array().unwrap().iter().any(|tag| tag.as_str()==Some("background")){
                background=Background::new(val["name"].as_str().unwrap().to_string());
            } else if val["type"].as_str()==Some("constant"){
                if val["variableName"].as_str()==Some("race"){
                    if race==String::new(){
                        race = val["calculation"].as_str().unwrap().to_string().replace('\"',"");
                    }
                } else if val["type"].as_str()==Some("constant") && val["variableName"].as_str()==Some("subRace"){
                    race = val["calculation"].as_str().unwrap().to_string().replace('\"',"");
                } else if val["variableName"].as_str()==Some("startingClass"){
                    starting_class = val["calculation"].as_str().unwrap().trim().to_string();
                }
            } else if val["tags"].as_array().unwrap().iter().any(|tag| tag.as_str()==Some("race")){
                if race==String::new(){
                    race = val["name"].as_str().unwrap().to_string();
                }
            } else if val["tags"].as_array().unwrap().iter().any(|tag| tag.as_str()==Some("subrace")){
                let subrace = val["name"].as_str().unwrap().to_string();
                race = if subrace.contains(&race){subrace} else {subrace +&race};
            }
        }
        for class in classes.iter_mut(){
            if class.name().to_lowercase() == starting_class.replace('\"',"").replace('\'',"").to_lowercase(){
                class.start_class=true;
                break;
            }
        }
        let race = race_translator(race,race_decoder);
        for pair in attacks_dict.into_iter(){
            if !pair.1.name().is_empty(){
                attacks.push(pair.1);
            }
        }
        let mut spell_lists: Vec<SpellList>= vec![];
        for pair in spell_ls_dict.into_iter(){
            if !pair.1.name.is_empty(){
                spell_lists.push(pair.1);
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
            actions,
            resources,
            equipment,
            traits,
            features,
            other_profs,
            coins,
            spell_lists,
            spell_slots,
            char_img
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
#[derive(Default)]
pub struct Hline{

}
impl Hline{
    pub fn new()->Hline{
        Hline{}
    }
}
impl Element for Hline{
    fn render(
        &mut self, 
        _context: &Context, 
        area: Area<'_>, 
        style: Style
    ) -> Result<RenderResult, Error>{
        let sz=area.size();
        let zero = Mm::from(0);
        let small = Mm::from(0.05);
        let p1= genpdf::Position{x:zero,y:small};
        let p2= genpdf::Position{x:sz.width,y:small};
        area.draw_line(vec![p1,p2],style);
        let ataken = if Mm::from(0.1)<sz.height{
            Mm::from(0.1)
        } else{
            sz.height
        };
        Ok(RenderResult{size:Size{width:sz.width,height:ataken},has_more:false})
    }
}