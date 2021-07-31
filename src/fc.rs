use crate::types::AttackType;
use crate::state::State;
use crate::artifact::Artifact;
use crate::action::{Attack, ElementalAttack, TimerGuard, FullCharacterTimers};
use crate::types::{Vision, WeaponType, FieldEnergy, VecFieldEnergy, Particle, ElementalGauge, ElementalReaction, ElementalReactionType};


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldCharacterIndex(pub usize);

#[allow(unused_variables)]
pub trait SpecialAbility {
    // the variable is named `owner_fc` because `FieldCharacter` will own this
    // `SpecialAbility`.

    // Synchronize own timers to the emulator so that cool down times and
    // passive effect duration are up to date. The three methods of
    // `additional_attack`, `modify` and `accelerate` will depend on the data
    // mutated by this self.
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () { }

    // `Vec::push` additional attacks created. The `atk_queue` is `Vec` because
    // some character (Eula's hold skill) deals many additional attacks of
    // different elements at once. Similarly, Kazuha (Chihayaburu) deals
    // additional attacks of different kinds.
    // `AdditionalAttack` created by various effects of characters and weapons
    // are another entities who take part in the battle. These entities
    // can also attack an enemy, so we need to know how strong their attacks
    // are, i.e. additional attack DMG (or `Attack.multiplier`).
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () { }

    // Apply own passive effects to `State`. This is the primary method that
    // "returns" passive effects to `FieldCharacter`. For example, suppose this
    // `SpecialAbility` is about Sucrose A1, "Catalyst Conversion" and she has
    // `FieldCharacterIndex(1)`. Then this method modifies the vector like:
    // 
    //     modifiable_state[0, 2 and 3].em += 50
    // 
    // The `Enemy` is mutable because some passive debuffs an enemy (e.g. DEF
    // down by Lisa).
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () { }

    // This method can change the `State` of `Attack`. For example, Some abilities
    // increase CR of a specific action: Amber A1 (Every Arrow Finds Its
    // Target), Ganyu A1 (Undivided Heart), Festering Desire.
    fn intensify(&self, attack: &Attack) -> Option<State> { None }

    // reinitialize own states
    fn reset(&mut self) -> () {}
}

#[allow(unused_variables)]
pub trait CharacterAbility : SpecialAbility {
    // utility methods. each struct implements the respective method.
    fn record(&self) -> CharacterRecord { Default::default() }

    fn timers(&self) -> FullCharacterTimers;

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> ();

    fn use_hold(&self) -> bool { false }

    fn use_ca(&self) -> bool { false }

    // Change speed of normal attacks or reset the cool down time of skill and
    // burst.
    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () { }
}

#[allow(unused_variables)]
pub trait WeaponAbility : SpecialAbility {
    // utility methods. each struct implements the respective method.
    fn record(&self) -> WeaponRecord { Default::default() }


    // Although this method, `init_attack`, could be defined, because additional
    // attacks created by weapons do not have ICD timers (all the attacks are
    // physical), it always does nothing.

    // fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> ();

    // Change speed of normal attacks or reset the cool down time of skill and
    // burst.
    fn accelerate(&self, ac: &mut FullCharacterTimers) -> () { }
}

#[allow(unused_variables)]
pub trait ArtifactAbility : SpecialAbility {
    // utility methods. each struct implements the respective method.
    fn record(&self) -> Artifact { Default::default() }

    // Change speed of normal attacks or reset the cool down time of skill and
    // burst.
    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () { }
}

// #[derive(Debug)]
pub struct FieldCharacter<'a> {
    pub character: &'a mut dyn CharacterAbility,
    pub weapon: &'a mut dyn WeaponAbility,
    pub artifact: &'a mut dyn ArtifactAbility,
    pub data: CharacterData<'a>,
    pub timers: &'a mut Box<FullCharacterTimers>,
}

