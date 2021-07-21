use crate::state::State;
use crate::types::{AttackType, WeaponType, Particle, UnstackableBuff};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, DurationTimer, SigilTimer};

use AttackType::*;
use WeaponType::*;
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

impl WeaponAbility for FreedomSworn {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Freedom-Sworn").type_(Sword).version(1.6)
            .base_atk(608.0)
            .em(198.0).dmg_na(10.0).dmg_ca(10.0).dmg_skill(10.0).dmg_burst(10.0)
    }
}

impl SpecialAbility for FreedomSworn {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a| enemy.trigger_er(&a.element).is_triggered());
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
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

impl WeaponAbility for MitternachtsWaltz {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Mitternachts Waltz").type_(Bow).version(1.6)
            .base_atk(510.0)
            .dmg_phy(51.7)
    }
}

impl SpecialAbility for MitternachtsWaltz {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let na = guard.kind == Na;
        let skill = guard.kind == PressSkill || guard.kind == HoldSkill;
        self.na_timer.update(guard.second(na), time);
        self.skill_timer.update(guard.second(skill), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.na_timer.is_active() {
            modifiable_state[data.idx.0].skill_dmg += 40.0;
        }
        if self.skill_timer.is_active() {
            modifiable_state[data.idx.0].na_dmg += 40.0;
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

impl WeaponAbility for DodocoTales {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Dodoco Tales").type_(Catalyst).version(1.6)
            .base_atk(454.0)
            .atk(55.1)
    }
}

impl SpecialAbility for DodocoTales {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let na = guard.kind == Na;
        let ca = guard.kind == Ca;
        self.na_timer.update(guard.second(na), time);
        self.ca_timer.update(guard.second(ca), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.na_timer.is_active() {
            modifiable_state[data.idx.0].ca_dmg += 32.0;
        }
        if self.ca_timer.is_active() {
            modifiable_state[data.idx.0].atk += 16.0;
        }
    }

    fn reset(&mut self) -> () {
        self.na_timer.reset();
        self.ca_timer.reset();
    }
}
