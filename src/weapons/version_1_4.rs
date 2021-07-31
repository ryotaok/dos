use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, MILLENNIAL_MOVEMENT_SERIES};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, StackTimer, SigilTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct ElegyForTheEnd {
    timer: SigilTimer,
}

impl ElegyForTheEnd {
    pub fn new() -> Self {
        Self {
            timer: SigilTimer::new(0.2, 20.0, 12.0, 4),
        }
    }
}

impl WeaponAbility for ElegyForTheEnd {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Elegy for the End").type_(Bow).version(1.4)
            .base_atk(608.0)
            .er(55.1).em(60.0)
    }
}

impl SpecialAbility for ElegyForTheEnd {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = unsafe {
            attack.iter().any(|&a|
                match (*a.atk).kind {
                    PressSkill | HoldSkill | SkillDot | Burst | BurstDot => true,
                    _ => false,
                }
            )
        };
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != MILLENNIAL_MOVEMENT_SERIES {
                    s.atk += 20.0;
                    s.em  += 100.0;
                    s.stacked_buff += MILLENNIAL_MOVEMENT_SERIES;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheAlleyFlash;

impl WeaponAbility for TheAlleyFlash {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Alley Flash").type_(Sword).version(1.4)
            .base_atk(620.0)
            .em(55.0)
            .dmg_na(24.0).dmg_ca(24.0).dmg_skill(24.0).dmg_burst(24.0)
    }
}

impl SpecialAbility for TheAlleyFlash {}

pub struct AlleyHunter {
    timer: StackTimer,
}

impl AlleyHunter {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(4.0, 8.0, 5) }
    }
}

impl WeaponAbility for AlleyHunter {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Alley Hunter").type_(Bow).version(1.4)
            .base_atk(565.0)
            .atk(27.6)
    }
}

impl SpecialAbility for AlleyHunter {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(guard.second(true), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].all_dmg += 8.0 * (5 - self.timer.n) as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct WineAndSong;

impl WeaponAbility for WineAndSong {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Wine and Song").type_(Catalyst).version(1.4)
            .base_atk(565.0)
            .atk(0.0 + 40.0).er(30.6)
    }
}

impl SpecialAbility for WineAndSong {}

pub struct WindblumeOde {
    timer: DurationTimer,
}

impl WindblumeOde {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 6.0) }
    }
}

impl WeaponAbility for WindblumeOde {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Windblume Ode").type_(Bow).version(1.4)
            .base_atk(510.0)
            .em(165.0)
    }
}

impl SpecialAbility for WindblumeOde {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == PressSkill || guard.kind == HoldSkill;
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].atk += 32.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
