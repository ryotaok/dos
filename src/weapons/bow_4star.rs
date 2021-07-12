use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer, StackTimer};
use crate::testutil;

use AttackType::*;
use Vision::*;

// version 1.0

pub struct PrototypeCrescentR5;

impl SpecialAbility for PrototypeCrescentR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Crescent R5").type_("Bow").version(1.0)
            .base_atk(510.0)
            .atk(41.3 + 72.0)
    }
}

pub struct CompoundBowR5 {
    timer: StackTimer,
}

impl CompoundBowR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.3, 6.0, 4),
        }
    }
}

impl SpecialAbility for CompoundBowR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Compound Bow R5").type_("Bow").version(1.0)
            .base_atk(454.0)
            .dmg_phy(69.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_naca())), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk     += 8.0 * self.timer.n as f32;
            modifiable_state[owner_fc.idx.0].atk_spd += 2.4 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheViridescentHuntR5 {
    timer: HitsTimer,
}

impl TheViridescentHuntR5 {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(10.0, 8) }
    }
}

impl SpecialAbility for TheViridescentHuntR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Viridescent Hunt R5").type_("Bow").version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(testutil::chance() < 0.5 && attack.iter().any(|a| a.is_naca())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 80.0,
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

// one stack is always active
pub struct BlackcliffWarbowR5;

impl SpecialAbility for BlackcliffWarbowR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Warbow R5").type_("Bow").version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalBowR5;

impl SpecialAbility for RoyalBowR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Bow R5").type_("Bow").version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

pub struct SlingshotR5;

impl SpecialAbility for SlingshotR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Slingshot R5").type_("Bow").version(1.0)
            .base_atk(354.0)
            .cr(31.2)
            .dmg_na(60.0).dmg_ca(60.0)
    }
}

pub struct RustR5;

impl SpecialAbility for RustR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Rust R5").type_("Bow").version(1.0)
            .base_atk(510.0)
            .atk(41.3)
            .dmg_na(80.0).dmg_ca(-10.0)
    }
}

pub struct TheStringlessR5;

impl SpecialAbility for TheStringlessR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Stringless R5").type_("Bow").version(1.0)
            .base_atk(510.0)
            .em(165.0)
            .dmg_skill(48.0).dmg_burst(48.0)
    }
}
