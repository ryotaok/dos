use std::ptr;
use std::mem;
// use std::cell::{RefCell};
// use rand::prelude::*;

use crate::state::State;
use crate::artifact::Artifact;

use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, Enemy, FieldCharacter, SpecialAbility, CharacterAbility, WeaponAbility, ArtifactAbility, FieldCharacterData, CharacterData};
use crate::types::{AttackType, Vision, Particle, BareElementalGauge, ElementalGaugeDecay};
use crate::action::{Attack, ElementalAttack, CharacterTimers, FullCharacterTimers, TimerGuard, EffectTimer, CharacterTimersBuilder, LoopTimer, DotTimer};

use Vision::*;

// #[cfg(test)]
// #[cfg(not(test))]
pub fn chance() -> f32 {
    // rand::random::<f32>()
    0.0
}

pub struct TestCharacter {
    pub vision: Vision,
    pub na: Attack,
    pub press: Attack,
    pub burst: Attack,
}

impl TestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Self {
        Self {
            vision,
            na: Attack {
                kind: AttackType::Na,
                gauge: BareElementalGauge::a1(),
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: BareElementalGauge::a1(),
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: BareElementalGauge::a1(),
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for TestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .vision(&self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn init_timers_and_attack(&mut self, timers: &mut Box<dyn CharacterTimers>) -> () {
        let mut ct = CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::single_hit(12.0))
            .build_na_press();
        *(*timers) = ct;
        self.na.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for TestCharacter {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &dyn CharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer.is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.press_timer.is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.press));
            particles.push(Particle::new(Pyro, 2.0));
        }
        if timers.na_timer.is_active() && timers.na_timer.n() > 0 {
            if owner_fc.state.infusion {
                atk_queue.push(ElementalAttack::new(self.vision, &self.na));
            } else {
                atk_queue.push(ElementalAttack::physical(&self.na));
            }
        }
    }
}

pub struct NoSkillTestCharacter {
    pub vision: Vision,
    pub na: Attack,
    pub burst: Attack,
}

impl NoSkillTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Self {
        Self {
            vision,
            na: Attack {
                kind: AttackType::Na,
                gauge: BareElementalGauge::a1(),
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: BareElementalGauge::a1(),
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for NoSkillTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .vision(&self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn init_timers_and_attack(&mut self, timers: &mut Box<dyn CharacterTimers>) -> () {
        let mut ct = CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .burst(DotTimer::single_hit(12.0))
            .build_na();
        *(*timers) = ct;
        self.na.icd_timer = &mut timers.na_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for NoSkillTestCharacter {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<Particle>, timers: &dyn CharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer.is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.na_timer.is_active() && timers.na_timer.n() > 0 {
            if owner_fc.state.infusion {
                atk_queue.push(ElementalAttack::new(self.vision, &self.na));
            } else {
                atk_queue.push(ElementalAttack::physical(&self.na));
            }
        }
    }
}

pub struct TestWeapon;

impl SpecialAbility for TestWeapon {}

impl WeaponAbility for TestWeapon {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
    }
}

pub struct TestArtifact(pub State);

impl SpecialAbility for TestArtifact {}

impl ArtifactAbility for TestArtifact {
    fn record(&self) -> Artifact {
        let mut state = State::new();
        state.merge(&self.0);
        Artifact {
            name: String::from("simple"),
            version: 1.0,
            preference: Vec::new(),
            state
        }
    }
}

pub struct TestEnvironment;

impl TestEnvironment {
    pub fn enemy() -> Enemy {
        Enemy::simple()
    }

    pub fn vision(timers: &mut Box<dyn CharacterTimers>, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<TestCharacter, TestWeapon, TestArtifact> {
        let ca = TestCharacter::new(idx, vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        let fc = FieldCharacter::new(timers, idx, ca, wa, aa);
        FieldCharacterData { fc, atk_queue: Vec::new() }
    }

    pub fn no_skill(timers: &mut Box<dyn CharacterTimers>, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<NoSkillTestCharacter, TestWeapon, TestArtifact> {
        let ca = NoSkillTestCharacter::new(idx, vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        let fc = FieldCharacter::new(timers, idx, ca, wa, aa);
        FieldCharacterData { fc, atk_queue: Vec::new() }
    }

    // pub fn ca(timers: &mut Box<FullCharacterTimers>, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData {
    //     let ca = CaTestCharacter::new(idx, vision);
    //     let wa = TestWeapon;
    //     let aa = TestArtifact(state);
    //     FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
    //         ca, Box::new(wa), Box::new(aa)
    //     )
    // }

    // pub fn hold(timers: &mut Box<FullCharacterTimers>, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData {
    //     let ca = PressHoldTestCharacter::new(idx, vision);
    //     let wa = TestWeapon;
    //     let aa = TestArtifact(state);
    //     FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
    //         ca, Box::new(wa), Box::new(aa)
    //     )
    // }

    // pub fn artifact<T: 'static + ArtifactAbility>(timers: &mut Box<FullCharacterTimers>, idx: FieldCharacterIndex, vision: Vision, aa: T) -> FieldCharacterData {
    //     let ca = TestCharacter::new(idx, vision);
    //     let wa = TestWeapon;
    //     FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
    //         ca, Box::new(wa), Box::new(aa)
    //     )
    // }

    // pub fn fc_artifact<T: 'static +  SpecialAbility>(idx: FieldCharacterIndex, aa: T) -> FieldCharacterData {
    //     let ca = TestCharacter { vision: String::from("Pyro") };
    //     let cr = ca.character();
    //     let vision = Vision::from(&cr.vision);
    //     let wa = TestWeapon;
    //     FieldCharacter::new(idx, cr, vision, wa.weapon(), aa.artifact()).to_data(FieldAbility {
    //         character: Box::new(ca),
    //         weapon: Box::new(wa),
    //         artifact: Box::new(aa),
    //     })
    // }
}
