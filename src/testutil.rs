use std::ptr;
use std::mem;
// use std::cell::{RefCell};
// use rand::prelude::*;

use crate::state::State;
use crate::artifact::Artifact;

use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, Enemy, FieldCharacter, SpecialAbility, CharacterAbility, WeaponAbility, ArtifactAbility, FieldCharacterData};
use crate::types::{AttackType, Vision, Particle, ElementalGauge, ElementalGaugeDecay};
use crate::action::{Attack, TimerGuard, MainAttack, EffectTimer, CharacterTimersBuilder, NaPressBurstTimers, NaBurstTimers, NaCaPressBurstTimers, NaPressHoldBurstTimers, LoopTimer, DotTimer, HitsTimer, StaminaTimer};

use Vision::*;
use ElementalGaugeDecay::*;

pub fn revert<T>(mut xs: Vec<T>, a: usize, b: usize, c: usize) -> (Vec<T>, Vec<T>, Vec<T>) {
    if a + b + c > xs.len() {
        panic!("cannot revert");
    }
    let mut av: Vec<T> = Vec::with_capacity(a);
    let mut bv: Vec<T> = Vec::with_capacity(b);
    let mut cv: Vec<T> = Vec::with_capacity(c);
    match (a, b, c) {
        (0, 0, 0) => (av, bv, cv),
        (_, 0, 0) => { for x in xs.drain(..) { av.push(x); } (av, bv, cv ) },
        (0, _, 0) => { for x in xs.drain(..) { bv.push(x); } (av, bv, cv ) },
        (0, 0, _) => { for x in xs.drain(..) { cv.push(x); } (av, bv, cv ) },
        (0, s, t) => {
            for x in xs.drain(..s) { bv.push(x); }
            for x in xs.drain(..t) { cv.push(x); }
            (av, bv, cv)
        }
        (s, 0, t) => {
            for x in xs.drain(..s) { av.push(x); }
            for x in xs.drain(..t) { cv.push(x); }
            (av, bv, cv)
        }
        (s, t, 0) => {
            for x in xs.drain(..s) { av.push(x); }
            for x in xs.drain(..t) { bv.push(x); }
            (av, bv, cv)
        }
        _ => {
            for x in xs.drain(..a) { av.push(x); }
            for x in xs.drain(..b) { bv.push(x); }
            for x in xs.drain(..c) { cv.push(x); }
            (av, bv, cv)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revert_1() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 1, 2, 2);
        assert_eq!(av, [1]);
        assert_eq!(bv, [2,3]);
        assert_eq!(cv, [4,5]);
    }

    #[test]
    fn revert_2() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 0, 3, 2);
        assert_eq!(av, []);
        assert_eq!(bv, [1,2,3]);
        assert_eq!(cv, [4,5]);
    }

    #[test]
    fn revert_3() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 0, 5, 0);
        assert_eq!(av, []);
        assert_eq!(bv, [1,2,3,4,5]);
        assert_eq!(cv, []);
    }

    #[test]
    fn revert_4() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 0, 0, 0);
        assert_eq!(av, []);
        assert_eq!(bv, []);
        assert_eq!(cv, []);
    }
}

#[derive(Debug)]
pub struct Srdst<T> {
    pub source: Vec<T>,
    pub destination: Vec<T>,
}

impl<T> Srdst<T> {
    pub fn new(source: Vec<T>) -> Self {
        let destination: Vec<T> = Vec::with_capacity(source.capacity());
        Self { source, destination }
    }

    pub fn swap(&mut self) -> () {
        mem::swap(&mut self.source, &mut self.destination);
    }
}


// #[cfg(test)]
// #[cfg(not(test))]
pub fn chance() -> f32 {
    // rand::random::<f32>()
    0.0
}


pub struct SimpleTestCharacter {
    pub vision: Vision,
    pub timers: NaPressBurstTimers,
    pub na: Attack,
    pub press: Attack,
    pub burst: Attack,
}

impl SimpleTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Box<Self> {
        let mut boxed = Box::new(Self {
            vision,
            timers: CharacterTimersBuilder::new()
                .na(LoopTimer::new(2.0, 5))
                .press(DotTimer::single_hit(6.0))
                .burst(DotTimer::single_hit(12.0))
                .na_press_burst(),
            na: Attack {
                kind: AttackType::Na,
                element: ElementalGauge::physical(),
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        });
        boxed.na.icd_timer = &mut boxed.timers.na_icd;
        boxed.press.icd_timer = &mut boxed.timers.skill_icd;
        boxed.burst.icd_timer = &mut boxed.timers.burst_icd;
        boxed
    }
}

