// use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, HitsTimer, NormalAttackAction, SkillAction, BurstAction};

// use AttackType::*;
// use Vision::*;

pub struct Composed {
    timer: HitsTimer,
}

impl Composed {
    pub fn new() -> Self {
        Self {
            timer: HitsTimer::new(16.0, 1),
        }
    }
}

impl SpecialAbility for Composed {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.is_skill())), time);
    }

    fn accelerate(&self, _na: &mut NormalAttackAction, skill: &mut SkillAction, _burst: &mut BurstAction) -> () {
        if self.timer.is_active() {
            skill.cd = 0.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SacrificialSwordR5(Composed);

impl SacrificialSwordR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialSwordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Sword R5").type_("Sword").version(1.0)
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

pub struct SacrificialGreatswordR5(Composed);

impl SacrificialGreatswordR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialGreatswordR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Greatsword R5").type_("Claymore").version(1.0)
            .base_atk(565.0)
            .er(30.6).em(0.0).atk_spd(0.0)
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

// pub struct SacrificialLanceR5(Composed);

pub struct SacrificialBowR5(Composed);

impl SacrificialBowR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialBowR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Bow R5").type_("Bow").version(1.0)
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

pub struct SacrificialFragmentsR5(Composed);

impl SacrificialFragmentsR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialFragmentsR5 {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Fragments R5").type_("Catalyst").version(1.0)
            .base_atk(454.0)
            .em(221.0)
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
