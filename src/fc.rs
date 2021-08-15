use std::ptr;

use crate::state::State;
use crate::artifact::Artifact;
use crate::action::{Attack, AttackEvent, ICDTimer, NTimer, ICDTimers};
use crate::types::{Vision, WeaponType, FieldEnergy, ElementalGauge, ElementalReaction, ElementalReactionType};


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldCharacterIndex(pub usize);

#[allow(unused_variables)]
pub trait SpecialAbility {
    // the variable is named `owner_fc` because `FieldCharacter` will own this
    // `SpecialAbility`.

    fn init(&mut self, timers: &mut ICDTimers) -> () {}

    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> { None }

    // fn build(&mut self, builder: &mut FieldAbilityBuilder) -> ();

    // Synchronize own timers to the emulator so that cool down times and
    // passive effect duration are up to date. The three methods of
    // `additional_attack`, `modify` and `accelerate` will depend on the data
    // mutated by this self.
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {}

    // `Vec::push` additional attacks created. The `atk_queue` is `Vec` because
    // some character (Eula's hold skill) deals many additional attacks of
    // different elements at once. Similarly, Kazuha (Chihayaburu) deals
    // additional attacks of different kinds.
    // `AdditionalAttack` created by various effects of characters and weapons
    // are another entities who take part in the battle. These entities
    // can also attack an enemy, so we need to know how strong their attacks
    // are, i.e. additional attack DMG (or `Attack.multiplier`).
    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {}

    // Apply own passive effects to `State`. This is the primary method that
    // "returns" passive effects to `FieldCharacter`. For example, suppose this
    // `SpecialAbility` is about Sucrose A1, "Catalyst Conversion" and she has
    // `FieldCharacterIndex(1)`. Then this method modifies the vector like:
    // 
    //     modifiable_state[0, 2 and 3].em += 50
    // 
    // The `Enemy` is mutable because some passive debuffs an enemy (e.g. DEF
    // down by Lisa).
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {}

    // This method can change the `State` of `Attack`. For example, Some abilities
    // increase CR of a specific action: Amber A1 (Every Arrow Finds Its
    // Target), Ganyu A1 (Undivided Heart), Festering Desire.
    fn intensify(&self, attack: &Attack) -> Option<State> { None }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> { None }

    fn reset(&mut self) -> () {}
}

pub trait SkillAbility : SpecialAbility {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> ();
}

// #[derive(Debug)]
pub struct FieldAbility<'a> {
    pub timers: &'a mut ICDTimers,
    pub na: &'a mut dyn SpecialAbility,
    pub ca: &'a mut dyn SpecialAbility,
    pub skill: &'a mut dyn SkillAbility,
    pub burst: &'a mut dyn SpecialAbility,
    pub passive: &'a mut dyn SpecialAbility,
    pub weapon: &'a mut dyn SpecialAbility,
    pub artifact: &'a mut dyn SpecialAbility,
}

impl<'a> FieldAbility<'a> {
    pub fn init(self) -> Self {
        self.na.init(self.timers);
        self.ca.init(self.timers);
        self.skill.init(self.timers);
        self.burst.init(self.timers);
        self.passive.init(self.timers);
        self
    }

    // pub fn init_timer(&mut self, timers: &'a mut ICDTimers) -> () {
    //     self.na.init(timers);
    //     self.ca.init(timers);
    //     self.skill.init(timers);
    //     self.burst.init(timers);
    //     self.passive.init(timers);
    //     self.timers = timers;
    // }

    pub fn init_timer(&mut self) -> () {
        self.na.init(self.timers);
        self.ca.init(self.timers);
        self.skill.init(self.timers);
        self.burst.init(self.timers);
        self.passive.init(self.timers);
        // unsafe {
        //     let x = ptr::null_mut();
        //     *x = 42;
        // }
    }

