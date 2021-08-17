use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::fc::{CharacterData, FieldCharacterIndex, SpecialAbility, SkillAbility, FieldAbilityBuilder, Enemy, Debuff};
use crate::types::{AttackType, Vision, FieldEnergy, Particle, VecFieldEnergy, ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE, PYRO_GAUGE1A, HYDRO_GAUGE1A, ELECTRO_GAUGE1A, CRYO_GAUGE1A, ANEMO_GAUGE1A, GEO_GAUGE1A, DENDRO_GAUGE1A};

use AttackType::*;

#[derive(Debug, Clone, Copy)]
pub struct AttackEvent {
    pub kind: AttackType,
    pub idx: FieldCharacterIndex,
}

impl AttackEvent {
    pub fn empty() -> Self {
        Self {
            kind: StandStill,
            idx: FieldCharacterIndex(0),
        }
    }
}

impl PartialEq<Attack> for AttackEvent {
    fn eq(&self, other: &Attack) -> bool {
        self.kind.eq(&other.kind) && self.idx.eq(&other.idx)
    }
}

#[derive(Debug)]
pub struct Attack {
    // type of this `Attack`. For example, Xiangling's skill summons Guoba to
    // deal DoT Pyro DMG. This DMG is considered as an additional attack and
    // since it is created by her skill, the `kind` is `AttackType::Skill`.
    pub kind: AttackType,

    // elemental gauge of this `Attack`.
    pub element: &'static ElementalGauge,

    pub multiplier: f32,

    pub hits: usize,

    pub icd_timer: Rc<RefCell<ICDTimer>>,

    pub idx: FieldCharacterIndex,
}

impl Attack {
    pub fn na(multiplier: f32, hits: usize, idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            kind: AttackType::Na,
            element: &PHYSICAL_GAUGE,
            multiplier,
            hits,
            icd_timer: Rc::clone(&icd_timer.na),
            idx,
        }
    }

    pub fn most_eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.idx == other.idx
    }

    pub fn to_event(&self, timer: &NTimer) -> Option<AttackEvent> {
        if timer.n == 0 {
            Some(AttackEvent {
                kind: self.kind,
                idx: self.idx,
            })
        } else {
            None
        }
    }

    pub fn icd_cleared(&self) -> bool {
        self.icd_timer.borrow().clear()
    }

    pub fn outgoing_damage(&self, state: Option<State>, fc: &CharacterData) -> f32 {
        // use ad-hoc state if available
        if let Some(mut state) = state {
            state.merge(&fc.state);
            self.outgoing_damage_inner(&state, fc)
        } else {
            self.outgoing_damage_inner(&fc.state, fc)
        }
    }

    fn outgoing_damage_inner(&self, state: &State, fc: &CharacterData) -> f32 {
        let bonus = state.DMG_bonus(&self.kind, &self.element.aura);
        let crcd = state.CRCD();
        let atk = match (fc.character.name, self.kind) {
            ("Albedo", AttackType::SkillDot) => state.DEF(),
            ("Noelle", AttackType::PressSkill) => state.DEF(),
            _ => state.ATK(),
        };
        let power = atk * bonus * crcd;
        self.multiplier / 100.0 * power * state.get_talent_bonus(&self.kind)
    }

    pub fn incoming_damage(&self, outgoing_damage: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        let def_down = 1.0 - enemy.get_def_down() / 100.0;
        let enemy_defense = enemy.level / (enemy.level * def_down + enemy.level);
        let resistance = self.resistance(&enemy);
        let dmg = outgoing_damage * resistance * enemy_defense;
        self.elemental_reaction(dmg, resistance, fc, enemy)
    }

    fn resistance(&self, enemy: &Enemy) -> f32 {
        let enemy_res: f32;
        let res_decrease: f32;
        if self.element.aura == Vision::Physical {
            enemy_res = enemy.physical_res;
            res_decrease = enemy.get_physical_res();
        } else {
            enemy_res = enemy.element_res;
            res_decrease = enemy.get_element_res();
        }
        let res = if res_decrease > enemy_res {
            -0.5 * (res_decrease - enemy_res)
        } else {
            enemy_res - res_decrease
        };
        (100.0 - res) / 100.0
    }

    pub fn elemental_reaction(&self, outgoing_damage: f32, resistance: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        use ElementalReactionType::*;
        let mut total_dmg = 0.0;
        for _ in 0..self.hits {
            // weapons do not have ICD timers.
            if self.kind != AdditionalAttack && self.icd_cleared() {
                let state = &fc.state;
                let elemental_reaction = ElementalReaction::new(enemy.aura.aura, self.element.aura);
                total_dmg += match elemental_reaction {
                    Overloaded(ref er) |
                    Shatter(ref er) |
                    ElectorCharged(ref er) |
                    Swirl(ref er) |
                    Superconduct(ref er) => outgoing_damage + resistance * er.transformative_reaction(state.em, state.transformative_bonus),
                    Vaporize(ref er) |
                    Melt(ref er) => outgoing_damage * er.amplifying_reaction(state.em, state.amplifying_bonus),
                    Crystallize(_) |
                    Equalize(_) |
                    Freeze(_) |
                    Burn(_) |
                    Neutralize(_) => outgoing_damage,
                };
                enemy.aura.trigger(self);
                if let Freeze(_) = elemental_reaction {
                    enemy.isfrozen = true;
                }
                if let Superconduct(_) = elemental_reaction {
                    enemy.physical_res_debuff.push(Debuff::superconduct());
                }
                self.icd_timer.borrow_mut().count_hit();
            } else {
                if self.kind != AdditionalAttack {
                    self.icd_timer.borrow_mut().count_hit();
                }
                total_dmg += outgoing_damage;
            }
        }
        total_dmg
    }
}

