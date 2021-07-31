use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct GrasscuttersLight {
    timer: DurationTimer,
}

impl GrasscuttersLight {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(0.0, 12.0),
        }
    }
}

impl WeaponAbility for GrasscuttersLight {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Grasscutter's Light").type_(Polearm).version(2.1)
            .base_atk(608.0)
            .er(55.1)
    }
}

impl SpecialAbility for GrasscuttersLight {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(guard.check_second(Burst), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        state.atk += 0.28 * data.state.er;
        if self.timer.is_active() {
            state.er += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct FumetsuGekka {
    timer: DurationTimer,
}

impl FumetsuGekka {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(0.0, 12.0),
        }
    }
}

impl WeaponAbility for FumetsuGekka {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Fumetsu Gekka").type_(Catalyst).version(2.1)
            .base_atk(608.0)
            // TODO healing bonus
            .hp(49.6)
    }
}

impl SpecialAbility for FumetsuGekka {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(guard.check_second(Burst), time);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if timers.na_timer().is_active() {
            for s in modifiable_state.iter_mut() {
                s.energy += 0.6;
            }
        }
        let state = &mut modifiable_state[data.idx.0];
        // TODO incorrect
        state.na_dmg += 0.0001 * data.state.HP();
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
