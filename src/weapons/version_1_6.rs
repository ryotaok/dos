use crate::state::State;
use crate::types::{AttackType, UnstackableBuff};
use crate::fc::{SpecialAbility, FieldCharacter, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, SigilTimer};

use AttackType::*;
// use Vision::*;

pub struct FreedomSworn {
    timer: SigilTimer,
}

impl FreedomSworn {
    pub fn new() -> Self {
        Self {
            timer: SigilTimer::new(0.5, 20.0, 12.0, 4),
        }
    }
}

impl SpecialAbility for FreedomSworn {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Freedom-Sworn").type_("Sword").version(1.6)
            .base_atk(608.0)
            .em(198.0).dmg_na(10.0).dmg_ca(10.0).dmg_skill(10.0).dmg_burst(10.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| enemy.trigger_er(&a.element).is_triggered())), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::MillennialMovementSeries() {
                    s.atk += 20.0;
                    s.na_dmg += 16.0;
                    s.ca_dmg += 16.0;
                    s.stacked_buff += UnstackableBuff::MillennialMovementSeries();
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct MitternachtsWaltz {
    na_timer: DurationTimer,
    skill_timer: DurationTimer,
}

impl MitternachtsWaltz {
    pub fn new() -> Self {
        Self {
            na_timer: DurationTimer::new(0.0, 5.0),
            skill_timer: DurationTimer::new(0.0, 5.0),
        }
    }
}

impl SpecialAbility for MitternachtsWaltz {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Mitternachts Waltz").type_("Bow").version(1.6)
            .base_atk(510.0)
            .dmg_phy(51.7)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.na_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Na)), time);
        self.skill_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Skill || a.kind == SkillDot)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.na_timer.is_active() {
            modifiable_state[owner_fc.idx.0].skill_dmg += 40.0;
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

pub struct DodocoTales {
    na_timer: DurationTimer,
    ca_timer: DurationTimer,
}

impl DodocoTales {
    pub fn new() -> Self {
        Self {
            na_timer: DurationTimer::new(0.0, 6.0),
            ca_timer: DurationTimer::new(0.0, 6.0),
        }
    }
}

impl SpecialAbility for DodocoTales {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Dodoco Tales").type_("Catalyst").version(1.6)
            .base_atk(454.0)
            .atk(55.1)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.na_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Na)), time);
        self.ca_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Ca)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.na_timer.is_active() {
            modifiable_state[owner_fc.idx.0].ca_dmg += 32.0;
        }
        if self.ca_timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk += 16.0;
        }
    }

    fn reset(&mut self) -> () {
        self.na_timer.reset();
        self.ca_timer.reset();
    }
}
