use std::mem;
use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer};

use AttackType::*;
use Vision::*;

pub struct FesteringDesire;

impl SpecialAbility for FesteringDesire {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Festering Desire").type_("Sword").version(1.2)
            .base_atk(510.0)
            .er(45.9).dmg_skill(32.0)
    }

    fn intensify(&self, attack: &mut Attack, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if attack.kind == Skill || attack.kind == SkillDot {
            let mut state: Option<State> = None;
            mem::swap(&mut state, &mut attack.state);
            attack.state = if let Some(mut state) = state {
                state.cr += 12.0;
                Some(state)
            } else {
                Some(State::new().cr(12.0))
            }
        }
    }
}

#[derive(Debug)]
pub struct FrostBurial {
    timer: HitsTimer,
}

impl FrostBurial {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(10.0, 1) }
    }
}

impl SpecialAbility for FrostBurial {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.is_naca())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: if enemy.aura.aura == Cryo { 360.0 } else { 140.0 },
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

pub struct SnowTombedStarsilver(FrostBurial);

impl SnowTombedStarsilver {
    pub fn new() -> Self {
        Self(FrostBurial::new())
    }
}

impl SpecialAbility for SnowTombedStarsilver {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Snow-Tombed Starsilver").type_("Claymore").version(1.2)
            .base_atk(565.0)
            .dmg_phy(34.5)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct DragonspineSpear(FrostBurial);

impl DragonspineSpear {
    pub fn new() -> Self {
        Self(FrostBurial::new())
    }
}

impl SpecialAbility for DragonspineSpear {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragonspine Spear").type_("Polearm").version(1.2)
            .base_atk(454.0)
            .dmg_phy(69.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct Frostbearer(FrostBurial);

impl Frostbearer {
    pub fn new() -> Self {
        Self(FrostBurial::new())
    }
}

impl SpecialAbility for Frostbearer {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Frostbearer").type_("Catalyst").version(1.2)
            .base_atk(510.0)
            .atk(41.3)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}
