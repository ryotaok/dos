use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer, StackTimer};
// use crate::testutil;

use AttackType::*;
use Vision::*;


// version 1.0

pub struct PrototypeStarglitterR5 {
    timer: StackTimer,
}

impl PrototypeStarglitterR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.0, 12.0, 2),
        }
    }
}

impl SpecialAbility for PrototypeStarglitterR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Starglitter R5").type_("Polearm").version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_skill())), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].na_dmg += 16.0 * self.timer.n as f32;
            modifiable_state[owner_fc.idx.0].ca_dmg += 16.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct CrescentPikeR5 {
    timer: HitsTimer,
}

impl CrescentPikeR5 {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(0.001, 1) }
    }
}

impl SpecialAbility for CrescentPikeR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Crescent Pike R5").type_("Polearm").version(1.0)
            .base_atk(566.0)
            .dmg_phy(34.5)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_naca())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 40.0,
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

pub struct DeathmatchR5;

impl SpecialAbility for DeathmatchR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Deathmatch R5").type_("Polearm").version(1.0)
            .base_atk(454.0)
            .atk(48.0).cr(36.8)
    }
}

// one stack is always active
pub struct BlackcliffPoleR5;

impl SpecialAbility for BlackcliffPoleR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Pole R5").type_("Polearm").version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalSpearR5;

impl SpecialAbility for RoyalSpearR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Spear R5").type_("Polearm").version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct WhiteTasselR5;

impl SpecialAbility for WhiteTasselR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("White Tassel R5").type_("Polearm").version(1.0)
            .base_atk(401.0)
            .cr(23.4)
            .dmg_na(48.0)
    }
}

pub struct DragonsBaneR5;

impl SpecialAbility for DragonsBaneR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragon's Bane R5").type_("Polearm").version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Hydro |
            Vision::Pyro => modifiable_state[owner_fc.idx.0].all_dmg += 36.0,
            _ => (),
        }
    }
}