impl SpecialAbility for SimpleTestCharacter {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[*const Attack], _particles: &[Particle], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timers.update(gaurd, attack, owner_fc, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<Particle>, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if self.timers.burst_timer.is_active() {
            atk_queue.push(&self.burst);
        }
        if self.timers.press_timer.is_active() {
            atk_queue.push(&self.press);
            particles.push(Particle::new(self.press.element.aura, 2.0));
        }
        if self.timers.na_timer.is_active() && self.timers.na_timer.n() > 0 {
            atk_queue.push(&self.na);
        }
    }
}

impl CharacterAbility for SimpleTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .base_hp(100.0).base_atk(100.0).base_def(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn maybe_attack(&self, fc: &FieldCharacter) -> Option<AttackType> {
        self.timers.maybe_attack(fc, self)
    }
}

pub struct NoSkillTestCharacter {
    pub vision: Vision,
    pub timers: NaBurstTimers,
    pub na: Attack,
    pub burst: Attack,
}

impl NoSkillTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Box<Self> {
        let mut boxed = Box::new(Self {
            vision,
            timers: CharacterTimersBuilder::new()
                .na(LoopTimer::new(2.0, 5))
                .press(DotTimer::single_hit(6.0))
                .burst(DotTimer::single_hit(12.0))
                .na_burst(),
            na: Attack {
                kind: AttackType::Na,
                element: ElementalGauge::physical(),
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        });
        boxed.na.icd_timer = &mut boxed.timers.na_icd;
        boxed.burst.icd_timer = &mut boxed.timers.burst_icd;
        boxed
    }
}

impl SpecialAbility for NoSkillTestCharacter {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[*const Attack], _particles: &[Particle], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timers.update(gaurd, attack, owner_fc, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<Particle>, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if self.timers.burst_timer.is_active() {
            atk_queue.push(&self.burst);
        }
        if self.timers.na_timer.is_active() {
            atk_queue.push(&self.na);
        }
    }
}

impl CharacterAbility for NoSkillTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .base_hp(100.0).base_atk(100.0).base_def(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn maybe_attack(&self, fc: &FieldCharacter) -> Option<AttackType> {
        self.timers.maybe_attack(fc, self)
    }
}

pub struct CaTestCharacter {
    pub vision: Vision,
    pub timers: NaCaPressBurstTimers,
    pub na: Attack,
    pub ca: Attack,
    pub press: Attack,
    pub burst: Attack,
}

