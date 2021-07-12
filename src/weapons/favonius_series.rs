use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer};

use AttackType::*;
use Vision::*;

pub struct Windfall {
    timer: HitsTimer,
}

impl Windfall {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(6.0, 1) }
    }
}

impl SpecialAbility for Windfall {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a|
            match &a.kind {
                Na | Ca | Skill | SkillDot | Burst | BurstDot => a.owned(owner_fc),
                _ => false,
            }
        );
        self.timer.update(gaurd.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: StandStill,
                element: Physical,
                multiplier: 0.0,
                particle: Some(3.0 * owner_fc.state.cr / 100.0),
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}


pub struct FavoniusGreatswordR5(Windfall);

impl FavoniusGreatswordR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl SpecialAbility for FavoniusGreatswordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Greatsword R5").type_("Claymore").version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusSwordR5(Windfall);

impl FavoniusSwordR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl SpecialAbility for FavoniusSwordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Sword R5").type_("Sword").version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusLanceR5(Windfall);

impl FavoniusLanceR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl SpecialAbility for FavoniusLanceR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Lance R5").type_("Polearm").version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusWarbowR5(Windfall);

impl FavoniusWarbowR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl SpecialAbility for FavoniusWarbowR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Warbow R5").type_("Bow").version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusCodexR5(Windfall);

impl FavoniusCodexR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl SpecialAbility for FavoniusCodexR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Codex R5").type_("Catalyst").version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}
