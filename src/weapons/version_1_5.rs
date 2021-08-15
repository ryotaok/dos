use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, MILLENNIAL_MOVEMENT_SERIES};
use crate::fc::{SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct SongOfBrokenPines {
    timer: DurationTimer,
}

impl SongOfBrokenPines {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Song of Broken Pines").type_(Claymore).version(1.5)
            .base_atk(741.0)
            .atk(16.0)
            .physical_dmg(20.7)
    }

    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(12.0, &[0.3,0.3,0.3,0.3, 20.0]),
        }
    }
}

impl SpecialAbility for SongOfBrokenPines {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == data.idx && (event.kind == Na || event.kind == Ca));
    }

    fn modify(&self, modifiable_state: &mut [State], _data: &CharacterData, _enemy: &mut Enemy) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 4) => for s in modifiable_state.iter_mut() {
                if s.stacked_buff != MILLENNIAL_MOVEMENT_SERIES {
                    s.atk += 20.0;
                    s.atk_spd += 12.0;
                    s.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
                }
            },
            (true, 0) => for s in modifiable_state.iter_mut() {
                if s.stacked_buff == MILLENNIAL_MOVEMENT_SERIES {
                    s.atk -= 20.0;
                    s.atk_spd -= 12.0;
                    s.stacked_buff.turn_off(&MILLENNIAL_MOVEMENT_SERIES);
                }
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
