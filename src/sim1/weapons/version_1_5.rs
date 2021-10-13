use crate::sim1::state::State;
use crate::sim1::types::{AttackType, WeaponType, FieldEnergy, MILLENNIAL_MOVEMENT_SERIES};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct SongOfBrokenPines {
    idx: FieldCharacterIndex,
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

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(12.0, &[0.3,0.3,0.3,0.3, 20.0]),
        }
    }
}

impl SpecialAbility for SongOfBrokenPines {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == Na || event.kind == Ca));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 5 {
            for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != MILLENNIAL_MOVEMENT_SERIES {
                    data.state.atk += 20.0;
                    data.state.atk_spd += 12.0;
                    data.state.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