impl<'a> FieldCharacter<'a> {
    // pub fn new(timers: &'a mut Box<FullCharacterTimers>, idx: FieldCharacterIndex, character: &'a mut dyn CharacterAbility, weapon: &'a mut dyn WeaponAbility, artifact: &'a mut dyn ArtifactAbility) -> Self {
    //     *(*timers) = character.timers();
    //     character.init_attack(timers);
    //     let cr = character.record();
    //     let data = CharacterData::new(idx, cr, weapon.record(), artifact.record());
    //     Self {
    //         character,
    //         weapon,
    //         artifact,
    //         data,
    //         timers,
    //     }
    // }

    pub fn init(&mut self) -> () {
        // init FieldCharacter
        *(*self.timers) = self.character.timers();
        self.character.init_attack(self.timers);
    }

    pub fn maybe_attack(&self) -> Option<AttackType> {
        self.timers.maybe_attack(&self.data, self.character)
    }

    pub fn update(&mut self, guard: &mut TimerGuard, attack: &[ElementalAttack], particles: &[FieldEnergy], enemy: &Enemy, time: f32) -> () {
        self.timers.update(guard, attack, &self.data, time);
        self.character.update(guard, &self.timers, attack, particles, &self.data, enemy, time);
        self.weapon.update(guard, &self.timers, attack, particles, &self.data, enemy, time);
        self.artifact.update(guard, &self.timers, attack, particles, &self.data, enemy, time);
    }

    pub fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, enemy: &Enemy) -> () {
        self.character.additional_attack(atk_queue, particles, &self.timers, &self.data, enemy);
        self.weapon.additional_attack(atk_queue, particles, &self.timers, &self.data, enemy);
        self.artifact.additional_attack(atk_queue, particles, &self.timers, &self.data, enemy);
    }

    pub fn modify(&self, modifiable_state: &mut [State], enemy: &mut Enemy) -> () {
        self.character.modify(modifiable_state, &self.timers, &self.data, enemy);
        self.weapon.modify(modifiable_state, &self.timers, &self.data, enemy);
        self.artifact.modify(modifiable_state, &self.timers, &self.data, enemy);
    }

    pub fn accelerate(&mut self) -> () {
        self.character.accelerate(self.timers);
        self.weapon.accelerate(self.timers);
        self.artifact.accelerate(self.timers);
    }

    pub fn intensify(&self, attack: &Attack) -> (Option<State>, Option<State>, Option<State>) {
        (
            self.character.intensify(attack),
            self.weapon.intensify(attack),
            self.artifact.intensify(attack),
        )
    }

    // just drop self?
    pub fn reset(&mut self) -> () {
        self.character.reset();
        self.weapon.reset();
        self.artifact.reset();
    }
}

#[derive(Debug)]
pub struct CharacterRecord {
    pub name: &'static str,
    pub release_date: &'static str,
    pub version: f32,
    pub vision: Vision,
    pub weapon: WeaponType,
    pub base_hp: f32,
    pub base_atk: f32,
    pub base_def: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub cr: f32,
    pub cd: f32,
    pub er: f32,
    pub em: f32,
    pub dmg_na: f32,
    pub dmg_ca: f32,
    pub dmg_skill: f32,
    pub dmg_burst: f32,
    pub dmg_phy: f32,
    pub dmg_pyro: f32,
    pub dmg_cryo: f32,
    pub dmg_hydro: f32,
    pub dmg_electro: f32,
    pub dmg_anemo: f32,
    pub dmg_geo: f32,
    pub dmg_dendro: f32,
    pub energy_cost: f32,
}

