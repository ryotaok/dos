use std::rc::Rc;
use std::cell::RefCell;
// use rand::prelude::*;

use crate::state::State;
use crate::artifact::Artifact;

use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, Enemy, FieldAbility, SpecialAbility, CharacterAbility, SkillAbility, NoopAbility, NoopSkillAbility, CharacterData};
use crate::types::{AttackType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE};
use crate::action::{Attack, AttackEvent, ICDTimer, NaLoop, NTimer, ICDTimers};

use Vision::*;

// #[cfg(test)]
// #[cfg(not(test))]
pub fn chance() -> f32 {
    // rand::random::<f32>()
    0.0
}

impl NaLoop {
    fn test(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        NaLoop::new(
            &[0.4, 0.4, ],
            vec![Attack {
                kind: AttackType::Na,
                element: &PHYSICAL_GAUGE,
                multiplier: 100.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.na),
                idx,
            }, Attack {
                kind: AttackType::Na,
                element: &PHYSICAL_GAUGE,
                multiplier: 100.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.na),
                idx,
            },]
        )
    }
}

#[derive(Debug)]
pub struct TestSkill {
    timer: NTimer,
    attack: Attack,
}

impl TestSkill {
    pub fn new(idx: FieldCharacterIndex, vision: &Vision, icd_timer: &ICDTimers) -> Self {
        Self {
            timer: NTimer::new(&[6.0]),
            attack: Attack {
                kind: AttackType::PressSkill,
                element: vision.to_gauge(),
                multiplier: 200.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }
        }
    }
}

impl SkillAbility for TestSkill {
    fn accelerate(&mut self, _f: fn(&mut NTimer)) -> () {}
}

impl SpecialAbility for TestSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer)
        // if self.timer.n == 0 {
        //     Some(AttackEvent {
        //         kind: AttackType::Skill,
        //         idx: self.attack.idx,
        //     })
        // } else {
        //     None
        // }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event == &self.attack);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
                particles.push_p(Particle::new(Pyro, 2.0));
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct TestHoldSkill {
    use_hold: bool,
    press_timer: NTimer,
    hold_timer: NTimer,
    press: Attack,
    hold: Attack,
}

impl TestHoldSkill {
    pub fn new(idx: FieldCharacterIndex, vision: &Vision, icd_timer: &ICDTimers) -> Self {
        Self {
            use_hold: true,
            press_timer: NTimer::new(&[1.0]),
            hold_timer: NTimer::new(&[1.5]),
            press: Attack {
                kind: AttackType::PressSkill,
                element: vision.to_gauge(),
                multiplier: 200.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                element: vision.to_gauge(),
                multiplier: 250.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
        }
    }
}

impl SkillAbility for TestHoldSkill {
    fn accelerate(&mut self, _f: fn(&mut NTimer)) -> () {}
}

impl SpecialAbility for TestHoldSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        if self.use_hold {
            self.hold.to_event(&self.hold_timer)
        } else {
            self.press.to_event(&self.press_timer)
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.hold_timer.update(time, event == &self.hold);
        self.press_timer.update(time, event == &self.press);
        // when hold cd ends
        if self.hold_timer.ping && self.hold_timer.n == 0 {
            self.use_hold = false;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.hold_timer.ping, self.hold_timer.n, self.press_timer.ping, self.press_timer.n) {
            (true, 1, _, _) => {
                atk_queue.push(&self.hold);
                particles.push_p(Particle::new(Pyro, 3.0));
            },
            (_, _, true, 1) => {
                atk_queue.push(&self.press);
                particles.push_p(Particle::new(Pyro, 2.0));
            },
            _ => (),
        }
    }
}

// #[derive(Debug)]
pub struct TestBurst {
    timer: NTimer,
    attack: Attack,
}

impl TestBurst {
    pub fn new(idx: FieldCharacterIndex, vision: &Vision, icd_timer: &ICDTimers) -> Self {
        Self {
            timer: NTimer::new(&[12.0]),
            attack: Attack {
                kind: AttackType::Burst,
                element: vision.to_gauge(),
                multiplier: 300.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }
        }
    }
}

impl SpecialAbility for TestBurst {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        // if data.can_burst() && (self.timer.n == 0 || self.timer.ping) {
        //     Some(AttackEvent {
        //         kind: AttackType::Burst,
        //         idx: self.attack.idx,
        //     })
        // } else {
        //     None
        // }
        if data.can_burst() {
            self.attack.to_event(&self.timer)
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event == &self.attack);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
            },
            _ => (),
        }
    }
}

pub struct TestCharacter {
    pub na: NaLoop,
    pub ca: NoopAbility,
    pub skill: TestSkill,
    pub burst: TestBurst,
}

