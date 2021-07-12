use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldAction, WeaponRecord, FieldCharacter, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, StackTimer, SigilTimer};

use AttackType::*;
use Vision::*;

// version 1.0

pub struct PrototypeRancourR5 {
    timer: StackTimer,
}

impl PrototypeRancourR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.3, 6.0, 4),
        }
    }
}

impl SpecialAbility for PrototypeRancourR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Rancour R5").type_("Sword").version(1.0)
            .base_atk(566.0)
            .dmg_phy(34.5)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_naca())), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk += 8.0 * self.timer.n as f32;
            modifiable_state[owner_fc.idx.0].def += 8.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// iron sting

pub struct TheBlackSwordR5;

impl SpecialAbility for TheBlackSwordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Black Sword R5").type_("Sword").version(1.0)
            .base_atk(510.0)
            .cr(27.6)
            .dmg_na(40.0).dmg_ca(40.0)
    }
}

// one stack is always active
pub struct BlackcliffLongswordR5;

impl SpecialAbility for BlackcliffLongswordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Longsword R5").type_("Sword").version(1.0)
            .base_atk(565.0)
            .atk(24.0).cd(36.8)
    }
}

pub struct RoyalLongswordR5;

impl SpecialAbility for RoyalLongswordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Longsword R5").type_("Sword").version(1.0)
            .base_atk(565.0)
            .atk(27.6).cr(0.0)
    }
}

// the passive is always active
pub struct HarbingerOfDawnR5;

impl SpecialAbility for HarbingerOfDawnR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Harbinger of Dawn R5").type_("Sword").version(1.0)
            .base_atk(401.0)
            .cr(28.0).cd(46.9)
    }
}

pub struct TheFluteR5 {
    timer: SigilTimer,
}

impl TheFluteR5 {
    pub fn new() -> Self {
        Self {
            timer: SigilTimer::new(0.5, 0.001, 0.0, 5),
        }
    }
}

impl SpecialAbility for TheFluteR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Flute R5").type_("Sword").version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_naca())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 200.0,
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

pub struct LionsRoarR5;

impl SpecialAbility for LionsRoarR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lion's Roar R5").type_("Sword").version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Electro |
            Vision::Pyro => modifiable_state[owner_fc.idx.0].all_dmg += 36.0,
            _ => (),
        }
    }
}