// `NTimer` should be constructed like: NTimer::new(&[ duration, cool_down ])
#[derive(Debug)]
pub struct ElementalAbsorption {
    pub timer: NTimer,
    pub attack: Attack,
}

impl ElementalAbsorption {
    pub fn new(idx: FieldCharacterIndex, kind: AttackType, multiplier: f32, timer: NTimer, icd_timer: &ICDTimers) -> Self {
        Self {
            timer,
            attack: Attack {
                kind,
                element: &PHYSICAL_GAUGE,
                multiplier,
                hits: 1,
                icd_timer: match &kind {
                    Na => Rc::clone(&icd_timer.na),
                    Ca => Rc::clone(&icd_timer.ca),
                    PressSkill | HoldSkill | SkillDot => Rc::clone(&icd_timer.skill),
                    Burst | BurstDot => Rc::clone(&icd_timer.burst),
                    _ => unimplemented!(),
                },
                idx,
            }
        }
    }

    pub fn did_absorb(&self) -> bool {
        self.attack.element != &PHYSICAL_GAUGE
    }

    pub fn absorb(&mut self, time: f32, guard: bool, enemy: &Enemy) -> () {
        use Vision::*;
        self.timer.update(time, guard);
        match (self.timer.ping, self.timer.n) {
            (true, 1) => match &enemy.aura.aura {
                Pyro => self.attack.element = &PYRO_GAUGE1A,
                Hydro => self.attack.element = &HYDRO_GAUGE1A,
                Electro => self.attack.element = &ELECTRO_GAUGE1A,
                Cryo => self.attack.element = &CRYO_GAUGE1A,
                _ => (),
            },
            (true, 2) => self.attack.element = &PHYSICAL_GAUGE,
            _ => (),
        }
    }

