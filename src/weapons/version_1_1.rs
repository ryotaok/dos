use crate::state::State;
use crate::types::{AttackType};
use crate::fc::{SpecialAbility, FieldCharacter, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, StackTimer};

use AttackType::*;
// use Vision::*;

pub struct GoldenMajesty {
    timer: StackTimer,
}

impl GoldenMajesty {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.3, 8.0, 5),
        }
    }
}

impl SpecialAbility for GoldenMajesty {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("GoldenMajesty").type_("None").version(1.1)
            .base_atk(608.0)
            .atk(49.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a|
            match &a.kind {
                Na | Ca | Skill | SkillDot | Burst | BurstDot => a.owned(owner_fc),
                _ => false,
            }
        );
        self.timer.update(gaurd.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk += 8.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheUnforged(GoldenMajesty);

impl TheUnforged {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl SpecialAbility for TheUnforged {
    fn weapon(&self) -> WeaponRecord {
        self.0.weapon().name("The Unforged").type_("Claymore")
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct SummitShaper(GoldenMajesty);

impl SummitShaper {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl SpecialAbility for SummitShaper {
    fn weapon(&self) -> WeaponRecord {
        self.0.weapon().name("Summit shaper").type_("Sword")
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct VortexVanquisher(GoldenMajesty);

impl VortexVanquisher {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl SpecialAbility for VortexVanquisher {
    fn weapon(&self) -> WeaponRecord {
        self.0.weapon().name("Vortex Vanquisher").type_("Polearm")
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct MemoryOfDust(GoldenMajesty);

impl MemoryOfDust {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl SpecialAbility for MemoryOfDust {
    fn weapon(&self) -> WeaponRecord {
        self.0.weapon().name("Memory of Dust").type_("Catalyst")
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}