impl SpecialAbility for TestCharacter {}
impl CharacterAbility for TestCharacter {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl TestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: &Vision, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::test(idx, icd_timer),
            ca: NoopAbility,
            skill: TestSkill::new(idx, vision, icd_timer),
            burst: TestBurst::new(idx, vision, icd_timer),
        }
    }

    pub fn record(vision: Vision) -> CharacterRecord {
        CharacterRecord::default()
            .vision(vision)
            .base_atk(100.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }
}

pub struct NoSkillTestCharacter {
    pub na: NaLoop,
    pub ca: NoopAbility,
    pub skill: NoopSkillAbility,
    pub burst: TestBurst,
}

impl NoSkillTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: &Vision, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::test(idx, icd_timer),
            ca: NoopAbility,
            skill: NoopSkillAbility,
            burst: TestBurst::new(idx, vision, icd_timer),
        }
    }
}

impl SpecialAbility for NoSkillTestCharacter {}
impl CharacterAbility for NoSkillTestCharacter {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

pub struct HoldTestCharacter {
    pub na: NaLoop,
    pub ca: NoopAbility,
    pub skill: TestHoldSkill,
    pub burst: TestBurst,
}

impl HoldTestCharacter {
    pub fn new(idx: FieldCharacterIndex, vision: &Vision, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::test(idx, icd_timer),
            ca: NoopAbility,
            skill: TestHoldSkill::new(idx, vision, icd_timer),
            burst: TestBurst::new(idx, vision, icd_timer),
        }
    }
}

impl SpecialAbility for HoldTestCharacter {}
impl CharacterAbility for HoldTestCharacter {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

// pub struct CaTestCharacter {
//     pub vision: Vision,
//     pub na: Attack,
//     pub ca: Attack,
//     pub press: Attack,
//     pub burst: Attack,
// }

// impl CaTestCharacter {
//     pub fn new(idx: FieldCharacterIndex, vision: Vision) -> Self {
//         Self {
//             vision,
//             na: Attack {
//                 kind: AttackType::Na,
//                 gauge: &GAUGE1A,
//                 multiplier: 100.0,
//                 hits: 1,
//                 icd_timer: Rc::clone(icd_timer),
//                 idx,
//             },
//             ca: Attack {
//                 kind: AttackType::Ca,
//                 gauge: &GAUGE1A,
//                 multiplier: 150.0,
//                 hits: 1,
//                 icd_timer: Rc::clone(icd_timer),
//                 idx,
//             },
//             press: Attack {
//                 kind: AttackType::PressSkill,
//                 gauge: &GAUGE1A,
//                 multiplier: 200.0,
//                 hits: 1,
//                 icd_timer: Rc::clone(icd_timer),
//                 idx,
//             },
//             burst: Attack {
//                 kind: AttackType::Burst,
//                 gauge: &GAUGE1A,
//                 multiplier: 300.0,
//                 hits: 1,
//                 icd_timer: Rc::clone(icd_timer),
//                 idx,
//             },
//         }
//     }
// }

// SpecialAbility CharacterAbility for CaTestCharacter {}
//     pub fn record(&self) -> CharacterRecord {
//         CharacterRecord::default()
//             .vision(self.vision)
//             .base_atk(100.0)
//             .cr(0.0).cd(0.0)
//             .energy_cost(40.0)
//     }

//     fn timers(&self) -> FullCharacterTimers {
//         CharacterTimersBuilder::new()
//             .na(LoopTimer::new(2.0, 5))
//             .ca(HitsTimer::new(2.0, 1))
//             .stamina(StaminaTimer::new(200.0))
//             .press(DotTimer::single_hit(6.0))
//             .burst(DotTimer::single_hit(12.0))
//             .build()
//     }

//     fn init(&mut self, timers: &mut FullCharacterTimers, data: &CharacterData) -> () {
//         self.na.icd_timer = &mut timers.na_icd;
//         self.ca.icd_timer = &mut timers.ca_icd;
//         self.press.icd_timer = &mut timers.skill_icd;
//         self.burst.icd_timer = &mut timers.burst_icd;
//     }

//     fn use_ca(&self) -> bool {
//         true
//     }
// }

// impl SpecialAbility for CaTestCharacter {
//     fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, owner_fc: &CharacterData, _enemy: &Enemy) -> () {
//         if timers.burst_timer().is_active() {
//             atk_queue.push(ElementalAttack::new(self.vision, &self.burst));
//         }
//         if timers.press_timer().is_active() {
//             atk_queue.push(ElementalAttack::new(self.vision, &self.press));
//             particles.push_p(Particle::new(Pyro, 2.0));
//         }
//         if timers.ca_timer().is_active() {
//             atk_queue.push(ElementalAttack::new(self.vision, &self.na));
//             atk_queue.push(ElementalAttack::new(self.vision, &self.ca));
//         }
//         if timers.na_timer().is_active() && timers.na_timer().n() > 0 {
//             if owner_fc.state.infusion {
//                 atk_queue.push(ElementalAttack::new(self.vision, &self.na));
//             } else {
//                 atk_queue.push(ElementalAttack::physical(&self.na));
//             }
//         }
//     }
// }

pub struct TestWeapon;

impl SpecialAbility for TestWeapon {}

impl TestWeapon {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
    }
}