    pub fn attack(&self) -> Option<*const Attack> {
        if self.did_absorb() {
            Some(&self.attack)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Time {
    Waiting(f32),
    Done,
}

// n = 0 means inactive
#[derive(Debug)]
pub struct NTimer {
    pub n: usize,
    cool_down: &'static [f32],
    pub ping: bool,
    pub state: Time,
    conditional_reset: bool,
}

impl NTimer {
    pub fn new(cool_down: &'static [f32]) -> Self {
        Self {
            n: 0,
            cool_down,
            ping: false,
            state: Time::Waiting(cool_down[0]),
            conditional_reset: false,
        }
    }

    pub fn with_condition(cool_down: &'static [f32]) -> Self {
        Self {
            n: 0,
            cool_down,
            ping: false,
            state: Time::Waiting(cool_down[0]),
            conditional_reset: true,
        }
    }

    pub fn update(&mut self, time: f32, guard: bool) -> () {
        let should_ping = self.n == 0;
        if (guard && should_ping) || !should_ping {
            self.update_inner(time, should_ping, true, guard);
        } else {
            self.ping = false;
        }
    }

    pub fn update_na(&mut self, time: f32, guard: bool) -> () {
        let should_ping = self.n == 0;
        if (guard && should_ping) || !should_ping {
            self.update_inner(time, should_ping, false, guard);
        } else {
            self.ping = false;
        }
    }

    pub fn reset(&mut self) -> () {
        self.n = 0;
        self.ping = false;
        self.state = Time::Waiting(self.cool_down[0]);
    }

    fn update_inner(&mut self, time: f32, should_ping: bool, has_terminus: bool, guard: bool) -> () {
        use Time::*;
        match (&mut self.state, self.conditional_reset, guard) {
            (Waiting(ref mut t), _, _) => {
                *t -= time;
                self.ping = should_ping;
                if should_ping {
                    self.n += 1;
                }
                if *t <= 0.0 {
                    let time = -1.0 * *t;
                    if self.n == self.cool_down.len() {
                        self.state = Done;
                    } else {
                        self.state = Waiting(self.cool_down[self.n]);
                    }
                    self.update_inner(time, true, has_terminus, guard);
                }
            },
            (Done, true, false) => (),
            (Done, true, true) |
            (Done, false, _) => {
                self.n = 0;
                self.ping = true;
                self.state = Waiting(self.cool_down[0]);
                if !has_terminus {
                    self.update_inner(time, self.ping, has_terminus, guard);
                }
            },
        }
    }
}

// n = 0 means inactive. This timer has `duration` to expire own states when it
// has ended. Typical examples are: Prototype Rancour, Elegy for the End. So,
// `NTimer` should be used in general.
#[derive(Debug)]
pub struct DurationTimer {
    pub previous_n: usize,
    pub n: usize,
    duration: f32,
    cool_down: &'static [f32],
    pub ping: bool,
    pub cd: f32,
    pub dr: Time,
}

impl DurationTimer {
    pub fn new(duration: f32, cool_down: &'static [f32]) -> Self {
        Self {
            previous_n: 0,
            n: 0,
            duration,
            cool_down,
            ping: false,
            cd: 0.0,
            dr: Time::Done,
        }
    }

    pub fn update(&mut self, time: f32, should_update: bool) -> () {
        self.update_inner(time, false, should_update);
    }

    pub fn reset(&mut self) -> () {
        self.previous_n = 0;
        self.n = 0;
        self.ping = false;
        self.cd = 0.0;
        self.dr = Time::Done;
    }

    fn update_inner(&mut self, time: f32, should_ping: bool, should_update: bool) -> () {
        use Time::*;
        match (&mut self.dr, self.cd > 0.0) {
            (Done, true) => {
                self.cd -= time;
                self.ping = should_ping;
                if should_update && self.cd <= 0.0 {
                    self.previous_n = self.n;
                    self.n = 1;
                    self.cd = self.cool_down[self.n - 1];
                    self.dr = Waiting(self.duration);
                    self.ping = true;
                }
            },
            (Waiting(ref mut dr), _) => {
                self.cd -= time;
                *dr -= time;
                self.ping = should_ping;
                if should_update && self.cd <= 0.0 {
                    let mut changed = false;
                    if self.n != self.cool_down.len() {
                        self.previous_n = self.n;
                        self.n += 1;
                        changed = true;
                    }
                    self.cd = self.cool_down[self.n - 1];
                    self.dr = Waiting(self.duration);
                    self.ping = changed || should_ping;
                } else if *dr <= 0.0 {
                    self.dr = Done;
                    self.previous_n = self.n;
                    self.n = 0;
                    self.ping = true;
                }
            },
            (Done, _) => {
                self.ping = should_ping;
                if should_update {
                    self.previous_n = self.n;
                    self.n = 1;
                    self.cd = self.cool_down[self.n - 1];
                    self.dr = Waiting(self.duration);
                    self.update_inner(time, true, should_update);
                }
            },
        }
    }
}

// meaning of `n`:
// 0 = inactive
// 1 = performing the motion
// 2 = out of stamina
#[derive(Debug)]
pub struct StaminaTimer {
    pub n: usize,
    cool_down: f32,
    pub ping: bool,
    pub motion: Time,
    pub recovery: Time,
    stamina: f32,
}

impl StaminaTimer {
    pub fn new(cool_down: f32) -> Self {
        Self {
            n: 0,
            cool_down,
            ping: false,
            motion: Time::Done,
            recovery: Time::Done,
            stamina: 240.0,
        }
    }

    pub fn reset(&mut self) -> () {
        self.n = 0;
        self.motion = Time::Done;
        self.recovery = Time::Done;
        self.stamina = 240.0;
    }

    pub fn update(&mut self, time: f32, consumption: f32, guard: bool) -> () {
        let should_ping = self.n == 0;
        if (guard && should_ping) || !should_ping {
            let consumption = if guard {
                consumption
            } else {
                0.0
            };
            self.update_inner(time, consumption, should_ping);
        } else {
            self.ping = false;
            if self.stamina < 240.0 {
                self.stamina += 25.0 * time;
                if self.stamina > 240.0 {
                    self.stamina = 240.0;
                }
            }
        }
    }

    fn update_inner(&mut self, time: f32, consumption: f32, should_ping: bool) -> () {
        use Time::*;
        match (&mut self.motion, &mut self.recovery) {
            (Done, Waiting(ref mut t)) => {
                *t -= time;
                self.ping = should_ping;
                if *t <= 0.0 {
                    self.n = 0;
                    self.ping = true;
                    self.recovery = Done;
                    self.stamina = 240.0;
                }
            },
            (Waiting(ref mut t), Done) => {
                *t -= time;
                self.stamina -= consumption - 25.0 * time;
                if self.stamina > 240.0 {
                    self.stamina = 240.0;
                }
                self.ping = should_ping;
                if should_ping {
                    self.n += 1;
                }
                if self.stamina <= 0.0 {
                    self.n = 2;
                    self.ping = true;
                    self.motion = Done;
                    // total stamina / recovery rate per second
                    self.recovery = Waiting(240.0 / 25.0);
                } else if *t <= 0.0 {
                    self.n = 0;
                    self.ping = true;
                    self.motion = Done;
                }
            },
            (Done, Done) => {
                self.n = 0;
                self.ping = true;
                self.motion = Waiting(self.cool_down);
                self.update_inner(time, consumption, self.ping);
            },
            (Waiting(_), Waiting(_)) => unimplemented!(),
        }
    }
}

// #[derive(Debug)]
pub struct NaLoop {
    idx: FieldCharacterIndex,
    pub timer: NTimer,
    // `Vec` is used because each `Attack` needs to be mutable.
    pub attack: Vec<Attack>,
    did_infuse: bool,
}

impl NaLoop {
    pub fn new(cool_down: &'static [f32], attack: Vec<Attack>) -> Self {
        let idx = attack[0].idx;
        Self {
            idx,
            timer: NTimer::new(cool_down),
            attack,
            did_infuse: false,
        }
    }
}

impl SpecialAbility for NaLoop {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        if (!self.timer.ping && self.timer.n == 0) || (self.timer.ping && self.timer.n > 0) {
            Some(AttackEvent {
                kind: AttackType::Na,
                idx: self.idx,
            })
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update_na(time, event == &self.attack[0]);
        if data.state.infusion && !self.did_infuse {
            self.did_infuse = true;
            match &data.character.vision {
                Vision::Pyro => for a in self.attack.iter_mut() { a.element = &PYRO_GAUGE1A; },
                Vision::Hydro => for a in self.attack.iter_mut() { a.element = &HYDRO_GAUGE1A; },
                Vision::Electro => for a in self.attack.iter_mut() { a.element = &ELECTRO_GAUGE1A; },
                Vision::Cryo => for a in self.attack.iter_mut() { a.element = &CRYO_GAUGE1A; },
                Vision::Anemo => for a in self.attack.iter_mut() { a.element = &ANEMO_GAUGE1A; },
                Vision::Geo => for a in self.attack.iter_mut() { a.element = &GEO_GAUGE1A; },
                Vision::Dendro => for a in self.attack.iter_mut() { a.element = &DENDRO_GAUGE1A; },
                _ => (),
            }
        } else if !data.state.infusion && self.did_infuse {
            self.did_infuse = false;
            for a in self.attack.iter_mut() { a.element = &PHYSICAL_GAUGE; }
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => atk_queue.push(&self.attack[0]),
            (true, 2) => atk_queue.push(&self.attack[1]),
            (true, 3) => atk_queue.push(&self.attack[2]),
            (true, 4) => atk_queue.push(&self.attack[3]),
            (true, 5) => atk_queue.push(&self.attack[4]),
            (true, 6) => atk_queue.push(&self.attack[5]),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.did_infuse = false;
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SimpleCa {
    pub consumption: f32,
    pub timer: StaminaTimer,
    pub attack: Attack,
}

impl SimpleCa {
    pub fn new(consumption: f32, cool_down: f32, attack: Attack) -> Self {
        Self {
            consumption,
            timer: StaminaTimer::new(cool_down),
            attack,
        }
    }
}

impl SpecialAbility for SimpleCa {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        // TODO Attack.to_event
        if self.timer.n == 0 {
            Some(AttackEvent {
                kind: self.attack.kind,
                idx: self.attack.idx,
            })
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, self.consumption, event == &self.attack);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n, &self.timer.recovery) {
            (true, 1, Time::Done) => {
                atk_queue.push(&self.attack);
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SimpleSkill {
    pub timer: NTimer,
    pub attack: Attack,
    pub particle: Particle,
}

impl SimpleSkill {
    pub fn new(cool_down: &'static [f32], particle: Particle, attack: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
            particle,
        }
    }
}

impl SkillAbility for SimpleSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer);
    }
}

impl SpecialAbility for SimpleSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer)
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event == &self.attack);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
                particles.push_p(self.particle);
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SimpleSkillDot {
    pub timer: NTimer,
    pub attack: Attack,
    pub particle: Particle,
}

impl SimpleSkillDot {
    pub fn new(cool_down: &'static [f32], particle: Particle, attack: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
            particle,
        }
    }
}

impl SkillAbility for SimpleSkillDot {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer);
    }
}

