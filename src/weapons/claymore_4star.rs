use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer, StackTimer};
use crate::testutil;

use AttackType::*;
use Vision::*;

// version 1.0

pub struct PrototypeArchaicR5 {
    timer: HitsTimer,
}

impl PrototypeArchaicR5 {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(15.0, 1) }
    }
}

impl SpecialAbility for PrototypeArchaicR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Archaic R5").type_("Claymore").version(1.0)
            .base_atk(566.0)
            .atk(27.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(testutil::chance() < 0.5 && attack.iter().any(|a| a.is_naca())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 480.0,
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

pub struct WhiteblindR5 {
    timer: StackTimer,
}

impl WhiteblindR5 {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(0.5, 6.0, 4) }
    }
}

impl SpecialAbility for WhiteblindR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Whiteblind R5").type_("Claymore").version(1.0)
            .base_atk(510.0)
            .def(51.7)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_naca())), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk += 12.0 * self.timer.n as f32;
            modifiable_state[owner_fc.idx.0].def += 12.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SerpentSpineR5 {
    timer: StackTimer,
}

impl SerpentSpineR5 {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(4.0, 8.0, 5) }
    }
}

impl SpecialAbility for SerpentSpineR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Serpent Spine R5").type_("Claymore").version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, _attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(true), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].all_dmg += 10.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// one stack is always active
pub struct BlackcliffSlasherR5;

impl SpecialAbility for BlackcliffSlasherR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Slasher R5").type_("Claymore").version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGreatswordR5;

impl SpecialAbility for RoyalGreatswordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Greatsword R5").type_("Claymore").version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct RainslasherR5;

impl SpecialAbility for RainslasherR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Rainslasher R5").type_("Claymore").version(1.0)
            .base_atk(510.0)
            .em(165.0)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Electro |
            Vision::Hydro => modifiable_state[owner_fc.idx.0].all_dmg += 36.0,
            _ => (),
        }
    }
}

// The Bell