pub struct TestArtifact;

impl TestArtifact {
    pub fn record(state: State) -> Artifact {
        Artifact {
            name: "simple",
            version: 1.0,
            preference: &[],
            state
        }
    }
}

impl SpecialAbility for TestArtifact {}

pub struct TestEnvironment {
    pub timers: ICDTimers,
    pub testcharacter: Option<TestCharacter>,
    pub noskilltestcharacter: Option<NoSkillTestCharacter>,
    // pub catestcharacter: Option<CaTestCharacter>,
    pub holdtestcharacter: Option<HoldTestCharacter>,
    // pub dotskillcharacter: Option<DotSkillCharacter>,
    pub weapon: TestWeapon,
    pub artifact: TestArtifact,
    pub cr: Option<CharacterRecord>,
    pub wr: Option<WeaponRecord>,
    pub ar: Option<Artifact>,
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self {
            timers: ICDTimers::new(),
            testcharacter: None,
            noskilltestcharacter: None,
            // catestcharacter: None,
            holdtestcharacter: None,
            // dotskillcharacter: None,
            weapon: TestWeapon,
            artifact: TestArtifact,
            cr: None,
            wr: None,
            ar: None,
        }
    }

    pub fn enemy() -> Enemy {
        Enemy::simple()
    }

/*
    let mut env = TestEnvironment::new();
    env.vision(&mut members, &mut state, &mut abilities, State::new(), Pyro);

    pub fn vision(&mut self, members: &mut Vec<CharacterData>, abilities: &mut Vec<FieldAbility>, state: State, vision: Vision) -> () {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        members.push(data);
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision));
        abilities.push(self.builder
            .na(&mut ca.na)
            .skill(&mut ca.skill)
            .burst(&mut ca.burst)
            .build(&mut self.timers)
        );
    }
*/

    pub fn vision(&mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> (CharacterData, FieldAbility) {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision, &self.timers));
        (data, FieldAbility {
            timers: &mut self.timers,
            character: ca,
            weapon: &mut self.weapon,
            artifact: &mut self.artifact,
        })
    }

    pub fn no_skill(&mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> (CharacterData, FieldAbility) {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.noskilltestcharacter.insert(NoSkillTestCharacter::new(idx, vision, &self.timers));
        (data, FieldAbility {
            timers: &mut self.timers,
            character: ca,
            weapon: &mut self.weapon,
            artifact: &mut self.artifact,
        })
    }

    pub fn hold(&mut self, idx: FieldCharacterIndex, state: State, vision: Vision) -> (CharacterData, FieldAbility) {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.holdtestcharacter.insert(HoldTestCharacter::new(idx, vision, &self.timers));
        (data, FieldAbility {
            timers: &mut self.timers,
            character: ca,
            weapon: &mut self.weapon,
            artifact: &mut self.artifact,
        })
    }

    pub fn artifact<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision, aa: &'a mut dyn SpecialAbility) -> (CharacterData, FieldAbility<'a>) {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision, &self.timers));
        (data, FieldAbility {
            timers: &mut self.timers,
            character: ca,
            weapon: &mut self.weapon,
            artifact: aa,
        })
    }

    pub fn weapon<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision, wa: &'a mut dyn SpecialAbility) -> (CharacterData, FieldAbility<'a>) {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.testcharacter.insert(TestCharacter::new(idx, vision, &self.timers));
        (data, FieldAbility {
            timers: &mut self.timers,
            character: ca,
            weapon: wa,
            artifact: &mut self.artifact,
        })
    }

    pub fn no_skill_weapon<'a>(&'a mut self, idx: FieldCharacterIndex, state: State, vision: Vision, wa: &'a mut dyn SpecialAbility) -> (CharacterData, FieldAbility<'a>) {
        self.cr.insert(TestCharacter::record(vision));
        self.wr.insert(TestWeapon::record());
        self.ar.insert(TestArtifact::record(state));
        let data = CharacterData::new(idx, self.cr.as_ref().unwrap(), self.wr.as_ref().unwrap(), self.ar.as_ref().unwrap());
        // 
        let vision = &self.cr.as_ref().unwrap().vision;
        let ca = self.noskilltestcharacter.insert(NoSkillTestCharacter::new(idx, vision, &self.timers));
        (data, FieldAbility {
            timers: &mut self.timers,
            character: ca,
            weapon: wa,
            artifact: &mut self.artifact,
        })
    }
}