impl SpecialAbility for SimpleSkillDot {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        if self.timer.n == 0 {
            Some(AttackEvent {
                kind: AttackType::PressSkill,
                idx: self.attack.idx,
            })
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.attack.idx && event.kind == AttackType::PressSkill);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => {
                atk_queue.push(&self.attack);
                particles.push_p(self.particle);
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SkillDamage2Dot {
    pub timer: NTimer,
    pub attack: Attack,
    pub dot: Attack,
    pub particle: Particle,
}

impl SkillDamage2Dot {
    pub fn new(cool_down: &'static [f32], particle: Particle, attack: Attack, dot: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
            dot,
            particle,
        }
    }
}

impl SkillAbility for SkillDamage2Dot {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer);
    }
}

impl SpecialAbility for SkillDamage2Dot {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer)
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event == &self.attack);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.dot);
                particles.push_p(self.particle);
            },
            (true, _) => {
                atk_queue.push(&self.dot);
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SkillDamage2DotParticle {
    pub timer: NTimer,
    pub attack: Attack,
    pub dot: Attack,
    pub particle: Particle,
}

impl SkillDamage2DotParticle {
    pub fn new(cool_down: &'static [f32], particle: Particle, attack: Attack, dot: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
            dot,
            particle,
        }
    }
}

