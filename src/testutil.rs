use std::ptr;
// use rand::prelude::*;

use crate::state::State;
use crate::artifact::Artifact;

use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, Enemy, FieldCharacter, SpecialAbility, CharacterAbility, WeaponAbility, ArtifactAbility, FieldCharacterData, CharacterData};
use crate::types::{AttackType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, CharacterTimersBuilder, LoopTimer, DotTimer, HitsTimer, StaminaTimer};

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
                gauge: &GAUGE1A,
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
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
            .vision(self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for TestCharacter {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.press));
            particles.push_p(Particle::new(Pyro, 2.0));
        }
        if timers.na_timer().is_active() {
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
                gauge: &GAUGE1A,
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
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
            .vision(self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na.icd_timer = &mut timers.na_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for NoSkillTestCharacter {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.na_timer().is_active() && timers.na_timer().n() > 0 {
            if owner_fc.state.infusion {
                atk_queue.push(ElementalAttack::new(self.vision, &self.na));
            } else {
                atk_queue.push(ElementalAttack::physical(&self.na));
            }
        }
    }
}

pub struct CaTestCharacter {
    pub vision: Vision,
    pub na: Attack,
    pub ca: Attack,
    pub press: Attack,
    pub burst: Attack,
}

impl CaTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Self {
        Self {
            vision,
            na: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 150.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for CaTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .vision(self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .ca(HitsTimer::new(2.0, 1))
            .stamina(StaminaTimer::new(200.0))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na.icd_timer = &mut timers.na_icd;
        self.ca.icd_timer = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }

    fn use_ca(&self) -> bool {
        true
    }
}

impl SpecialAbility for CaTestCharacter {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.press));
            particles.push_p(Particle::new(Pyro, 2.0));
        }
        if timers.ca_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.na));
            atk_queue.push(ElementalAttack::new(self.vision, &self.ca));
        }
        if timers.na_timer().is_active() && timers.na_timer().n() > 0 {
            if owner_fc.state.infusion {
                atk_queue.push(ElementalAttack::new(self.vision, &self.na));
            } else {
                atk_queue.push(ElementalAttack::physical(&self.na));
            }
        }
    }
}

pub struct HoldTestCharacter {
    use_hold: bool,
    pub vision: Vision,
    pub na: Attack,
    pub press: Attack,
    pub hold: Attack,
    pub burst: Attack,
}

impl HoldTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Self {
        Self {
            use_hold: true,
            vision,
            na: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE1A,
                multiplier: 250.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for HoldTestCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .vision(self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .press(DotTimer::single_hit(1.0))
            .hold(DotTimer::single_hit(1.5))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.hold.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }

    fn use_hold(&self) -> bool {
        self.use_hold
    }
}

impl SpecialAbility for HoldTestCharacter {
    fn update(&mut self, _guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, _time: f32) -> () {
        self.use_hold = false;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.press));
            particles.push_p(Particle::new(Pyro, 2.0));
        }
        if timers.hold_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.hold));
            particles.push_p(Particle::new(Pyro, 3.0));
        }
        if timers.na_timer().is_active() && timers.na_timer().n() > 0 {
            if owner_fc.state.infusion {
                atk_queue.push(ElementalAttack::new(self.vision, &self.na));
            } else {
                atk_queue.push(ElementalAttack::physical(&self.na));
            }
        }
    }
}

pub struct DotSkillCharacter {
    pub vision: Vision,
    pub na: Attack,
    pub press: Attack,
    pub burst: Attack,
}

impl DotSkillCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Self {
        Self {
            vision,
            na: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 100.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 50.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 300.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for DotSkillCharacter {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .vision(self.vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .press(DotTimer::new(6.0, 0.5, 4))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for DotSkillCharacter {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::new(self.vision, &self.press));
            particles.push_p(Particle::new(Pyro, 1.0));
        }
        if timers.na_timer().is_active() {
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

pub struct TestWeaponAbility<T: WeaponAbility>(pub T);

impl<T: WeaponAbility> SpecialAbility for TestWeaponAbility<T> {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }

    fn intensify(&self, attack: &Attack) -> Option<State> {
        self.0.intensify(attack)
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

impl<T: WeaponAbility> WeaponAbility for TestWeaponAbility<T> {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
    }

    fn accelerate(&self, ac: &mut FullCharacterTimers) -> () {
        self.0.accelerate(ac);
    }
}

pub struct TestArtifact(pub State);

impl SpecialAbility for TestArtifact {}

impl ArtifactAbility for TestArtifact {
    fn record(&self) -> Artifact {
        let mut state = State::new();
        state.merge(&self.0);
        Artifact {
            name: "simple",
            version: 1.0,
            preference: Vec::new(),
            state
        }
    }
}

pub struct TestEnvironment {
    timers: Box<FullCharacterTimers>,
    testcharacter: Option<TestCharacter>,
    noskilltestcharacter: Option<NoSkillTestCharacter>,
    catestcharacter: Option<CaTestCharacter>,
    holdtestcharacter: Option<HoldTestCharacter>,
    dotskillcharacter: Option<DotSkillCharacter>,
    testweapon: Option<TestWeapon>,
    testartifact: Option<TestArtifact>,
    cr: Option<CharacterRecord>,
    wr: Option<WeaponRecord>,
    ar: Option<Artifact>,
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self {
            timers: Box::new(CharacterTimersBuilder::new().build()),
            testcharacter: None,
            noskilltestcharacter: None,
            catestcharacter: None,
            holdtestcharacter: None,
            dotskillcharacter: None,
            testweapon: None,
            testartifact: None,
            cr: None,
            wr: None,
            ar: None,
        }
    }

    pub fn enemy() -> Enemy {
        Enemy::simple()
    }

    pub fn vision<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<'a> {
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision));
        let wa = self.testweapon.insert(TestWeapon);
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }

    pub fn no_skill<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<'a> {
        let ca = self.noskilltestcharacter.insert(NoSkillTestCharacter::new(idx, vision));
        let wa = self.testweapon.insert(TestWeapon);
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }

    pub fn ca<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<'a> {
        let ca = self.catestcharacter.insert(CaTestCharacter::new(idx, vision));
        let wa = self.testweapon.insert(TestWeapon);
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }

    pub fn hold<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<'a> {
        let ca = self.holdtestcharacter.insert(HoldTestCharacter::new(idx, vision));
        let wa = self.testweapon.insert(TestWeapon);
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }

    pub fn dot<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> FieldCharacterData<'a> {
        let ca = self.dotskillcharacter.insert(DotSkillCharacter::new(idx, vision));
        let wa = self.testweapon.insert(TestWeapon);
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }

    pub fn artifact<'a, T: 'static + ArtifactAbility>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision, aa: &'a mut T) -> FieldCharacterData<'a> {
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision));
        let wa = self.testweapon.insert(TestWeapon);
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        ar.state.clear();
        ar.state.merge(&state);
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))

    }

    pub fn weapon<'a, T: 'static + WeaponAbility>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision, wa: &'a mut T) -> FieldCharacterData<'a> {
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision));
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }

    pub fn no_skill_weapon<'a, T: 'static + WeaponAbility>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision, wa: &'a mut T) -> FieldCharacterData<'a> {
        let ca = self.noskilltestcharacter.insert(NoSkillTestCharacter::new(idx, vision));
        let aa = self.testartifact.insert(TestArtifact(state));
        let cr = self.cr.insert(ca.record());
        let wr = self.wr.insert(wa.record());
        let ar = self.ar.insert(aa.record());
        FieldCharacterData::new(&mut self.timers, ca, wa, aa, CharacterData::new(idx, cr, wr, ar))
    }
}