    pub fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.na.additional_attack(atk_queue, particles, data);
        self.ca.additional_attack(atk_queue, particles, data);
        self.skill.additional_attack(atk_queue, particles, data);
        self.burst.additional_attack(atk_queue, particles, data);
        self.passive.additional_attack(atk_queue, particles, data);
        self.weapon.additional_attack(atk_queue, particles, data);
    }

    pub fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.passive.modify(modifiable_state, data, enemy);
        self.weapon.modify(modifiable_state, data, enemy);
        self.artifact.modify(modifiable_state, data, enemy);
    }

    pub fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.timers.update(time);
        self.na.update(time * (1.0 + data.state().atk_spd / 100.0), event, data, attack, particles, enemy);
        self.ca.update(time, event, data, attack, particles, enemy);
        self.skill.update(time, event, data, attack, particles, enemy);
        self.burst.update(time, event, data, attack, particles, enemy);
        self.passive.update(time, event, data, attack, particles, enemy);
        self.weapon.update(time, event, data, attack, particles, enemy);
        self.artifact.update(time, event, data, attack, particles, enemy);
    }

    pub fn accelerate(&mut self) -> () {
        if let Some(f) = self.passive.accelerator() {
            self.skill.accelerate(f);
        }
        if let Some(f) = self.weapon.accelerator() {
            self.skill.accelerate(f);
        }
    }

    pub fn intensify(&self, attack: &Attack) -> Option<State> {
        let mut result: Option<State> = None;
        let xs = &mut [
            self.passive.intensify(attack),
            self.weapon.intensify(attack),
        ];
        for some_state in xs.iter_mut() {
            match (&mut result, some_state) {
                (Some(state), Some(s)) => state.merge(&s),
                (Some(_), None) => (),
                (None, state @ Some(_)) => result = state.take(),
                (None, None) => (),
            };
        }
        result
    }

    pub fn reset(&mut self) -> () {
        self.na.reset();
        self.ca.reset();
        self.skill.reset();
        self.burst.reset();
        self.passive.reset();
        self.weapon.reset();
        self.artifact.reset();
    }
}

pub struct FieldAbilityBuilder {
    na: Option<*mut dyn SpecialAbility>,
    ca: Option<*mut dyn SpecialAbility>,
    skill: Option<*mut dyn SkillAbility>,
    burst: Option<*mut dyn SpecialAbility>,
    passive: Option<*mut dyn SpecialAbility>,
    weapon: Option<*mut dyn SpecialAbility>,
    artifact: Option<*mut dyn SpecialAbility>,

    na_noop: Option<NoopAbility>,
    ca_noop: Option<NoopAbility>,
    skill_noop: Option<NoopSkillAbility>,
    burst_noop: Option<NoopAbility>,
    passive_noop: Option<NoopAbility>,
    weapon_noop: Option<NoopAbility>,
    artifact_noop: Option<NoopAbility>,
}

impl FieldAbilityBuilder {
    pub fn new() -> Self {
        Self {
            na: None,
            ca: None,
            skill: None,
            burst: None,
            passive: None,
            weapon: None,
            artifact: None,
            na_noop: None,
            ca_noop: None,
            skill_noop: None,
            burst_noop: None,
            passive_noop: None,
            weapon_noop: None,
            artifact_noop: None,
        }
    }

    pub fn na(&mut self, na: *mut dyn SpecialAbility) -> &mut Self {
        self.na = Some(na);
        self
    }

    pub fn ca(&mut self, ca: *mut dyn SpecialAbility) -> &mut Self {
        self.ca = Some(ca);
        self
    }

    pub fn skill(&mut self, skill: *mut dyn SkillAbility) -> &mut Self {
        self.skill = Some(skill);
        self
    }

    pub fn burst(&mut self, burst: *mut dyn SpecialAbility) -> &mut Self {
        self.burst = Some(burst);
        self
    }

    pub fn passive(&mut self, passive: *mut dyn SpecialAbility) -> &mut Self {
        self.passive = Some(passive);
        self
    }

    pub fn weapon(&mut self, weapon: *mut dyn SpecialAbility) -> &mut Self {
        self.weapon = Some(weapon);
        self
    }