impl SkillAbility for SkillDamage2DotParticle {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer);
    }
}

impl SpecialAbility for SkillDamage2DotParticle {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer)
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event == &self.attack);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.dot);
            },
            (true, _) => {
                atk_queue.push(&self.dot);
                particles.push_p(self.particle);
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SimpleBurst {
    pub timer: NTimer,
    pub attack: Attack,
}

impl SimpleBurst {
    pub fn new(cool_down: &'static [f32], attack: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
        }
    }
}

impl SpecialAbility for SimpleBurst {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
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

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct SimpleBurstDot {
    pub timer: NTimer,
    pub attack: Attack,
}

impl SimpleBurstDot {
    pub fn new(cool_down: &'static [f32], attack: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
        }
    }
}

impl SpecialAbility for SimpleBurstDot {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        if self.timer.n == 0 && data.can_burst() {
            Some(AttackEvent {
                kind: AttackType::Burst,
                idx: self.attack.idx,
            })
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.attack.idx && event.kind == Burst);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => atk_queue.push(&self.attack),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct BurstDamage2Dot {
    pub timer: NTimer,
    pub attack: Attack,
    pub dot: Attack,
}

impl BurstDamage2Dot {
    pub fn new(cool_down: &'static [f32], attack: Attack, dot: Attack) -> Self {
        Self {
            timer: NTimer::new(cool_down),
            attack,
            dot,
        }
    }
}

impl SpecialAbility for BurstDamage2Dot {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
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
                atk_queue.push(&self.dot);
            },
            // TODO up to a certain hit?
            (true, _) => {
                atk_queue.push(&self.dot);
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct ICDTimer {
    cd: f32,
    n_hits: usize,
    counting: bool,
}

impl ICDTimer {
    pub fn new() -> Self {
        Self {
            cd: 0.0,
            n_hits: 0,
            counting: false,
        }
    }

    pub fn clear(&self) -> bool {
        self.cd == 0.0 || self.n_hits == 0
    }

    pub fn count_hit(&mut self) -> () {
        // TODO counter increases if attack is infused (dont function on physical attack)
        self.counting = true;
        self.n_hits += 1;
        if self.n_hits >= 3 {
            self.n_hits = 0;
        }
    }

    pub fn update(&mut self, time: f32) -> () {
        if self.counting {
            self.cd += time;
        }
        if self.cd >= 2.5 {
            self.n_hits = 0;
            self.cd = 0.0;
            self.counting = false;
        }
    }

    pub fn reset(&mut self) -> () {
        self.cd = 0.0;
        self.n_hits = 0;
        self.counting = false;
    }
}

#[derive(Debug)]
pub struct ICDTimers {
    pub na: Rc<RefCell<ICDTimer>>,
    pub ca: Rc<RefCell<ICDTimer>>,
    pub skill: Rc<RefCell<ICDTimer>>,
    pub burst: Rc<RefCell<ICDTimer>>,
    pub noop: Rc<RefCell<ICDTimer>>,
}

impl ICDTimers {
    pub fn new() -> Self {
        Self {
            na: Rc::new(RefCell::new(ICDTimer::new())),
            ca: Rc::new(RefCell::new(ICDTimer::new())),
            skill: Rc::new(RefCell::new(ICDTimer::new())),
            burst: Rc::new(RefCell::new(ICDTimer::new())),
            noop: Rc::new(RefCell::new(ICDTimer::new())),
        }
    }

    pub fn update(&mut self, time: f32) -> () {
        self.na.borrow_mut().update(time);
        self.ca.borrow_mut().update(time);
        self.skill.borrow_mut().update(time);
        self.burst.borrow_mut().update(time);
        // noop
    }

    pub fn reset(&mut self) -> () {
        self.na.borrow_mut().reset();
        self.ca.borrow_mut().reset();
        self.skill.borrow_mut().reset();
        self.burst.borrow_mut().reset();
        // noop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[derive(Debug)]
struct ShortLoop {
    timer: NTimer
}

impl ShortLoop {
    fn new() -> Self {
        Self {
            timer: NTimer::new(&[0.5, 0.8]),
        }
    }

    fn maybe_attack(&self) -> bool {
        self.timer.ping || self.timer.n == 0
    }

    fn update(&mut self, time: f32) -> () {
        self.timer.update_na(time, true);
    }

    fn attack(&self) -> Option<f32> {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => Some(10.0),
            (true, 2) => Some(20.0),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct KeqingSkill {
    timer: NTimer
}

impl KeqingSkill {
    fn new() -> Self {
        Self {
            timer: NTimer::new(&[5.0, 2.5]),
        }
    }

    fn maybe_attack(&self) -> bool {
        self.timer.n == 0
    }

    fn update(&mut self, time: f32) -> () {
        self.timer.update(time, true);
    }

    // (mut queue) -> ()
    fn attack(&self) -> Option<f32> {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => Some(100.0),
            _ => None,
        }
    }

    fn modify(&self) -> bool {
        match (&self.timer.state, self.timer.n) {
            (Time::Waiting(_), 1) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct SoBP {
    timer: DurationTimer
}

impl SoBP {
    fn new() -> Self {
        Self {
            timer: DurationTimer::new(12.0, &[0.3, 0.3, 20.0]),
        }
    }

    fn update(&mut self, time: f32, should_update: bool) -> () {
        self.timer.update(time, should_update);
    }

    fn modify(&self) -> usize {
        match &self.timer.dr {
            Time::Waiting(_) => self.timer.n,
            _ => 0,
        }
    }
}

    #[test]
    fn keqing_1() {
        let k = KeqingSkill::new();
        assert!(k.maybe_attack());
    }

    #[test]
    fn keqing_2() {
        let mut k = KeqingSkill::new();
        k.update(1.0);
        assert!(!k.maybe_attack());
        assert_eq!(k.attack(), Some(100.0));
        assert_eq!(k.modify(), true);
    }

    #[test]
    fn keqing_3() {
        let mut k = KeqingSkill::new();
        k.update(5.0);
        assert!(!k.maybe_attack());
        assert_eq!(k.attack(), None);
        assert_eq!(k.modify(), false);
    }

    #[test]
    fn keqing_4() {
        let mut k = KeqingSkill::new();
        k.update(7.5);
        assert!(k.maybe_attack());
    }

    #[test]
    fn naloop_1() {
        let na = ShortLoop::new();
        assert!(na.maybe_attack());
    }

    #[test]
    fn naloop_2() {
        let mut na = ShortLoop::new();
        na.update(0.25);
        assert!(na.maybe_attack());
        assert_eq!(na.attack(), Some(10.0));
    }

    #[test]
    fn naloop_3() {
        let mut na = ShortLoop::new();
        na.update(0.25);
        na.update(0.1);
        assert!(!na.maybe_attack());
        assert_eq!(na.attack(), None);
    }

    #[test]
    fn naloop_4() {
        let mut na = ShortLoop::new();
        na.update(0.5);
        assert!(na.maybe_attack());
        assert_eq!(na.attack(), Some(20.0));
    }

    #[test]
    fn sobp_1() {
        let sobp = SoBP::new();
        assert_eq!(sobp.modify(), 0);
    }

    #[test]
    fn sobp_2() {
        let mut sobp = SoBP::new();
        sobp.update(0.0, true);
        assert_eq!(sobp.modify(), 1);
    }

    #[test]
    fn sobp_3() {
        let mut sobp = SoBP::new();
        sobp.update(0.3, true);
        assert_eq!(sobp.modify(), 2);
    }

    #[test]
    fn sobp_4() {
        let mut sobp = SoBP::new();
        sobp.update(0.3, true);
        sobp.update(0.3, true);
        assert_eq!(sobp.modify(), 3);
    }

    #[test]
    fn sobp_5() {
        let mut sobp = SoBP::new();
        sobp.update(0.3, true);
        sobp.update(0.3, true);
        assert_eq!(sobp.modify(), 3);
        sobp.update(12.0, true);
        assert_eq!(sobp.modify(), 0);
    }

    #[test]
    fn stamina_1() {
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(0.0, 0.0, true);
        assert_eq!(stamina.n, 1);
        assert_eq!(stamina.ping, true);
        assert_eq!(stamina.motion, Time::Waiting(1.0));
        assert_eq!(stamina.recovery, Time::Done);
    }

    #[test]
    fn stamina_2() {
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(0.5, 0.0, true);
        assert_eq!(stamina.n, 1);
        assert_eq!(stamina.ping, true);
        assert_eq!(stamina.motion, Time::Waiting(0.5));
        assert_eq!(stamina.recovery, Time::Done);
    }

    #[test]
    fn stamina_3() {
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(1.0, 0.0, true);
        assert_eq!(stamina.n, 0);
        assert_eq!(stamina.ping, true);
        assert_eq!(stamina.motion, Time::Done);
        assert_eq!(stamina.recovery, Time::Done);
    }

    #[test]
    fn stamina_4() {
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(0.5, 0.0, true);
        stamina.update(0.2, 0.0, true);
        assert_eq!(stamina.n, 1);
        assert_eq!(stamina.ping, false);
        assert_eq!(stamina.motion, Time::Waiting(0.3));
        assert_eq!(stamina.recovery, Time::Done);
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(0.5, 0.0, true);
        stamina.update(0.2, 0.0, false);
        assert_eq!(stamina.n, 1);
        assert_eq!(stamina.ping, false);
        assert_eq!(stamina.motion, Time::Waiting(0.3));
        assert_eq!(stamina.recovery, Time::Done);
    }

    #[test]
    fn stamina_5() {
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(1.0, 0.0, true);
        stamina.update(0.5, 0.0, true);
        assert_eq!(stamina.n, 1);
        assert_eq!(stamina.ping, true);
        assert_eq!(stamina.motion, Time::Waiting(0.5));
        assert_eq!(stamina.recovery, Time::Done);
    }

    #[test]
    fn stamina_6() {
        let mut stamina = StaminaTimer::new(1.0);
        stamina.update(1.0, 0.0, true);
        stamina.update(0.5, 0.0, false);
        assert_eq!(stamina.n, 0);
        assert_eq!(stamina.ping, false);
        assert_eq!(stamina.motion, Time::Done);
        assert_eq!(stamina.recovery, Time::Done);
    }
}
