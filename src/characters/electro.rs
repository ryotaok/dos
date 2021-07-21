use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B, GAUGE4C};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer};
// StaminaTimer

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Beidou {
    skill_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Beidou {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a4: DurationTimer::new(0.0, 10.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 140.59,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 140.08,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 174.59,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 171.02,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 221.68,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: (218.88 + 288.0) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE4C,
                multiplier: 218.88,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 172.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Beidou {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Beidou").vision(Electro).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(13050.0).base_atk(225.0).base_def(648.0)
            .dmg_electro(24.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(3.75, 5))
            .press(DotTimer::single_hit(7.5))
            .burst(DotTimer::new(20.0, 1.0, 10))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Beidou {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let skill = guard.kind == PressSkill;
        self.skill_a4.update(guard.second(skill), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::electro(&self.burst));
                atk_queue.push(ElementalAttack::electro(&self.burst_dot));
            } else {
                atk_queue.push(ElementalAttack::electro(&self.burst_dot));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::electro(&self.press));
            particles.push(Particle::new(Electro, 2.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_electro(data, &self.na_1),
                2 => atk_queue.push_electro(data, &self.na_2),
                3 => atk_queue.push_electro(data, &self.na_3),
                4 => atk_queue.push_electro(data, &self.na_4),
                5 => atk_queue.push_electro(data, &self.na_5),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.skill_a4.is_active() {
            let state = &mut modifiable_state[data.idx.0];
            state.na_dmg += 15.0;
            state.ca_dmg += 15.0;
            state.atk_spd += 15.0;
        }
    }

    // TODO inaccurate
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if attack.kind == PressSkill {
            Some(State::new().skill_dmg(20.0))
        } else {
            None
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
    }
}

pub struct Fischl {
    ca_a1_timer: HitsTimer,
    aa_a4_timer: HitsTimer,
    oz_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
    ca_a1: Attack,
    aa_a4: Attack,
}

impl Fischl {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            ca_a1_timer: HitsTimer::new(1.0, 1),
            aa_a4_timer: HitsTimer::new(1.0, 1),
            oz_timer: DurationTimer::new(25.0, 12.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 87.21,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 92.48,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 114.92,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 114.07,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 142.46,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 207.79,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 159.84,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: 374.4,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_a1: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 152.7,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            aa_a4: Attack {
                kind: AttackType::SkillDot, // TODO inaccurate
                gauge: &GAUGE1A,
                multiplier: 80.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Fischl {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Fischl").vision(Electro).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(9189.0).base_atk(244.0).base_def(594.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.1, 5))
            // TODO CA
            .press(DotTimer::new(25.0, 1.0, 12))
            .burst(DotTimer::single_hit(15.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.press_dot.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.ca_a1.icd_timer = &mut timers.ca_icd;
        self.aa_a4.icd_timer = &mut timers.skill_icd;
    }

    // TODO CharacterAbility::accelerate could be &mut self
    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        if timers.burst_timer().is_active() {
            timers.reset_cd();
        }
    }
}

impl SpecialAbility for Fischl {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let electro_er = attack.iter().any(|a| enemy.trigger_er(&a.element).is_electro());
        self.oz_timer.update(guard.second(guard.kind == PressSkill), time);
        self.ca_a1_timer.update(guard.second(guard.kind == Ca), time);
        self.aa_a4_timer.update(guard.second(electro_er), time);
        if guard.kind == Burst {
            self.oz_timer.reset();
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::electro(&self.press));
                atk_queue.push(ElementalAttack::electro(&self.press_dot));
                particles.push(Particle::new(Electro, 1.0));
            } else {
                atk_queue.push(ElementalAttack::electro(&self.press_dot));
                particles.push(Particle::new(Electro, 1.0));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_electro(data, &self.na_1),
                2 => atk_queue.push_electro(data, &self.na_2),
                3 => atk_queue.push_electro(data, &self.na_3),
                4 => atk_queue.push_electro(data, &self.na_4),
                5 => atk_queue.push_electro(data, &self.na_5),
                _ => (),
            };
        }
        if self.oz_timer.is_active() && self.ca_a1_timer.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.ca_a1));
        }
        if self.oz_timer.is_active() && self.aa_a4_timer.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.aa_a4));
        }
    }

    fn reset(&mut self) -> () {
        self.ca_a1_timer.reset();
        self.aa_a4_timer.reset();
        self.oz_timer.reset();
    }
}

