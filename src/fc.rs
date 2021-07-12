use crate::state::State;
use crate::artifact::Artifact;
use crate::action::{Attack, TimerGuard, NormalAttackAction, SkillAction, BurstAction};
use crate::types::{Vision, ElementalGauge, ElementalGaugeDecay, ElementalReaction, ElementalReactionType};


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldCharacterIndex(pub usize);

#[allow(unused_variables)]
pub trait SpecialAbility {
    // utility methods. each struct implements the respective method.
    fn character(&self) -> CharacterRecord { Default::default() }
    fn weapon(&self) -> WeaponRecord { Default::default() }
    fn artifact(&self) -> Artifact { Default::default() }

    // the variable is named `owner_fc` because `FieldCharacter` will own this
    // `SpecialAbility`.

    // Synchronize own timers to the emulator so that cool down times and
    // passive effect duration are up to date. The three methods of
    // `additional_attack`, `modify` and `accelerate` will depend on the data
    // mutated by this self.
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () { }

    // `Vec::push` additional attacks created. The `atk_queue` is `Vec` because
    // some character (Eula's hold skill) deals many additional attacks of
    // different elements at once. Similarly, Kazuha (Chihayaburu) deals
    // additional attacks of different kinds.
    // `AdditionalAttack` created by various effects of characters and weapons
    // are another entities who take part in the battle. These entities
    // can also attack an enemy, so we need to know how strong their attacks
    // are, i.e. additional attack DMG (or `Attack.multiplier`).
    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () { }

    // Apply own passive effects to `State`. This is the primary method that
    // "returns" passive effects to `FieldCharacter`. For example, suppose this
    // `SpecialAbility` is about Sucrose A1, "Catalyst Conversion" and she has
    // `FieldCharacterIndex(1)`. Then this method modifies the vector like:
    // 
    //     modifiable_state[0, 2 and 3].em += 50
    // 
    // The `Enemy` is mutable because some passive debuffs an enemy (e.g. DEF
    // down by Lisa).
    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () { }

    // Change speed of normal attacks or reset the cool down time of skill and
    // burst.
    fn accelerate(&self, na: &mut NormalAttackAction, skill: &mut SkillAction, burst: &mut BurstAction) -> () { }

    // This method can change the `State` of `Attack`. For example, Some abilities
    // increase CR of a specific action: Amber A1 (Every Arrow Finds Its
    // Target), Ganyu A1 (Undivided Heart), Festering Desire.
    fn intensify(&self, attack: &mut Attack, owner_fc: &FieldCharacter, enemy: &Enemy) -> () { }

    // reinitialize own states
    fn reset(&mut self) -> () {}
}

#[derive(Debug)]
pub struct CharacterRecord {
    pub name: String,
    pub release_date: String,
    pub version: f32,
    pub vision: String,
    pub weapon: String,
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
    pub na_1: f32,
    pub na_2: f32,
    pub na_3: f32,
    pub na_4: f32,
    pub na_5: f32,
    pub na_6: f32,
    pub na_time: f32,
    pub na_0: f32,
    pub ca_1: f32,
    pub ca_2: f32,
    pub ca_time: f32,
    pub burst_cd: f32,
    pub energy_cost: f32,
    pub burst_dmg: f32,
    pub press_cd: f32,
    pub hold_cd: f32,
    pub press_particle: f32,
    pub hold_particle: f32,
    pub press_dmg: f32,
    pub hold_dmg: f32,
    pub na_unit: f32,
    pub na_decay: ElementalGaugeDecay,
    pub ca_unit: f32,
    pub ca_decay: ElementalGaugeDecay,
    pub skill_unit: f32,
    pub skill_decay: ElementalGaugeDecay,
    pub skilldot_unit: f32,
    pub skilldot_decay: ElementalGaugeDecay,
    pub burst_unit: f32,
    pub burst_decay: ElementalGaugeDecay,
    pub burstdot_unit: f32,
    pub burstdot_decay: ElementalGaugeDecay,
}

impl Default for CharacterRecord {
    fn default() -> Self {
        use ElementalGaugeDecay::*;
        Self {
            name: String::from("Amber"), vision: String::from("Pyro"), weapon: String::from("Bow"), release_date: String::from("2020-09-28"), version: 1.0,
            base_hp: 0.0, base_atk: 0.0, base_def: 0.0,
            hp: 0.0, atk: 0.0, def: 0.0, cr: 5.0, cd: 50.0, er: 0.0, em: 0.0,
            dmg_na: 0.0, dmg_ca: 0.0, dmg_skill: 0.0, dmg_burst: 0.0,
            dmg_phy: 0.0, dmg_pyro: 0.0, dmg_cryo: 0.0, dmg_hydro: 0.0, dmg_electro: 0.0, dmg_anemo: 0.0, dmg_geo: 0.0, dmg_dendro: 0.0,
            na_1: 0.0, na_2: 0.0, na_3: 0.0, na_4: 0.0, na_5: 0.0, na_6: 0.0, na_time: 0.0,
            na_0: 0.0, ca_1: 0.0, ca_2: 0.0, ca_time: 0.0,
            burst_cd: 0.0, energy_cost: 0.0, burst_dmg: 0.0, press_cd: 0.0, hold_cd: 0.0, press_particle: 0.0, hold_particle: 0.0, press_dmg: 0.0, hold_dmg: 0.0,
            na_unit: 1.0, na_decay: A, ca_unit: 1.0, ca_decay: A, skill_unit: 1.0, skill_decay: A, skilldot_unit: 1.0, skilldot_decay: A, burst_unit: 1.0, burst_decay: A, burstdot_unit: 1.0, burstdot_decay: A,
        }
    }
}

