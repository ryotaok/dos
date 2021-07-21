use crate::state::State;
use crate::types::{AttackType, WeaponType, Particle, UnstackableBuff};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, SigilTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct SongOfBrokenPines {
    timer: SigilTimer,
}

impl SongOfBrokenPines {
    pub fn new() -> Self {
        Self {
            timer: SigilTimer::new(0.3, 20.0, 12.0, 4),
        }
    }
}

impl WeaponAbility for SongOfBrokenPines {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Song of Broken Pines").type_(Claymore).version(1.5)
            .base_atk(741.0)
            .atk(16.0)
            .dmg_phy(20.7)
    }
}

impl SpecialAbility for SongOfBrokenPines {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Na || guard.kind == Ca;
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::MillennialMovementSeries() {
                    s.atk     += 20.0;
                    s.atk_spd += 12.0;
                    s.stacked_buff += UnstackableBuff::MillennialMovementSeries();
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
