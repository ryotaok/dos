use crate::state::State;
use crate::types::{AttackType, UnstackableBuff};
use crate::fc::{SpecialAbility, FieldCharacter, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, StackTimer, SigilTimer};

use AttackType::*;
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

impl SpecialAbility for ElegyForTheEnd {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Elegy for the End").type_("Bow").version(1.4)
            .base_atk(608.0)
            .er(55.1).em(60.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a|
            match &a.kind {
                Skill | SkillDot | Burst | BurstDot => a.owned(owner_fc),
                _ => false,
            }
        );
        self.timer.update(gaurd.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::MillennialMovementSeries() {
                    s.atk += 20.0;
                    s.em  += 100.0;
                    s.stacked_buff += UnstackableBuff::MillennialMovementSeries();
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheAlleyFlash;

impl SpecialAbility for TheAlleyFlash {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Alley Flash").type_("Sword").version(1.4)
            .base_atk(620.0)
            .em(55.0)
            .dmg_na(24.0).dmg_ca(24.0).dmg_skill(24.0).dmg_burst(24.0)
    }
}

pub struct AlleyHunter {
    timer: StackTimer,
}

impl AlleyHunter {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(4.0, 8.0, 5) }
    }
}

impl SpecialAbility for AlleyHunter {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Alley Hunter").type_("Bow").version(1.4)
            .base_atk(565.0)
            .atk(27.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, _attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(true), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].all_dmg += 8.0 * (5 - self.timer.n) as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct WineAndSong;

impl SpecialAbility for WineAndSong {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Wine and Song").type_("Catalyst").version(1.4)
            .base_atk(565.0)
            .atk(0.0 + 40.0).er(30.6)
    }
}

pub struct WindblumeOde {
    timer: DurationTimer,
}

impl WindblumeOde {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 6.0) }
    }
}

impl SpecialAbility for WindblumeOde {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Windblume Ode").type_("Bow").version(1.4)
            .base_atk(510.0)
            .em(165.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Skill)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk += 32.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