pub struct Lisa {
    conductive_status: usize,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    hold_0: Attack,
    hold_1: Attack,
    hold_2: Attack,
    hold_3: Attack,
    burst_dot: Attack,
}

impl Lisa {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            conductive_status: 0,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 71.28,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 64.66,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 77.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 98.93,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 144.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold_0: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE2B,
                multiplier: 576.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold_1: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE2B,
                multiplier: 662.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold_2: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE2B,
                multiplier: 763.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold_3: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE2B,
                multiplier: 876.96,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 65.81,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Lisa {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Lisa").vision(Electro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(9570.0).base_atk(232.0).base_def(573.0)
            .em(96.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.5, 4))
            .press(DotTimer::single_hit(1.0))
            .hold(DotTimer::single_hit(16.0))
            .burst(DotTimer::new(20.0, 0.5555, 28))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.hold_0.icd_timer = &mut timers.skill_icd;
        self.hold_1.icd_timer = &mut timers.skill_icd;
        self.hold_2.icd_timer = &mut timers.skill_icd;
        self.hold_3.icd_timer = &mut timers.skill_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }

    fn use_hold(&self) -> bool {
        self.conductive_status == 3
    }
}

impl SpecialAbility for Lisa {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, _time: f32) -> () {
        if guard.kind == PressSkill {
            self.conductive_status += 1;
        }
        if guard.kind == HoldSkill {
            self.conductive_status = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.burst_dot));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::electro(&self.press));
            particles.push(Particle::new(Electro, 1.0));
        }
        if timers.hold_timer().is_active() {
            match self.conductive_status {
                0 => atk_queue.push(ElementalAttack::electro(&self.hold_0)),
                1 => atk_queue.push(ElementalAttack::electro(&self.hold_1)),
                2 => atk_queue.push(ElementalAttack::electro(&self.hold_2)),
                3 => atk_queue.push(ElementalAttack::electro(&self.hold_3)),
                _ => atk_queue.push(ElementalAttack::electro(&self.hold_3)),
            };
            particles.push(Particle::new(Electro, 5.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(ElementalAttack::electro(&self.na_1)),
                2 => atk_queue.push(ElementalAttack::electro(&self.na_2)),
                3 => atk_queue.push(ElementalAttack::electro(&self.na_3)),
                4 => atk_queue.push(ElementalAttack::electro(&self.na_4)),
                _ => (),
            };
        }
    }

    // a4
    fn modify(&self, _modifiable_state: &mut [State], timers: &FullCharacterTimers, _data: &CharacterData, enemy: &mut Enemy) -> () {
        if timers.burst_timer().is_active() {
            enemy.def_down_debuff.push(Debuff::lisa_a4());
        }
    }

    fn reset(&mut self) -> () {
        self.conductive_status = 0;
    }
}