#[allow(dead_code)]
impl CharacterRecord {
    pub fn name(mut self, name: &str) -> Self { self.name = name.to_string() ; self }
    pub fn release_date(mut self, release_date: &str) -> Self { self.release_date = release_date.to_string() ; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version ; self }
    pub fn vision(mut self, vision: &str) -> Self { self.vision = vision.to_string() ; self }
    pub fn weapon(mut self, weapon: &str) -> Self { self.weapon = weapon.to_string() ; self }
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
    pub fn na_1(mut self, na_1: f32) -> Self { self.na_1 = na_1 ; self }
    pub fn na_2(mut self, na_2: f32) -> Self { self.na_2 = na_2 ; self }
    pub fn na_3(mut self, na_3: f32) -> Self { self.na_3 = na_3 ; self }
    pub fn na_4(mut self, na_4: f32) -> Self { self.na_4 = na_4 ; self }
    pub fn na_5(mut self, na_5: f32) -> Self { self.na_5 = na_5 ; self }
    pub fn na_6(mut self, na_6: f32) -> Self { self.na_6 = na_6 ; self }
    pub fn na_time(mut self, na_time: f32) -> Self { self.na_time = na_time ; self }
    pub fn na_0(mut self, na_0: f32) -> Self { self.na_0 = na_0 ; self }
    pub fn ca_1(mut self, ca_1: f32) -> Self { self.ca_1 = ca_1 ; self }
    pub fn ca_2(mut self, ca_2: f32) -> Self { self.ca_2 = ca_2 ; self }
    pub fn ca_time(mut self, ca_time: f32) -> Self { self.ca_time = ca_time ; self }
    pub fn burst_cd(mut self, burst_cd: f32) -> Self { self.burst_cd = burst_cd ; self }
    pub fn energy_cost(mut self, energy_cost: f32) -> Self { self.energy_cost = energy_cost ; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.burst_dmg = burst_dmg ; self }
    pub fn press_cd(mut self, press_cd: f32) -> Self { self.press_cd = press_cd ; self }
    pub fn hold_cd(mut self, hold_cd: f32) -> Self { self.hold_cd = hold_cd ; self }
    pub fn press_particle(mut self, press_particle: f32) -> Self { self.press_particle = press_particle ; self }
    pub fn hold_particle(mut self, hold_particle: f32) -> Self { self.hold_particle = hold_particle ; self }
    pub fn press_dmg(mut self, press_dmg: f32) -> Self { self.press_dmg = press_dmg ; self }
    pub fn hold_dmg(mut self, hold_dmg: f32) -> Self { self.hold_dmg = hold_dmg ; self }
    pub fn na_unit(mut self, na_unit: f32) -> Self { self.na_unit = na_unit; self }
    pub fn na_decay(mut self, na_decay: ElementalGaugeDecay) -> Self { self.na_decay = na_decay; self }
    pub fn ca_unit(mut self, ca_unit: f32) -> Self { self.ca_unit = ca_unit; self }
    pub fn ca_decay(mut self, ca_decay: ElementalGaugeDecay) -> Self { self.ca_decay = ca_decay; self }
    pub fn skill_unit(mut self, skill_unit: f32) -> Self { self.skill_unit = skill_unit; self }
    pub fn skill_decay(mut self, skill_decay: ElementalGaugeDecay) -> Self { self.skill_decay = skill_decay; self }
    pub fn skilldot_unit(mut self, skilldot_unit: f32) -> Self { self.skilldot_unit = skilldot_unit; self }
    pub fn skilldot_decay(mut self, skilldot_decay: ElementalGaugeDecay) -> Self { self.skilldot_decay = skilldot_decay; self }
    pub fn burst_unit(mut self, burst_unit: f32) -> Self { self.burst_unit = burst_unit; self }
    pub fn burst_decay(mut self, burst_decay: ElementalGaugeDecay) -> Self { self.burst_decay = burst_decay; self }
    pub fn burstdot_unit(mut self, burstdot_unit: f32) -> Self { self.burstdot_unit = burstdot_unit; self }
    pub fn burstdot_decay(mut self, burstdot_decay: ElementalGaugeDecay) -> Self { self.burstdot_decay = burstdot_decay; self }

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