    pub fn artifact(&mut self, artifact: *mut dyn SpecialAbility) -> &mut Self {
        self.artifact = Some(artifact);
        self
    }

    pub fn build<'a>(&'a mut self, timers: &'a mut ICDTimers) -> FieldAbility<'a> {
        let na = match self.na.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.na_noop.insert(NoopAbility),
        };
        let ca = match self.ca.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.ca_noop.insert(NoopAbility),
        };
        // let skill = unsafe { &mut *self.skill.take().unwrap() };
        let skill = match self.skill.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.skill_noop.insert(NoopSkillAbility),
        };
        let burst = match self.burst.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.burst_noop.insert(NoopAbility),
        };
        let passive = match self.passive.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.passive_noop.insert(NoopAbility),
        };
        let weapon = match self.weapon.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.weapon_noop.insert(NoopAbility),
        };
        let artifact = match self.artifact.take() {
            Some(ability) => unsafe { &mut *ability },
            None => self.artifact_noop.insert(NoopAbility),
        };
        (FieldAbility {
            timers,
            na,
            ca,
            skill,
            burst,
            passive,
            weapon,
            artifact,
        }).init()
    }
}

#[derive(Debug)]
pub struct NoopAbility;

impl SpecialAbility for NoopAbility {
    // fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {}
}

#[derive(Debug)]
pub struct NoopSkillAbility;

impl SpecialAbility for NoopSkillAbility {
    // fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {}
}

impl SkillAbility for NoopSkillAbility {
    fn accelerate(&mut self, _f: fn(&mut NTimer)) -> () {}
}

#[derive(Debug)]
pub struct CharacterRecord {
    pub name: &'static str,
    pub vision: Vision,
    pub weapon: WeaponType,
    pub release_date: &'static str,
    pub version: f32,
    pub energy_cost: f32,
    pub state: State,
}

impl Default for CharacterRecord {
    fn default() -> Self {
        Self {
            name: "Amber",
            vision: Vision::Pyro,
            weapon: WeaponType::Bow,
            release_date: "2020-09-28",
            version: 1.0,
            energy_cost: 40.0,
            state: State::new().cr(5.0).cd(50.0)
        }
    }
}

#[allow(dead_code)]
impl CharacterRecord {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name ; self }
    pub fn release_date(mut self, release_date: &'static str) -> Self { self.release_date = release_date ; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version ; self }
    pub fn vision(mut self, vision: Vision) -> Self { self.vision = vision ; self }
    pub fn weapon(mut self, weapon: WeaponType) -> Self { self.weapon = weapon ; self }
    pub fn energy_cost(mut self, energy_cost: f32) -> Self { self.energy_cost = energy_cost ; self }
    pub fn base_hp(mut self, base_hp: f32) -> Self { self.state.base_hp = base_hp ; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.state.base_atk = base_atk ; self }
    pub fn base_def(mut self, base_def: f32) -> Self { self.state.base_def = base_def ; self }
    pub fn hp(mut self, hp: f32) -> Self { self.state.hp = hp ; self }
    pub fn atk(mut self, atk: f32) -> Self { self.state.atk = atk ; self }
    pub fn def(mut self, def: f32) -> Self { self.state.def = def ; self }
    pub fn cr(mut self, cr: f32) -> Self { self.state.cr = cr ; self }
    pub fn cd(mut self, cd: f32) -> Self { self.state.cd = cd ; self }
    pub fn er(mut self, er: f32) -> Self { self.state.er = er ; self }
    pub fn em(mut self, em: f32) -> Self { self.state.em = em ; self }
    pub fn na_dmg(mut self, na_dmg: f32) -> Self { self.state.na_dmg = na_dmg ; self }
    pub fn ca_dmg(mut self, ca_dmg: f32) -> Self { self.state.ca_dmg = ca_dmg ; self }
    pub fn skill_dmg(mut self, skill_dmg: f32) -> Self { self.state.skill_dmg = skill_dmg ; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.state.burst_dmg = burst_dmg ; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.state.physical_dmg = physical_dmg ; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.state.pyro_dmg = pyro_dmg ; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.state.cryo_dmg = cryo_dmg ; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.state.hydro_dmg = hydro_dmg ; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.state.electro_dmg = electro_dmg ; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.state.anemo_dmg = anemo_dmg ; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.state.geo_dmg = geo_dmg ; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.state.dendro_dmg = dendro_dmg ; self }
    pub fn infusion(mut self, infusion: bool) -> Self { self.state.infusion = infusion ; self }
}