impl Default for CharacterRecord {
    fn default() -> Self {
        Self {
            name: "Amber", vision: Vision::Pyro, weapon: WeaponType::Bow, release_date: "2020-09-28", version: 1.0,
            base_hp: 0.0, base_atk: 0.0, base_def: 0.0,
            hp: 0.0, atk: 0.0, def: 0.0, cr: 5.0, cd: 50.0, er: 0.0, em: 0.0,
            dmg_na: 0.0, dmg_ca: 0.0, dmg_skill: 0.0, dmg_burst: 0.0,
            dmg_phy: 0.0, dmg_pyro: 0.0, dmg_cryo: 0.0, dmg_hydro: 0.0, dmg_electro: 0.0, dmg_anemo: 0.0, dmg_geo: 0.0, dmg_dendro: 0.0,
            energy_cost: 0.0,
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
    pub fn base_hp(mut self, base_hp: f32) -> Self { self.base_hp = base_hp ; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.base_atk = base_atk ; self }
    pub fn base_def(mut self, base_def: f32) -> Self { self.base_def = base_def ; self }
    pub fn hp(mut self, hp: f32) -> Self { self.hp = hp ; self }
    pub fn atk(mut self, atk: f32) -> Self { self.atk = atk ; self }
    pub fn def(mut self, def: f32) -> Self { self.def = def ; self }
    pub fn cr(mut self, cr: f32) -> Self { self.cr = cr ; self }
    pub fn cd(mut self, cd: f32) -> Self { self.cd = cd ; self }
    pub fn er(mut self, er: f32) -> Self { self.er = er ; self }
    pub fn em(mut self, em: f32) -> Self { self.em = em ; self }
    pub fn dmg_na(mut self, dmg_na: f32) -> Self { self.dmg_na = dmg_na ; self }
    pub fn dmg_ca(mut self, dmg_ca: f32) -> Self { self.dmg_ca = dmg_ca ; self }
    pub fn dmg_skill(mut self, dmg_skill: f32) -> Self { self.dmg_skill = dmg_skill ; self }
    pub fn dmg_burst(mut self, dmg_burst: f32) -> Self { self.dmg_burst = dmg_burst ; self }
    pub fn dmg_phy(mut self, dmg_phy: f32) -> Self { self.dmg_phy = dmg_phy ; self }
    pub fn dmg_pyro(mut self, dmg_pyro: f32) -> Self { self.dmg_pyro = dmg_pyro ; self }
    pub fn dmg_cryo(mut self, dmg_cryo: f32) -> Self { self.dmg_cryo = dmg_cryo ; self }
    pub fn dmg_hydro(mut self, dmg_hydro: f32) -> Self { self.dmg_hydro = dmg_hydro ; self }
    pub fn dmg_electro(mut self, dmg_electro: f32) -> Self { self.dmg_electro = dmg_electro ; self }
    pub fn dmg_anemo(mut self, dmg_anemo: f32) -> Self { self.dmg_anemo = dmg_anemo ; self }
    pub fn dmg_geo(mut self, dmg_geo: f32) -> Self { self.dmg_geo = dmg_geo ; self }
    pub fn dmg_dendro(mut self, dmg_dendro: f32) -> Self { self.dmg_dendro = dmg_dendro ; self }
    pub fn energy_cost(mut self, energy_cost: f32) -> Self { self.energy_cost = energy_cost ; self }

    pub fn state(&self) -> State {
        let mut state = State::new();
        state.base_hp = self.base_hp;
        state.base_def = self.base_def;
        state.base_atk = self.base_atk;
        state.hp = self.hp;
        state.atk = self.atk;
        state.def = self.def;
        state.cr = self.cr;
        state.cd = self.cd;
        state.er = self.er;
        state.em = self.em;

        state.energy_cost = self.energy_cost;
        // state.press_cd = self.press_cd;
        // state.hold_cd = self.hold_cd;
        // state.press_particle = self.press_particle;
        // state.hold_particle = self.hold_particle;

        state.na_dmg = self.dmg_na;
        state.ca_dmg = self.dmg_ca;
        state.skill_dmg = self.dmg_skill;
        state.burst_dmg = self.dmg_burst;
        state.physical_dmg = self.dmg_phy;
        state.pyro_dmg = self.dmg_pyro;
        state.cryo_dmg = self.dmg_cryo;
        state.hydro_dmg = self.dmg_hydro;
        state.electro_dmg = self.dmg_electro;
        state.anemo_dmg = self.dmg_anemo;
        state.geo_dmg = self.dmg_geo;
        state.dendro_dmg = self.dmg_dendro;

        state.infusion = if self.weapon == WeaponType::Catalyst {
            true
        } else {
            false
        };
        state
    }
}

#[derive(Debug)]
pub struct WeaponRecord {
    pub name: &'static str,
    pub type_: WeaponType,
    pub version: f32,
    pub base_atk: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub cr: f32,
    pub cd: f32,
    pub er: f32,
    pub em: f32,
    pub atk_spd: f32,
    pub dmg_na: f32,
    pub dmg_ca: f32,
    pub dmg_skill: f32,
    pub dmg_burst: f32,
    pub dmg_phy: f32,
    pub dmg_pyro: f32,
    pub dmg_cryo: f32,
    pub dmg_hydro: f32,
    pub dmg_electro: f32,
    pub dmg_anemo: f32,
    pub dmg_geo: f32,
    pub dmg_dendro: f32,
}

impl Default for WeaponRecord {
    fn default() -> Self {
        Self {
            name: "", type_: WeaponType::Sword, version: 0.0,
            base_atk: 0.0,
            hp: 0.0, atk: 0.0, def: 0.0, cr: 0.0, cd: 0.0, er: 0.0, em: 0.0, atk_spd: 0.0,
            dmg_na: 0.0, dmg_ca: 0.0, dmg_skill: 0.0, dmg_burst: 0.0,
            dmg_phy: 0.0, dmg_pyro: 0.0, dmg_cryo: 0.0, dmg_hydro: 0.0, dmg_electro: 0.0, dmg_anemo: 0.0, dmg_geo: 0.0, dmg_dendro: 0.0,
        }
    }
}

#[allow(dead_code)]
impl WeaponRecord {
    pub fn state(&self) -> State {
        let mut state = State::new();
        state.base_atk = self.base_atk;
        state.hp = self.hp;
        state.atk = self.atk;
        state.def = self.def;
        state.cr = self.cr;
        state.cd = self.cd;
        state.er = self.er;
        state.em = self.em;
        state.atk_spd = self.atk_spd;

        state.na_dmg = self.dmg_na;
        state.ca_dmg = self.dmg_ca;
        state.skill_dmg = self.dmg_skill;
        state.burst_dmg = self.dmg_burst;

        state.physical_dmg = self.dmg_phy;
        state.pyro_dmg = self.dmg_pyro;
        state.cryo_dmg = self.dmg_cryo;
        state.hydro_dmg = self.dmg_hydro;
        state.electro_dmg = self.dmg_electro;
        state.anemo_dmg = self.dmg_anemo;
        state.geo_dmg = self.dmg_geo;
        state.dendro_dmg = self.dmg_dendro;

        state
    }

    pub fn name(mut self, name: &'static str) -> Self { self.name = name; self }
    pub fn type_(mut self, type_: WeaponType) -> Self { self.type_ = type_; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.base_atk = base_atk; self }
    pub fn hp(mut self, hp: f32) -> Self { self.hp = hp; self }
    pub fn atk(mut self, atk: f32) -> Self { self.atk = atk; self }
    pub fn def(mut self, def: f32) -> Self { self.def = def; self }
    pub fn cr(mut self, cr: f32) -> Self { self.cr = cr; self }
    pub fn cd(mut self, cd: f32) -> Self { self.cd = cd; self }
    pub fn er(mut self, er: f32) -> Self { self.er = er; self }
    pub fn em(mut self, em: f32) -> Self { self.em = em; self }
    pub fn atk_spd(mut self, atk_spd: f32) -> Self { self.atk_spd = atk_spd; self }
    pub fn dmg_na(mut self, dmg_na: f32) -> Self { self.dmg_na = dmg_na; self }
    pub fn dmg_ca(mut self, dmg_ca: f32) -> Self { self.dmg_ca = dmg_ca; self }
    pub fn dmg_skill(mut self, dmg_skill: f32) -> Self { self.dmg_skill = dmg_skill; self }
    pub fn dmg_burst(mut self, dmg_burst: f32) -> Self { self.dmg_burst = dmg_burst; self }
    pub fn dmg_phy(mut self, dmg_phy: f32) -> Self { self.dmg_phy = dmg_phy; self }
    pub fn dmg_pyro(mut self, dmg_pyro: f32) -> Self { self.dmg_pyro = dmg_pyro; self }
    pub fn dmg_cryo(mut self, dmg_cryo: f32) -> Self { self.dmg_cryo = dmg_cryo; self }
    pub fn dmg_hydro(mut self, dmg_hydro: f32) -> Self { self.dmg_hydro = dmg_hydro; self }
    pub fn dmg_electro(mut self, dmg_electro: f32) -> Self { self.dmg_electro = dmg_electro; self }
    pub fn dmg_anemo(mut self, dmg_anemo: f32) -> Self { self.dmg_anemo = dmg_anemo; self }
    pub fn dmg_geo(mut self, dmg_geo: f32) -> Self { self.dmg_geo = dmg_geo; self }
    pub fn dmg_dendro(mut self, dmg_dendro: f32) -> Self { self.dmg_dendro = dmg_dendro; self }
}

// TODO remove or refine
impl From<State> for WeaponRecord {
    fn from(state: State) -> Self {
        Self {
            name: "",
            type_: WeaponType::Sword,
            version: 1.0,

            base_atk: state.base_atk,
            hp: state.hp,
            atk: state.atk,
            def: state.def,
            cr: state.cr,
            cd: state.cd,
            er: state.er,
            em: state.em,
            atk_spd: state.atk_spd,

            dmg_na: state.na_dmg,
            dmg_ca: state.ca_dmg,
            dmg_skill: state.skill_dmg,
            dmg_burst: state.burst_dmg,

            dmg_phy: state.physical_dmg,
            dmg_pyro: state.pyro_dmg,
            dmg_cryo: state.cryo_dmg,
            dmg_hydro: state.hydro_dmg,
            dmg_electro: state.electro_dmg,
            dmg_anemo: state.anemo_dmg,
            dmg_geo: state.geo_dmg,
            dmg_dendro: state.dendro_dmg,
        }
    }
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
pub struct FieldCharacterData<'a> {
    pub fc: FieldCharacter<'a>,

    // `Attack`s created by this `FieldAbility`
    pub atk_queue: Vec<ElementalAttack>,
    // `Vec<State>` does not need to be included here because it can be modfied
    // by other characters.
}

impl<'a> FieldCharacterData<'a> {
    pub fn new(timers: &'a mut Box<FullCharacterTimers>, character: &'a mut dyn CharacterAbility, weapon: &'a mut dyn WeaponAbility, artifact: &'a mut dyn ArtifactAbility, data: CharacterData<'a>) -> Self {
        let mut fc = FieldCharacter {
            character,
            weapon,
            artifact,
            data,
            timers,
        };
        fc.init();
        Self { fc, atk_queue: Vec::new() }
    }
}

// #[derive(Debug)]
pub struct CharacterData<'a> {
    pub idx: FieldCharacterIndex,
    pub cr: &'a CharacterRecord,
    pub wr: &'a WeaponRecord,
    pub ar: &'a Artifact,
    pub state: State,
    pub vision: Vision,
}

impl<'a> CharacterData<'a> {
    pub fn new(idx: FieldCharacterIndex, cr: &'a CharacterRecord, wr: &'a WeaponRecord, ar: &'a Artifact) -> Self {
        let mut state = State::new();
        state.merge(&cr.state());
        state.merge(&wr.state());
        state.merge(&ar.state);
        let vision = cr.vision;
        Self {
            idx,
            cr,
            wr,
            ar,
            state,
            vision,
        }
    }

    // pub fn to_data(self, character: Box<dyn CharacterAbility>, weapon: Box<dyn WeaponAbility>, artifact: Box<dyn ArtifactAbility>,) -> FieldCharacterData {
    //     FieldCharacterData {
    //         fc: self,
    //         atk_queue: Vec::new(),
    //         particles: Vec::new(),
    //         character,
    //         weapon,
    //         artifact,
    //     }
    // }

    pub fn can_burst(&self) -> bool {
        self.state.energy >= self.cr.energy_cost
    }

    pub fn infused_element<'b>(&'b self, attack: &'b ElementalAttack) -> &'b Vision {
        if self.state.infusion {
            &self.vision
        } else {
            &attack.element
        }
    }
}
