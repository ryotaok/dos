use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer, StackTimer, DurationTimer};
use crate::testutil;

use AttackType::*;
use Vision::*;

// version 1.0

pub struct PrototypeAmberR5;

impl SpecialAbility for PrototypeAmberR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Amber R5").type_("Catalyst").version(1.0)
            .base_atk(510.0)
            .hp(41.3)
    }
}

pub struct MappaMareR5 {
    timer: StackTimer,
}

impl MappaMareR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.0, 10.0, 2),
        }
    }
}

impl SpecialAbility for MappaMareR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Mappa Mare R5").type_("Catalyst").version(1.0)
            .base_atk(565.0)
            .em(110.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| enemy.trigger_er(&a.element).is_triggered())), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].elemental_dmg += 16.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SolarPearlR5 {
    na_timer: DurationTimer,
    skill_timer: DurationTimer,
}

impl SolarPearlR5 {
    pub fn new() -> Self {
        Self {
            na_timer: DurationTimer::new(0.0, 6.0),
            skill_timer: DurationTimer::new(0.0, 6.0),
        }
    }
}

impl SpecialAbility for SolarPearlR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Solar Pearl R5").type_("Catalyst").version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.na_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Na)), time);
        self.skill_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Skill || a.kind == Burst)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.na_timer.is_active() {
            modifiable_state[owner_fc.idx.0].skill_dmg += 40.0;
            modifiable_state[owner_fc.idx.0].burst_dmg += 40.0;
        }
        if self.skill_timer.is_active() {
            modifiable_state[owner_fc.idx.0].na_dmg += 40.0;
        }
    }

    fn reset(&mut self) -> () {
        self.na_timer.reset();
        self.skill_timer.reset();
    }
}

// one stack is always active
pub struct BlackcliffAgateR5;

impl SpecialAbility for BlackcliffAgateR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Agate R5").type_("Catalyst").version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGrimoireR5;

impl SpecialAbility for RoyalGrimoireR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Grimoire R5").type_("Catalyst").version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct ThrillingTalesOfDragonSlayersR5 {
    timer: DurationTimer,
}

impl ThrillingTalesOfDragonSlayersR5 {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(20.0, 10.0),
        }
    }
}

impl SpecialAbility for ThrillingTalesOfDragonSlayersR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Thrilling Tales of Dragon Slayers R5").type_("Catalyst").version(1.0)
            .base_atk(401.0)
            .hp(35.2)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, _attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(true), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            // always buff the first member
            modifiable_state[0].atk += 48.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct EyeOfPerceptionR5 {
    timer: HitsTimer,
}

impl EyeOfPerceptionR5 {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(8.0, 1) }
    }
}

impl SpecialAbility for EyeOfPerceptionR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Eye of Perception R5").type_("Catalyst").version(1.0)
            .base_atk(454.0)
            .atk(55.1)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(testutil::chance() < 0.5 && attack.iter().any(|a| a.is_naca()) ), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 360.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheWidsithR5 {
    timer: DurationTimer,
    random_theme_song: usize,
}

impl TheWidsithR5 {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(30.0, 10.0), random_theme_song: 0,
        }
    }
}

impl SpecialAbility for TheWidsithR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Widsith R5").type_("Catalyst").version(1.0)
            .base_atk(510.0)
            .cd(55.1)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, _attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let before = self.timer.is_active();
        self.timer.update(gaurd.second(true), time);
        let after = self.timer.is_active();
        // check if the first time to gain the theme
        if !before && after {
            let p = testutil::chance();
            self.random_theme_song = if p > 0.6666 {
                0
            } else if p > 0.3333 {
                1
            } else {
                2
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            match self.random_theme_song {
                0 => modifiable_state[owner_fc.idx.0].atk += 120.0,
                1 => modifiable_state[owner_fc.idx.0].all_dmg += 96.0,
                2 => modifiable_state[owner_fc.idx.0].em += 480.0,
                _ => (),
            };
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