#[derive(Debug)]
pub struct WeaponRecord {
    pub name: &'static str,
    pub type_: WeaponType,
    pub version: f32,
    pub state: State,
}

impl Default for WeaponRecord {
    fn default() -> Self {
        Self {
            name: "",
            type_: WeaponType::Sword,
            version: 0.0,
            state: State::new()
        }
    }
}

#[allow(dead_code)]
impl WeaponRecord {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name; self }
    pub fn type_(mut self, type_: WeaponType) -> Self { self.type_ = type_; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.state.base_atk = base_atk; self }
    pub fn hp(mut self, hp: f32) -> Self { self.state.hp = hp; self }
    pub fn atk(mut self, atk: f32) -> Self { self.state.atk = atk; self }
    pub fn def(mut self, def: f32) -> Self { self.state.def = def; self }
    pub fn cr(mut self, cr: f32) -> Self { self.state.cr = cr; self }
    pub fn cd(mut self, cd: f32) -> Self { self.state.cd = cd; self }
    pub fn er(mut self, er: f32) -> Self { self.state.er = er; self }
    pub fn em(mut self, em: f32) -> Self { self.state.em = em; self }
    pub fn atk_spd(mut self, atk_spd: f32) -> Self { self.state.atk_spd = atk_spd; self }
    pub fn na_dmg(mut self, na_dmg: f32) -> Self { self.state.na_dmg = na_dmg; self }
    pub fn ca_dmg(mut self, ca_dmg: f32) -> Self { self.state.ca_dmg = ca_dmg; self }
    pub fn skill_dmg(mut self, skill_dmg: f32) -> Self { self.state.skill_dmg = skill_dmg; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.state.burst_dmg = burst_dmg; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.state.physical_dmg = physical_dmg; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.state.pyro_dmg = pyro_dmg; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.state.cryo_dmg = cryo_dmg; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.state.hydro_dmg = hydro_dmg; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.state.electro_dmg = electro_dmg; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.state.anemo_dmg = anemo_dmg; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.state.geo_dmg = geo_dmg; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.state.dendro_dmg = dendro_dmg; self }
}

#[derive(Debug)]
pub struct Debuff {
    caster: usize,
    amount: f32,
    duration: f32,
}

// https://genshin-impact.fandom.com/wiki/Damage#Resistance
impl Debuff {
    // pub fn new(caster: usize, amount: f32, duration: f32) -> Self {
    //     Self { caster, amount, duration }
    // }

    pub fn superconduct() -> Self {
        Self { caster: 1, amount: 40.0, duration: 12.0 }
    }

    // pub fn frozen() -> Self {
    //     Self { caster: 2, amount: 1.0, duration: 4.0 }
    // }

    pub fn viridescent_venerer() -> Self {
        Self { caster: 3, amount: 40.0, duration: 10.0 }
    }

    pub fn lisa_a4() -> Self {
        Self { caster: 4, amount: 15.0, duration: 10.0 }
    }

    pub fn chongyun_a4() -> Self {
        Self { caster: 5, amount: 10.0, duration: 8.0 }
    }

    pub fn eula_cryo() -> Self {
        Self { caster: 6, amount: 25.0, duration: 7.0 }
    }

    pub fn eula_physical() -> Self {
        Self { caster: 7, amount: 25.0, duration: 7.0 }
    }