pub struct Razor {
    burst_timer: DurationTimer,
    burst_aa: HitsTimer,
    electro_sigil: usize,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    hold: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Razor {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(20.0, 15.0),
            burst_aa: HitsTimer::new(0.001, 1),
            electro_sigil: 0,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 171.13,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 147.42,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 184.32,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 242.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 358.56,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE2B,
                multiplier: 531.36,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: 288.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 43.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Razor {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Razor").vision(Electro).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(11962.0).base_atk(234.0).base_def(751.0)
            .dmg_phy(30.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.734, 4))
            // a1
            .press(DotTimer::single_hit(6.0 * 0.82))
            .hold(DotTimer::single_hit(10.0 * 0.82))
            .burst(DotTimer::single_hit(20.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.hold.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }

    // a1 TODO fix
    // fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
    //     if timers.burst_timer().is_active() {
    //         timers.reset_cd();
    //     }
    // }

    fn use_hold(&self) -> bool {
        self.electro_sigil == 3
    }
}

impl SpecialAbility for Razor {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.second(guard.kind == Burst), time);
        self.burst_aa.update(guard.second(guard.kind == Na), time);
        if guard.kind == PressSkill {
            self.electro_sigil += 1;
        }
        if guard.kind == HoldSkill {
            self.electro_sigil = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.burst));
        }
        if self.burst_timer.is_active() && self.burst_aa.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.burst_dot));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::electro(&self.press));
            particles.push(Particle::new(Electro, 4.0));
        }
        if timers.hold_timer().is_active() {
            atk_queue.push(ElementalAttack::electro(&self.hold));
            particles.push(Particle::new(Electro, 5.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_electro(data, &self.na_1),
                2 => atk_queue.push_electro(data, &self.na_2),
                3 => atk_queue.push_electro(data, &self.na_3),
                4 => atk_queue.push_electro(data, &self.na_4),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let mut state = &mut modifiable_state[data.idx.0];
        if self.burst_timer.is_active() {
            state.atk_spd += 40.0;
        }
        state.er += 20.0 * self.electro_sigil as f32;
        if timers.hold_timer().is_active() {
            state.energy.0 += 5.0 * self.electro_sigil as f32;
        }
        // a4
        if data.state.energy.0 / data.state.energy_cost <= 0.5 {
            state.er += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.burst_aa.reset();
        self.electro_sigil = 0;
    }
}

pub struct Keqing {
    skill_timer: DurationTimer,
    burst_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press_lightning_siletto: Attack,
    press_slashing: Attack,
    burst: Attack,
    burst_consecutive_slash: Attack,
    burst_last_attack: Attack,
}

impl Keqing {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_timer: DurationTimer::new(7.5, 5.0),
            burst_a4: DurationTimer::new(12.0, 8.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 81.09,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 81.09,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 107.61,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (62.22+68.0) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 132.43,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_lightning_siletto: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 90.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_slashing: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 302.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 158.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_consecutive_slash: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 43.2,
                hits: 8,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_last_attack: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 339.84,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Keqing {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Keqing").vision(Electro).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(13103.0).base_atk(323.0).base_def(799.0)
            .cd(88.4)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.017, 5))
            // TODO CA
            .press(DotTimer::single_hit(7.5))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.press_lightning_siletto.icd_timer = &mut timers.skill_icd;
        self.press_slashing.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_consecutive_slash.icd_timer = &mut timers.burst_icd;
        self.burst_last_attack.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Keqing {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.skill_timer.update(guard.second(guard.kind == PressSkill), time);
        self.burst_a4.update(guard.second(guard.kind == Burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.burst));
            atk_queue.push(ElementalAttack::electro(&self.burst_consecutive_slash));
            atk_queue.push(ElementalAttack::electro(&self.burst_last_attack));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::electro(&self.press_lightning_siletto));
            atk_queue.push(ElementalAttack::electro(&self.press_slashing));
            particles.push(Particle::new(Electro, 2.5));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_electro(data, &self.na_1),
                2 => atk_queue.push_electro(data, &self.na_2),
                3 => atk_queue.push_electro(data, &self.na_3),
                4 => atk_queue.push_electro(data, &self.na_4),
                5 => atk_queue.push_electro(data, &self.na_5),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.skill_timer.is_active() {
            state.infusion = true;
        }
        if self.burst_a4.is_active() {
            state.cr += 15.0;
            state.er += 15.0;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        self.burst_a4.reset();
    }
}
