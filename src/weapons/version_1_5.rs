use crate::state::State;
use crate::types::{AttackType, UnstackableBuff};
use crate::fc::{SpecialAbility, FieldCharacter, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, SigilTimer};

use AttackType::*;
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

impl SpecialAbility for SongOfBrokenPines {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Song of Broken Pines").type_("Claymore").version(1.5)
            .base_atk(741.0)
            .atk(0.0 + 16.0)
            .dmg_phy(20.7)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && (a.kind == Na || a.kind == Ca))), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
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