        state.infusion = if self.weapon == "Catalyst" {
            true
        } else {
            false
        };
        state
    }

    pub fn burst_action(&self) -> BurstAction {
        let element = Vision::from_string(&self.vision);
        BurstAction::new(element, self.burst_cd, self.energy_cost, self.burst_dmg)
    }

    pub fn skill_action(&self) -> SkillAction {
        let element = Vision::from_string(&self.vision);
        SkillAction::new(element, self.press_particle, self.press_cd, self.press_dmg)
    }

    pub fn normal_action(&self) -> NormalAttackAction {
        let element = Vision::from_string(&self.vision);
        let na_element = match self.weapon.as_str() {
            "Sword" => Vision::Physical,
            "Claymore" => Vision::Physical,
            "Polearm" => Vision::Physical,
            "Bow" => element,
            "Catalyst" => element,
            _ => panic!("This weapon is not recognized: {:?}", self.weapon.as_str()),
        };
        let mut nas: Vec<f32> = Vec::new();
        let mut na_total = 0.0_f32;
        for v in &[self.na_1, self.na_2, self.na_3, self.na_4, self.na_5, self.na_6, ] {
            if *v > 0.0 {
                nas.push(*v);
                na_total += *v;
            }
        }
        let mut cas: Vec<f32> = Vec::new();
        let mut ca_total = 0.0_f32;
        for v in &[self.na_0, self.ca_1, self.ca_2] {
            if *v > 0.0 {
                cas.push(*v);
                ca_total += *v;
            }
        }
        let na_dps = na_total / self.na_time;
        let ca_dps = ca_total / self.ca_time;
        match (self.na_time == 0.0, self.ca_time == 0.0, na_dps > ca_dps) {
            (true, true,  _) => panic!("division by zero: check na_time and ca_time of the character: {:?}", self.name),
            (true, false, _) => NormalAttackAction::ca(na_element, self.ca_time, ca_total),
            (false, true, _) => NormalAttackAction::na(na_element, self.na_time / nas.len() as f32, nas),
            (false, false, true) => NormalAttackAction::na(na_element, self.na_time / nas.len() as f32, nas),
            (false, false, false) => NormalAttackAction::ca(na_element, self.ca_time, ca_total),
        }
        // if na_dps > ca_dps {
        //     NormalAttackAction::na(na_element, self.na_time / nas.len() as f32, nas)
        // } else {
        //     NormalAttackAction::ca(na_element, self.ca_time, ca_total)
        // }
    }
}

#[derive(Debug)]
pub struct WeaponRecord {
    pub name: String,
    pub type_: String,
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
            name: String::from(""), type_: String::from("Sword"), version: 0.0,
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

    pub fn name(mut self, name: &str) -> Self { self.name = name.to_string(); self }
    pub fn type_(mut self, type_: &str) -> Self { self.type_ = type_.to_string(); self }
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

pub type FieldCharacterData = (
    FieldCharacter,
    FieldAbility,
    // `Attack`s created by this `FieldAbility`
    Vec<Attack>,
    // `Vec<State>` does not need to be included here because it can be modfied
    // by other characters.
    FieldAction
);

// #[derive(Debug)]
pub struct FieldCharacter {
    pub idx: FieldCharacterIndex,
    pub cr: CharacterRecord,
    pub wr: WeaponRecord,
    pub ar: Artifact,
    pub state: State,
    pub vision: Vision,
}

impl FieldCharacter {
    pub fn new(idx: FieldCharacterIndex, cr: CharacterRecord, vision: Vision, wr: WeaponRecord, ar: Artifact) -> Self {
        let mut state = State::new();
        state.merge(&cr.state());
        state.merge(&wr.state());
        state.merge(&ar.state);
        Self {
            idx,
            cr,
            wr,
            ar,
            state,
            vision
        }
    }

    pub fn to_data(self, fa: FieldAbility) -> FieldCharacterData {
        let a = FieldAction::new(&self.cr);
        (self, fa, Vec::new(), a)
    }

    pub fn noop(self, fa: FieldAbility) -> FieldCharacterData {
        let mut d = Self::to_data(self, fa);
        d.3.na = NormalAttackAction::noop();
        d
    }
}

pub struct FieldAbility {
    pub character: Box<dyn SpecialAbility>,
    pub weapon:    Box<dyn SpecialAbility>,
    pub artifact:  Box<dyn SpecialAbility>,
}

impl FieldAbility {
    pub fn boxed<C: 'static + SpecialAbility, W: 'static + SpecialAbility, A: 'static + SpecialAbility>(ca: C, wa: W, aa: A) -> Self {
        Self {
            character: Box::new(ca),
            weapon: Box::new(wa),
            artifact: Box::new(aa),
        }
    }

    pub fn to_data(self, idx: FieldCharacterIndex) -> FieldCharacterData {
        let cr = self.character.character();
        let wr = self.weapon.weapon();
        let ar = self.artifact.artifact();
        let vision = Vision::from_string(&cr.vision);
        FieldCharacter::new(idx, cr, vision, wr, ar).to_data(self)
    }
}

pub struct FieldAction {
    pub burst: BurstAction,
    pub skill: SkillAction,
    pub na: NormalAttackAction,
}

impl FieldAction {
    pub fn new(cr: &CharacterRecord) -> Self {
        Self {
            burst: cr.burst_action(),
            skill: cr.skill_action(),
            na: cr.normal_action(),
        }
    }
}