    pub fn value(&self) -> f32 {
        if self.duration > 0.0 {
            self.amount
        } else {
            0.0
        }
    }

    pub fn update(&mut self, time: f32) -> () {
        self.duration -= time;
    }

    pub fn accumulate(list: &Vec<Debuff>) -> f32 {
        let mut m: Vec<usize> = Vec::with_capacity(list.len());
        let mut res = 0.0;
        for debuff in list {
            // do not stack the same RES down debuff
            if !m.contains(&debuff.caster) {
                let v = debuff.value();
                res += v;
                if v != 0.0 {
                    m.push(debuff.caster);
                }
            }
        }
        res
    }
}

#[derive(Debug)]
pub struct Enemy {
    pub element_res: f32,
    pub physical_res: f32,
    pub level: f32,

    pub aura: ElementalGauge,
    pub isfrozen: bool,
    pub element_res_debuff: Vec<Debuff>,
    pub physical_res_debuff: Vec<Debuff>,
    pub def_down_debuff: Vec<Debuff>,
}

impl Enemy {
    pub fn hilichurl() -> Self {
        Self {
            element_res: 10.0,
            physical_res: 10.0,
            level: 90.0,
            aura: ElementalGauge::default(),
            isfrozen: false,
            element_res_debuff: Vec::new(),
            physical_res_debuff: Vec::new(),
            def_down_debuff: Vec::new()
        }
    }

    pub fn simple() -> Self {
        Self {
            element_res: 0.0,
            physical_res: 0.0,
            level: 90.0,
            aura: ElementalGauge::default(),
            isfrozen: false,
            element_res_debuff: Vec::new(),
            physical_res_debuff: Vec::new(),
            def_down_debuff: Vec::new()
        }
    }

    pub fn trigger_er(&self, e: &Vision) -> ElementalReactionType {
        ElementalReaction::new(self.aura.aura, *e)
    }

    pub fn get_element_res(&self) -> f32 {
        if self.element_res_debuff.len() == 0 {
            0.0
        } else {
            Debuff::accumulate(&self.element_res_debuff)
        }
    }

    pub fn get_physical_res(&self) -> f32 {
        if self.physical_res_debuff.len() == 0 {
            0.0
        } else {
            Debuff::accumulate(&self.physical_res_debuff)
        }
    }

    pub fn get_def_down(&self) -> f32 {
        if self.def_down_debuff.len() == 0 {
            0.0
        } else {
            Debuff::accumulate(&self.def_down_debuff)
        }
    }

    // TODO slow?
    pub fn update(&mut self, time: f32) -> () {
        self.aura.update(time);
        if self.isfrozen && self.aura.aura == Vision::Physical {
            self.isfrozen = false;
        }
        for x in &mut self.element_res_debuff { x.update(time); }
        for x in &mut self.physical_res_debuff { x.update(time); }
        for x in &mut self.def_down_debuff { x.update(time); }
    }
}

// #[derive(Debug)]
pub struct CharacterData<'a> {
    pub idx: FieldCharacterIndex,
    pub character: &'a CharacterRecord,
    pub weapon: &'a WeaponRecord,
    pub artifact: &'a Artifact,
    pub state_ptr: *mut State,
}

impl<'a> CharacterData<'a> {
    pub fn new(idx: FieldCharacterIndex, character: &'a CharacterRecord, weapon: &'a WeaponRecord, artifact: &'a Artifact) -> Self {
        Self {
            idx,
            character,
            weapon,
            artifact,
            state_ptr: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, state: &mut State) -> () {
        self.state_ptr = state;
        state.merge(&self.character.state);
        state.merge(&self.weapon.state);
        state.merge(&self.artifact.state);
    }

    pub fn can_burst(&self) -> bool {
        self.state().energy >= self.character.energy_cost
    }

    pub fn state(&self) -> &State {
        unsafe {
            & *self.state_ptr
        }
    }

    pub fn state_mut(&self) -> &mut State {
        unsafe {
            &mut *self.state_ptr
        }
    }
}