impl CaTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Box<Self> {
        let mut boxed = Box::new(Self {
            vision,
            timers: CharacterTimersBuilder::new()
                .na(LoopTimer::new(2.0, 5))
                .ca(HitsTimer::new(1.0, 2))
                .ca_stamina(StaminaTimer::new(200.0))
                .press(DotTimer::single_hit(6.0))
                .burst(DotTimer::single_hit(12.0))
                .na_ca_press_burst(),
            na: Attack {
                kind: AttackType::Na,
                element: ElementalGauge::physical(),
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca: Attack {
                kind: AttackType::Ca,
                element: ElementalGauge::physical(),
                multiplier: 150.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        });
        boxed.na.icd_timer = &mut boxed.timers.na_icd;
        boxed.ca.icd_timer = &mut boxed.timers.ca_icd;
        boxed.press.icd_timer = &mut boxed.timers.skill_icd;
        boxed.burst.icd_timer = &mut boxed.timers.burst_icd;
        boxed
    }
}

impl SpecialAbility for CaTestCharacter {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[*const Attack], _particles: &[Particle], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timers.update(gaurd, attack, owner_fc, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<Particle>, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if self.timers.burst_timer.is_active() {
            atk_queue.push(&self.burst);
        }
        if self.timers.press_timer.is_active() {
            atk_queue.push(&self.press);
            particles.push(Particle::new(self.press.element.aura, 2.0));
        }
        if self.timers.ca_timer.is_active() && self.timers.stamina.is_active() {
            match self.timers.ca_timer.n() {
                1 => atk_queue.push(&self.na),
                2 => atk_queue.push(&self.ca),
                _ => (),
            };
        }
        if self.timers.na_timer.is_active() {
            atk_queue.push(&self.na);
        }
    }
}

impl CharacterAbility for CaTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .base_hp(100.0).base_atk(100.0).base_def(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn maybe_attack(&self, fc: &FieldCharacter) -> Option<AttackType> {
        self.timers.maybe_attack(fc, self)
    }
}

pub struct PressHoldTestCharacter {
    use_hold: bool,
    pub vision: Vision,
    pub timers: NaPressHoldBurstTimers,
    pub na: Attack,
    pub press: Attack,
    pub hold: Attack,
    pub burst: Attack,
}

impl PressHoldTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Box<Self> {
        let mut boxed = Box::new(Self {
            use_hold: true,
            vision,
            timers: CharacterTimersBuilder::new()
                .na(LoopTimer::new(2.0, 5))
                .press(DotTimer::single_hit(1.0))
                .hold(DotTimer::single_hit(1.5))
                .burst(DotTimer::single_hit(12.0))
                .na_press_hold_burst(),
            na: Attack {
                kind: AttackType::Na,
                element: ElementalGauge::physical(),
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 250.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                element: ElementalGauge::new(vision, 1.0, A),
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        });
        boxed.na.icd_timer = &mut boxed.timers.na_icd;
        boxed.press.icd_timer = &mut boxed.timers.skill_icd;
        boxed.hold.icd_timer  = &mut boxed.timers.skill_icd;
        boxed.burst.icd_timer = &mut boxed.timers.burst_icd;
        boxed
    }
}

impl SpecialAbility for PressHoldTestCharacter {
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[*const Attack], _particles: &[Particle], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        if gaurd.kind == AttackType::HoldSkill {
            self.use_hold = false;
        }
        self.timers.update(gaurd, attack, owner_fc, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<Particle>, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if self.timers.burst_timer.is_active() {
            atk_queue.push(&self.burst);
        }
        if self.timers.press_timer.is_active() {
            atk_queue.push(&self.press);
            particles.push(Particle::new(self.press.element.aura, 2.0));
        }
        if self.timers.hold_timer.is_active() {
            atk_queue.push(&self.hold);
            particles.push(Particle::new(self.hold.element.aura, 3.0));
        }
        if self.timers.na_timer.is_active() {
            atk_queue.push(&self.na);
        }
    }
}

impl CharacterAbility for PressHoldTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .base_hp(100.0).base_atk(100.0).base_def(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn use_hold(&self) -> bool {
        self.use_hold
    }

    fn maybe_attack(&self, fc: &FieldCharacter) -> Option<AttackType> {
        self.timers.maybe_attack(fc, self)
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

    pub fn fc(state: State) -> FieldCharacterData {
        TestEnvironment::fc_n(FieldCharacterIndex(0), state)
    }

    pub fn fc1(state: State) -> FieldCharacterData {
        TestEnvironment::fc_n(FieldCharacterIndex(1), state)
    }

    pub fn fc_n(idx: FieldCharacterIndex, state: State) -> FieldCharacterData {
        TestEnvironment::vision(idx, state, Pyro)
    }

    pub fn vision(idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData {
        let ca = SimpleTestCharacter::new(idx, vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
            ca, Box::new(wa), Box::new(aa)
        )
    }

    pub fn ca(idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData {
        let ca = CaTestCharacter::new(idx, vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
            ca, Box::new(wa), Box::new(aa)
        )
    }

    pub fn hold(idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData {
        let ca = PressHoldTestCharacter::new(idx, vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
            ca, Box::new(wa), Box::new(aa)
        )
    }

    pub fn artifact<T: 'static + ArtifactAbility>(idx: FieldCharacterIndex, vision: Vision, aa: T) -> FieldCharacterData {
        let ca = SimpleTestCharacter::new(idx, vision);
        let wa = TestWeapon;
        FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
            ca, Box::new(wa), Box::new(aa)
        )
    }

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

    pub fn no_skill(idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData {
        let ca = NoSkillTestCharacter::new(idx, vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        FieldCharacter::new(idx, ca.record(), vision, wa.record(), aa.record()).to_data(
            ca, Box::new(wa), Box::new(aa)
        )
    }
}
