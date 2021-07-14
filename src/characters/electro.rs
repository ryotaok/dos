use std::mem;
use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, NormalAttackAction, SkillAction, BurstAction};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.0

pub struct Beidou {
    burst_aa: DotTimer,
    skill_a4: DurationTimer,
}

impl Beidou {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 1.0, 10),
            skill_a4: DurationTimer::new(0.0, 10.0),
        }
    }
}

impl SpecialAbility for Beidou {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Beidou").vision("Electro").weapon("Claymore").release_date("2020-09-28").version(1.0)
            .base_hp(13050.0).base_atk(225.0).base_def(648.0)
            .dmg_electro(24.0)
            .na_1(140.59).na_2(140.08).na_3(174.59).na_4(171.02).na_5(221.68).na_time(3.75)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(1.9)
            .press_cd(7.5).press_particle(2.0).press_dmg(218.88 + 288.0)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(218.88)
            .skill_unit(2.0).skill_decay(B).burst_unit(4.0).burst_decay(C)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let mut skill = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Skill => skill = true,
                Burst => burst = true,
                _ => (),
            };
        }
        self.burst_aa.update(gaurd.second(burst), time);
        self.skill_a4.update(gaurd.second(skill), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Electro,
                multiplier: 172.8,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.skill_a4.is_active() {
            let state = &mut modifiable_state[owner_fc.idx.0];
            state.na_dmg += 15.0;
            state.ca_dmg += 15.0;
            state.atk_spd += 15.0;
        }
    }

    // TODO inaccurate
    fn intensify(&self, attack: &mut Attack, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if attack.kind == Skill {
            let mut state: Option<State> = None;
            mem::swap(&mut state, &mut attack.state);
            attack.state = if let Some(mut state) = state {
                state.skill_dmg += 20.0;
                Some(state)
            } else {
                Some(State::new().skill_dmg(20.0))
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.skill_a4.reset();
    }
}

pub struct Fischl {
    skill_aa: DotTimer,
    burst_timer: HitsTimer,
    ca_a1: HitsTimer,
    aa_a4: HitsTimer,
}

impl Fischl {
    pub fn new() -> Self {
        Self {
            skill_aa: DotTimer::new(25.0, 1.0, 12),
            burst_timer: HitsTimer::new(0.001, 1),
            ca_a1: HitsTimer::new(1.0, 1),
            aa_a4: HitsTimer::new(1.0, 1),
        }
    }
}

impl SpecialAbility for Fischl {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Fischl").vision("Electro").weapon("Bow").release_date("2020-09-28").version(1.0)
            .base_hp(9189.0).base_atk(244.0).base_def(594.0)
            .atk(24.0)
            .na_1(87.21).na_2(92.48).na_3(114.92).na_4(114.07).na_5(142.46).na_6(0.0).na_time(2.1)
            // TODO
            .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(2.0)
            .press_cd(25.0).press_particle(0.0).press_dmg(207.79)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(374.4 * 3.0)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        let mut ca = false;
        let mut skill = false;
        let mut burst = false;
        let mut electro_er = false;
        for a in attack {
            match a.kind {
                Ca    => ca = true,
                Skill => skill = true,
                Burst => burst = true,
                _ => (),
            };
            electro_er = enemy.trigger_er(&a.element).is_electro();
        }
        self.ca_a1.update(gaurd.second(ca), time);
        self.skill_aa.update(gaurd.second(skill), time);
        self.burst_timer.update(gaurd.second(burst), time);
        self.aa_a4.update(gaurd.second(electro_er), time);
        // reset skill timer on burst
        if self.burst_timer.is_active() {
            self.skill_aa.update(gaurd.second(true), 25.0);
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Electro,
                multiplier: 159.84,
                particle: Some(1.0),
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
        if self.skill_aa.is_active() && self.ca_a1.is_active() {
            atk_queue.push(Attack {
                kind: Ca,
                element: Electro,
                multiplier: 152.7,
                particle: None,
                state: None,
                icd_cleared: fa.na.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
        if self.skill_aa.is_active() && self.aa_a4.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot, // TODO inaccurate
                element: Electro,
                multiplier: 80.0,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn accelerate(&self, _na: &mut NormalAttackAction, skill: &mut SkillAction, _burst: &mut BurstAction) -> () {
        if self.burst_timer.is_active() {
            skill.cd = 0.0;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_aa.reset();
        self.burst_timer.reset();
        self.ca_a1.reset();
        self.aa_a4.reset();
    }
}

pub struct Lisa {
    burst_aa: DotTimer,
}

impl Lisa {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 0.5555, 28),
        }
    }
}

impl SpecialAbility for Lisa {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Lisa").vision("Electro").weapon("Catalyst").release_date("2020-09-28").version(1.0)
            .base_hp(9570.0).base_atk(232.0).base_def(573.0)
            .em(96.0)
            .na_1(71.28).na_2(64.66).na_3(77.04).na_4(98.93).na_5(0.0).na_6(0.0).na_time(1.5)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(16.0).press_particle(5.0).press_dmg(576.0)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(0.0)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Electro,
                multiplier: 65.81,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    // a4
    fn modify(&self, _modifiable_state: &mut [State], _owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        if self.burst_aa.is_active() {
            enemy.def_down_debuff.push(Debuff::lisa_a4());
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
    }
}

pub struct Razor {
    burst_timer: DurationTimer,
    burst_aa: HitsTimer
}

impl Razor {
    pub fn new() -> Self {
        Self {
            burst_timer: DurationTimer::new(20.0, 15.0),
            burst_aa: HitsTimer::new(0.001, 1),
        }
    }
}

impl SpecialAbility for Razor {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Razor").vision("Electro").weapon("Claymore").release_date("2020-09-28").version(1.0)
            .base_hp(11962.0).base_atk(234.0).base_def(751.0)
            .dmg_phy(30.0)
            .na_1(171.13).na_2(147.42).na_3(184.32).na_4(242.72).na_5(0.0).na_6(0.0).na_time(2.734)
            // a1
            .press_cd(6.0 * 0.82).press_particle(4.0).press_dmg(358.56)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(288.0)
            .skill_unit(2.0).skill_decay(B).burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let mut na = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Skill => na = true,
                Burst => burst = true,
                _ => (),
            }
        }
        self.burst_timer.update(gaurd.second(burst), time);
        self.burst_aa.update(gaurd.second(na), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_timer.is_active() && self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Electro,
                multiplier: 43.2,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk_spd += 40.0;
        }
        // a4
        if owner_fc.state.energy.0 / owner_fc.state.energy_cost <= 0.5 {
            modifiable_state[owner_fc.idx.0].er += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.burst_aa.reset();
    }
}

pub struct Keqing {
    skill_timer: DurationTimer,
    burst_a4: DurationTimer,
}

impl Keqing {
    pub fn new() -> Self {
        Self {
            skill_timer: DurationTimer::new(7.5, 5.0),
            burst_a4: DurationTimer::new(12.0, 8.0),
        }
    }
}

impl SpecialAbility for Keqing {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Keqing").vision("Electro").weapon("Sword").release_date("2020-09-28").version(1.0)
            .base_hp(13103.0).base_atk(323.0).base_def(799.0)
            .cd(88.4)
            .na_1(81.09).na_2(81.09).na_3(107.61).na_4(62.22+68.0).na_5(132.43).na_6(0.0).na_time(2.017)
            // .na_0(81.09).ca_1(151.81 + 170.0).ca_2(0.0).ca_time(0.934)
            .press_cd(7.5).press_particle(2.5).press_dmg(90.72 + 302.4)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(158.4 + 43.2*8.0 + 339.84)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let mut skill = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Skill => skill = true,
                Burst => burst = true,
                _ => (),
            }
        }
        self.skill_timer.update(gaurd.second(skill), time);
        self.burst_a4.update(gaurd.second(burst), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[owner_fc.idx.0];
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
