
use crate::fc::{CharacterData, FieldCharacterIndex, CharacterAbility, Enemy, Debuff};
use crate::types::{AttackType, Vision, ElementalGauge, BareElementalGauge, ElementalReactionType, ElementalReaction};
use crate::state::State;

use AttackType::*;

#[derive(Debug, Clone, Copy)]
pub struct AttackEvent {
    pub kind: AttackType,
    pub idx: FieldCharacterIndex,
}

impl AttackEvent {
    pub fn new(kind: AttackType, idx: usize) -> Self {
        Self {
            kind,
            idx: FieldCharacterIndex(idx),
        }
    }

    pub fn empty() -> Self {
        Self {
            kind: StandStill,
            idx: FieldCharacterIndex(0),
        }
    }
}

// Because of `State.infusion`, infused element of every attack is determined at
// the run time.
#[derive(Debug, Clone, Copy)]
pub struct ElementalAttack {
    pub element: Vision,
    pub atk: *const Attack,
}

impl ElementalAttack {
    pub fn new(element: Vision, atk: *const Attack) -> Self {
        Self {
            element,
            atk,
        }
    }

    pub fn pyro(atk: *const Attack) -> Self {
        Self {
            element: Vision::Pyro,
            atk,
        }
    }

    pub fn hydro(atk: *const Attack) -> Self {
        Self {
            element: Vision::Hydro,
            atk,
        }
    }

    pub fn electro(atk: *const Attack) -> Self {
        Self {
            element: Vision::Electro,
            atk,
        }
    }

    pub fn cryo(atk: *const Attack) -> Self {
        Self {
            element: Vision::Cryo,
            atk,
        }
    }

    pub fn anemo(atk: *const Attack) -> Self {
        Self {
            element: Vision::Anemo,
            atk,
        }
    }

    pub fn geo(atk: *const Attack) -> Self {
        Self {
            element: Vision::Geo,
            atk,
        }
    }

    pub fn dendro(atk: *const Attack) -> Self {
        Self {
            element: Vision::Dendro,
            atk,
        }
    }

    pub fn physical(atk: *const Attack) -> Self {
        Self {
            element: Vision::Physical,
            atk,
        }
    }

    pub fn outgoing_damage(&self, attack_element: &Vision, state: Option<State>, fc: &CharacterData) -> f32 {
        let atk = unsafe { &(*self.atk) };
        atk.outgoing_damage(attack_element, state, fc)
    }

    pub fn incoming_damage(&self, attack_element: &Vision, outgoing_damage: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        let atk = unsafe { &(*self.atk) };
        atk.incoming_damage(attack_element, outgoing_damage, fc, enemy)
    }

}

impl PartialEq<Vision> for ElementalAttack {
    fn eq(&self, other: &Vision) -> bool {
        self.element.eq(other)
    }
}

#[derive(Debug)]
pub struct Attack {
    // type of this `Attack`. For example, Xiangling's skill summons Guoba to
    // deal DoT Pyro DMG. This DMG is considered as an additional attack and
    // since it is created by her skill, the `kind` is `AttackType::Skill`.
    pub kind: AttackType,

    // elemental gauge of this `Attack`.
    pub gauge: BareElementalGauge,

    pub multiplier: f32,

    pub hits: usize,

    // A pointer to `ICDTimer` of the respective `AttackType`. In order to have
    // a stable pointer address, the pointee `ICDTimer` must be stable. For
    // example, when we box a character who has `ICDTimer`, the character will
    // be stable and so is the timer.
    pub icd_timer: *mut ICDTimer,

    pub idx: FieldCharacterIndex,
}

impl Attack {
    // pub fn infuse(mut self, element: ElementalGauge) -> Self {
    //     self.element = element;
    //     self
    // }

    pub fn icd_cleared(&self) -> bool {
        unsafe {
            (*self.icd_timer).clear()
        }
    }

    pub fn outgoing_damage(&self, attack_element: &Vision, state: Option<State>, fc: &CharacterData) -> f32 {
        // use ad-hoc state if available
        if let Some(mut state) = state {
            state.merge(&fc.state);
            self.outgoing_damage_inner(attack_element, &state, fc)
        } else {
            self.outgoing_damage_inner(attack_element, &fc.state, fc)
        }
    }

    fn outgoing_damage_inner(&self, attack_element: &Vision, state: &State, fc: &CharacterData) -> f32 {
        let bonus = state.DMG_bonus(&self.kind, attack_element);
        let crcd = state.CRCD();
        let atk = match (fc.cr.name, self.kind) {
            ("Albedo", AttackType::SkillDot) => state.DEF(),
            ("Noelle", AttackType::PressSkill) => state.DEF(),
            _ => state.ATK(),
        };
        let power = atk * bonus * crcd;
        self.multiplier / 100.0 * power
    }

    pub fn incoming_damage(&self, attack_element: &Vision, outgoing_damage: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        let def_down = 1.0 + enemy.get_def_down() / 100.0;
        let level_multiplier = enemy.level / (enemy.level * def_down + enemy.level);
        let dmg = outgoing_damage * self.resistance(attack_element, &enemy) * level_multiplier;
        self.elemental_reaction(attack_element, dmg, fc, enemy)
    }

    fn resistance(&self, attack_element: &Vision, enemy: &Enemy) -> f32 {
        let enemy_res: f32;
        let res_decrease: f32;
        if *attack_element == Vision::Physical {
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

    pub fn elemental_reaction(&self, attack_element: &Vision, outgoing_damage: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        use ElementalReactionType::*;
        let mut total_dmg = 0.0;
        for _ in 0..self.hits {
            if self.icd_cleared() {
                let elemental_reaction = ElementalReaction::new(enemy.aura.aura, *attack_element);
                total_dmg += match elemental_reaction {
                    Overloaded(ref er) |
                    Shatter(ref er) |
                    ElectorCharged(ref er) |
                    Swirl(ref er) |
                    Superconduct(ref er) => outgoing_damage + er.transformative_reaction(fc.state.em, fc.state.transformative_bonus),
                    Vaporize(ref er) |
                    Melt(ref er) => outgoing_damage * er.amplifying_reaction(fc.state.em, fc.state.amplifying_bonus),
                    Crystallize(_) |
                    Equalize(_) |
                    Freeze(_) |
                    Burn(_) |
                    Neutralize(_) => outgoing_damage,
                };
                enemy.aura.trigger(self, attack_element);
                if let Freeze(_) = elemental_reaction {
                    enemy.isfrozen = true;
                }
                if let Superconduct(_) = elemental_reaction {
                    enemy.physical_res_debuff.push(Debuff::superconduct());
                }
            } else {
                total_dmg += outgoing_damage;
            }
            let icd_timer = unsafe { &mut *self.icd_timer };
            icd_timer.count_hit();
        }
        total_dmg
    }
}

pub trait EffectTimer {
    fn is_cd_off(&self) -> bool;
    fn is_active(&self) -> bool;
    fn n(&self) -> usize;
    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> ();
    fn reset(&mut self) -> ();
}

// cool_down == duration
#[derive(Debug)]
pub struct CDTimer {
    cool_down: f32,
    cd: f32,
}

impl CDTimer {
    pub fn new(cool_down: f32) -> Self {
        Self { cool_down, cd: 0.0 }
    }
}

impl EffectTimer for CDTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.cd > 0.0
    }

    fn n(&self) -> usize {
        if self.is_active() {
            1
        } else {
            0
        }
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        if gaurd.second && self.is_cd_off() {
            self.cd = self.cool_down;
        }
        self.cd -= time;
    }

    fn reset(&mut self) -> () {
        self.cd = 0.0;
    }
}

// cool_down != duration
#[derive(Debug)]
pub struct DurationTimer {
    cool_down: f32,
    duration: f32,
    cd: f32,
    dr: f32,
}

impl DurationTimer {
    pub fn new(cool_down: f32, duration: f32) -> Self {
        Self { cool_down, duration, cd: 0.0, dr: 0.0 }
    }
}

impl EffectTimer for DurationTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.dr > 0.0
    }

    fn n(&self) -> usize {
        if self.is_active() {
            1
        } else {
            0
        }
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.cd -= time;
        self.dr -= time;
        if gaurd.second && self.is_cd_off() {
            self.cd = self.cool_down;
            self.dr = self.duration;
        }
    }

    fn reset(&mut self) -> () {
        self.cd = 0.0;
        self.dr = 0.0;
    }
}

#[derive(Debug)]
pub struct HitsTimer {
    cool_down: f32,
    n_hits: usize,
    cd: f32,
    n: usize,
}

impl HitsTimer {
    pub fn new(cool_down: f32, n_hits: usize) -> Self {
        Self { cool_down, n_hits, cd: 0.0, n: 0 }
    }

    pub fn noop() -> Self {
        Self {
            cool_down: 10.0_f32.powf(6.0),
            n_hits: 0,
            cd: 10.0_f32.powf(6.0),
            n: 0
        }
    }
}

impl EffectTimer for HitsTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.n > 0
    }

    fn n(&self) -> usize {
        self.n
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.cd -= time;
        if self.n > 0 {
            self.n -= 1;
        }
        if gaurd.second && self.is_cd_off() {
            self.cd = self.cool_down;
            self.n = self.n_hits;
        }
    }

    fn reset(&mut self) -> () {
        self.cd = 0.0;
        self.n = 0
    }
}

// There is some "delay" between each DoT DMG. The delay is `dot_cd`.
#[derive(Debug)]
pub struct DotTimer {
    cool_down: f32,
    dot_cd: f32,
    n_hits: usize,
    cd: f32,
    dcd: f32,
    dcd_cleared: bool,
    n: usize,
}

impl DotTimer {
    pub fn new(cool_down: f32, dot_cd: f32, n_hits: usize) -> Self {
        Self {
            cool_down,
            dot_cd,
            n_hits,
            cd: 0.0,
            dcd: 0.0,
            dcd_cleared: false,
            n: 0
        }
    }

    pub fn single_hit(cool_down: f32) -> Self {
        Self {
            cool_down,
            dot_cd: 0.0,
            n_hits: 1,
            cd: 0.0,
            dcd: 0.0,
            dcd_cleared: false,
            n: 0
        }
    }

    pub fn noop() -> Self {
        Self {
            cool_down: 10.0_f32.powf(6.0),
            dot_cd: 10.0_f32.powf(6.0),
            n_hits: 1,
            cd: 10.0_f32.powf(6.0),
            dcd: 10.0_f32.powf(6.0),
            dcd_cleared: false,
            n: 0
        }
    }
}

impl EffectTimer for DotTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.n > 0 && self.dcd_cleared
    }

    fn n(&self) -> usize {
        if self.is_active() {
            self.n
        } else {
            0
        }
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.cd -= time;
        self.dcd -= time;
        self.dcd_cleared = self.dcd <= 0.0;
        if self.n > 0 && self.dcd_cleared {
            self.n -= 1;
            self.dcd = self.dot_cd;
        }
        if gaurd.second && self.is_cd_off() {
            self.cd = self.cool_down;
            self.dcd = self.dot_cd;
            self.n = self.n_hits;
            self.dcd_cleared = true;
        }
    }

    fn reset(&mut self) -> () {
        self.cd = 0.0;
        self.dcd = 0.0;
        self.dcd_cleared = false;
        self.n = 0;
    }
}

#[derive(Debug)]
pub struct StackTimer {
    cool_down: f32,
    duration: f32,
    level: usize,
    cd: f32,
    dr: f32,
    pub n: usize,
}

impl StackTimer {
    pub fn new(cool_down: f32, duration: f32, level: usize) -> Self {
        Self { cool_down, duration, level, cd: 0.0, dr: 0.0, n: 0 }
    }
}

impl EffectTimer for StackTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.n > 0 && self.dr > 0.0
    }

    fn n(&self) -> usize {
        if self.is_active() {
            self.n
        } else {
            0
        }
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.cd -= time;
        self.dr -= time;
        if self.dr <= 0.0 {
            self.n = 0;
        }
        if gaurd.second && self.is_cd_off() {
            self.cd = self.cool_down;
            self.dr = self.duration;
            self.n += 1;
            if self.n > self.level {
                self.n = self.level;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.cd = 0.0;
        self.dr = 0.0;
        self.n = 0
    }
}

#[derive(Debug)]
pub struct SigilTimer {
    cool_down: f32,
    effect_cd: f32,
    effect_duration: f32,
    max_level: usize,
    cd: f32,
    dr: f32,
    pub n: usize,
}

impl SigilTimer {
    pub fn new(cool_down: f32, effect_cd: f32, effect_duration: f32, max_level: usize) -> Self {
        Self { cool_down, effect_cd, effect_duration, max_level, cd: 0.0, n: 0, dr: 0.0, }
    }
}

impl EffectTimer for SigilTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.n == self.max_level
    }

    fn n(&self) -> usize {
        self.n
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.cd -= time;
        if self.is_active() {
            self.dr -= time;
        }
        // expire
        if self.is_active() && self.dr <= 0.0 {
            self.n = 0;
        }
        if gaurd.second && self.cd <= 0.0 {
            self.cd = self.cool_down;
            self.n += 1;
            if self.is_active() {
                self.cd = self.effect_cd;
                self.dr = self.effect_duration;
            }
            if self.n > self.max_level {
                self.n = self.max_level;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.cd = 0.0;
        self.dr = 0.0;
        self.n = 0;
    }
}

#[derive(Debug)]
pub struct LoopTimer {
    cool_down: f32,
    steps: usize,

    cd: f32,
    n: usize,
}

impl LoopTimer {
    pub fn new(total_time: f32, steps: usize) -> Self {
        Self {
            cool_down: total_time / steps as f32,
            steps,
            cd: 0.0,
            n: 0,
        }
    }

    pub fn noop() -> Self {
        Self {
            cool_down: 10.0_f32.powf(6.0),
            steps: 1,
            cd: 10.0_f32.powf(6.0),
            n: 0,
        }
    }
}

impl EffectTimer for LoopTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.cd == self.cool_down
    }

    fn n(&self) -> usize {
        self.n
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.cd -= time;
        if gaurd.second && self.is_cd_off() {
            self.cd = self.cool_down;
            self.n += 1;
            if self.n > self.steps {
                self.n = 0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.n = 0;
        self.cd = 0.0;
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
        self.n_hits += 1;
        if self.n_hits >= 3 {
            self.n_hits = 0;
        }
    }

    pub fn update(&mut self, should_update: bool, time: f32) -> () {
        if self.counting {
            self.cd += time;
        }
        if self.cd >= 2.5 {
            self.n_hits = 0;
            self.cd = 0.0;
            self.counting = false;
        }
        if self.n_hits >= 3 {
            self.n_hits = 0;
        }
        // TODO counter increases if attack is infused (dont function on physical attack)
        if should_update {
            self.counting = true;
        }
    }
}

pub trait TimerGuardCheck<T> {
    fn check(&self, timer: T) -> bool;
}

#[derive(Debug)]
pub struct TimerGuard {
    pub kind: AttackType,

    // 1. the `Attack` was created by this owner
    // example: attack.on_field_character_index == fc.idx.0
    pub first:  bool,

    // 2. the `Attack` is the same kind as this action
    // alternatively, `should_update` condition is satisfied
    // example: na.action == attack.kind
    pub second: bool,

    // 3. action's timer is cooling down
    // example: timer.cd > 0.0
    pub third: bool,

    // If the 1st and 2nd conditions are true, it means this skill, timer or
    // ability was used. Otherwise, `TimerGuard` needs to check the 3rd
    // condition. If all the conditions are false, it means the ability has not
    // been used yet. In that case, the timer should should not update itself.
}

impl TimerGuard {
    // test?
    fn first_ok() -> Self {
        Self {
            kind: StandStill,
            first: true,
            second: false,
            third: false,
        }
    }

    pub fn with_first(attack: &Attack, fc: &CharacterData) -> Self {
        Self {
            kind: attack.kind, // should be cheap
            first: attack.idx == fc.idx,
            second: false,
            third: false,
        }
    }

    // TODO refactor the method
    pub fn with_first_2(attack: &AttackEvent, fc: &CharacterData) -> Self {
        Self {
            kind: attack.kind, // should be cheap
            first: attack.idx.0 == fc.idx.0,
            second: false,
            third: false,
        }
    }

    // pub fn first(mut self, first: bool) -> Self {
    //     self.first = first;
    //     self
    // }

    pub fn second(&mut self, second: bool) -> &mut Self {
        self.second = second;
        self
    }

    pub fn check_second(&mut self, attack: AttackType) -> &mut Self {
        self.second = self.kind == attack;
        self
    }

    pub fn third(&mut self, third: bool) -> &mut Self {
        self.third = third;
        self
    }
}


impl<T: EffectTimer> TimerGuardCheck<&T> for TimerGuard {
    fn check(&self, timer: &T) -> bool {
        self.first && self.second || !timer.is_cd_off()
    }
}

// for ad-hoc types which cannot implement `EffectTimer`
impl TimerGuardCheck<()> for TimerGuard {
    fn check(&self, _timer: ()) -> bool {
        self.first && self.second || self.third
    }
}

#[derive(Debug)]
pub struct StaminaTimer {
    stamina: f32,
    recovery: bool,
    consumption: f32,
}

impl StaminaTimer {
    pub fn new(consumption: f32) -> Self {
        Self {
            stamina: 240.0,
            recovery: false,
            consumption,
        }
    }

    pub fn noop() -> Self {
        Self {
            stamina: 240.0,
            recovery: false,
            consumption: 0.0,
        }
    }
}

impl EffectTimer for StaminaTimer {
    fn is_cd_off(&self) -> bool {
        !self.recovery && 0.0 < self.stamina
    }

    fn is_active(&self) -> bool {
        !self.recovery && 0.0 < self.stamina
    }

    fn n(&self) -> usize {
        self.stamina as usize
    }

    fn update(&mut self, gaurd: &TimerGuard, time: f32) -> () {
        if !gaurd.check(&*self) {
            return;
        }
        self.stamina += time * 10.0; // TODO recovery rate of energy
        if self.stamina >= 240.0 {
            self.recovery = false;
        }
        if gaurd.second && self.is_cd_off() {
            self.stamina -= self.consumption;
        }
        if self.stamina <= 0.0 {
            self.recovery = true;
        }
    }

    fn reset(&mut self) -> () {
        self.stamina = 240.0;
        self.recovery = false;
    }
}


pub trait CharacterTimers {
    fn maybe_attack(&self, fc: &CharacterData, ca: &dyn CharacterAbility) -> Option<AttackType>;
    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[ElementalAttack], fc: &CharacterData, time: f32) -> ();
    fn na_timer(&self) -> &LoopTimer;
    fn ca_timer(&self) -> &HitsTimer;
    fn press_timer(&self) -> &DotTimer;
    fn hold_timer(&self) -> &DotTimer;
    fn burst_timer(&self) -> &DotTimer;
    fn reset_cd(&mut self) -> ();
}

pub trait Acceleration {
    fn reset_cd(&mut self) -> ();
}

#[derive(Debug)]
pub struct NaTimers<const B: bool> {
    pub timer: LoopTimer,
    pub icd: ICDTimer,
}

impl NaTimers<true> {
    pub fn new(timer: LoopTimer) -> Self {
        Self {
            timer,
            icd: ICDTimer::new()
        }
    }

    pub fn should_attack(&self) -> bool {
        self.timer.is_cd_off()
    }

    pub fn is_active(&self) -> bool {
        self.timer.is_active()
    }

    pub fn n(&self) -> usize {
        self.timer.n()
    }

    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () {
        self.timer.update(gaurd.check_second(Na), time);
        self.icd.update(icd_should_update, time);
    }
}

impl NaTimers<false> {
    pub fn new() -> Self {
        Self {
            timer: LoopTimer::noop(),
            icd: ICDTimer::new()
        }
    }

    pub fn should_attack(&self) -> bool { false }
    pub fn is_active(&self) -> bool { false }
    pub fn n(&self) -> usize { 0 }
    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () { }
}

#[derive(Debug)]
pub struct CaTimers<const B: bool> {
    pub stamina: StaminaTimer,
    pub timer: HitsTimer,
    pub icd: ICDTimer,
}

impl CaTimers<true> {
    pub fn new(timer: HitsTimer, stamina: StaminaTimer) -> Self {
        Self {
            stamina,
            timer,
            icd: ICDTimer::new()
        }
    }

    pub fn should_attack(&self) -> bool {
        self.timer.is_cd_off() && self.stamina.is_active()
    }

    pub fn is_active(&self) -> bool {
        self.timer.is_active() && self.stamina.is_active()
    }

    pub fn n(&self) -> usize {
        self.timer.n()
    }

    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () {
        self.stamina.update(gaurd.check_second(Ca), time);
        self.timer.update(gaurd, time);
        self.icd.update(icd_should_update, time);
    }
}

impl CaTimers<false> {
    pub fn new() -> Self {
        Self {
            stamina: StaminaTimer::noop(),
            timer: HitsTimer::noop(),
            icd: ICDTimer::new()
        }
    }

    pub fn should_attack(&self) -> bool { false }
    pub fn is_active(&self) -> bool { false }
    pub fn n(&self) -> usize { 0 }
    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () { }
}

// N = 1: only press skill is used
// N = 2: both press and hold skills are used
// N = _: for any other values, `SkillTimers` are disabled
#[derive(Debug)]
pub struct SkillTimers<const N: usize> {
    pub press: DotTimer,
    pub hold: DotTimer,
    pub icd: ICDTimer,
}

impl SkillTimers<1> {
    pub fn new(press: DotTimer) -> Self {
        Self {
            press,
            hold: DotTimer::noop(),
            icd: ICDTimer::new()
        }
    }

    pub fn should_press(&self) -> bool {
        self.press.is_cd_off()
    }

    pub fn should_hold(&self) -> bool {
        false
    }

    pub fn is_active(&self) -> bool {
        self.press.is_active()
    }

    pub fn n(&self) -> usize {
        self.press.n()
    }

    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () {
        self.press.update(gaurd.check_second(PressSkill), time);
        self.icd.update(icd_should_update, time);
    }

    pub fn reset_cd(&mut self) -> () {
        self.press.reset();
    }
}

impl SkillTimers<2> {
    pub fn new(press: DotTimer, hold: DotTimer) -> Self {
        Self {
            press,
            hold,
            icd: ICDTimer::new()
        }
    }

    pub fn should_press(&self) -> bool {
        // TODO Because hold CD is longer than press CD, hold skill needs to be off to use press skill
        self.press.is_cd_off() && self.hold.is_cd_off()
    }

    pub fn should_hold(&self) -> bool {
        self.hold.is_cd_off()
    }

    pub fn is_active(&self) -> bool {
        self.press.is_active() || self.hold.is_active()
    }

    pub fn n(&self) -> usize {
        self.hold.n()
    }

    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () {
        self.press.update(gaurd.check_second(PressSkill), time);
        self.hold.update(gaurd.check_second(HoldSkill), time);
        self.icd.update(icd_should_update, time);
    }

    pub fn reset_cd(&mut self) -> () {
        self.press.reset();
        self.hold.reset();
    }
}

impl SkillTimers<0> {
    pub fn new() -> Self {
        Self {
            press: DotTimer::noop(),
            hold: DotTimer::noop(),
            icd: ICDTimer::new()
        }
    }

    pub fn should_press(&self) -> bool { false }
    pub fn should_hold(&self) -> bool { false }
    pub fn is_active(&self) -> bool { false }
    pub fn n(&self) -> usize { 0 }
    pub fn update(&mut self, gaurd: &mut TimerGuard, icd_should_update: bool, time: f32) -> () {}
    pub fn reset_cd(&mut self) -> () {}
}

#[derive(Debug)]
pub struct FullCharacterTimers<const N: bool, const C: bool, const S: usize> {
    pub na: NaTimers<N>,

    pub ca: CaTimers<C>,

    pub skill: SkillTimers<S>,

    pub burst_timer: DotTimer,
    pub burst_icd: ICDTimer,
}

impl<const N: bool, const C: bool, const S: usize> CharacterTimers for FullCharacterTimers<N, C, S> {
    fn maybe_attack(&self, fc: &CharacterData, ca: &dyn CharacterAbility) -> Option<AttackType> {
        // na combo blocks other actions.
        if self.na.n() > 0 && self.na.is_active() {
            Some(Na)
        } else if fc.can_burst() && self.burst_timer.is_cd_off() {
            Some(Burst)
        } else if ca.use_hold() && self.skill.should_hold() {
            Some(HoldSkill)
        } else if self.skill.should_press() {
            Some(PressSkill)
        } else if self.ca.should_attack() {
            Some(Ca)
        } else if self.na.should_attack() {
            Some(Na)
        } else {
            None
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[ElementalAttack], fc: &CharacterData, time: f32) -> () {
        let mut na = false;
        let mut ca = false;
        let mut skill = false;
        let mut burst = false;
        unsafe {
            for &a in attack {
                match &(*(a.atk)).kind {
                    Na => na = true,
                    Ca => ca = true,
                    PressSkill | HoldSkill | SkillDot => skill = true,
                    Burst | BurstDot => burst = true,
                    _ => (),
                }
            }
        }
        self.na.update(gaurd, na, time * (1.0 + fc.state.atk_spd / 100.0));
        self.ca.update(gaurd, ca, time);
        self.skill.update(gaurd, skill, time);
        // 
        // self.na_icd.update(na, time);
        // self.ca_icd.update(ca, time);
        // self.skill_icd.update(skill, time);
        self.burst_icd.update(burst, time);
        // self.na_timer.update();
        // self.ca_timer.update(gaurd.check_second(Ca), time);
        // self.stamina.update(gaurd, time);
        // self.press_timer.update(gaurd.check_second(PressSkill), time);
        // self.hold_timer.update(gaurd.check_second(HoldSkill), time);
        self.burst_timer.update(gaurd.check_second(Burst), time);
    }

    fn na_timer(&self) -> &LoopTimer {
        &self.na.timer
    }

    fn ca_timer(&self) -> &HitsTimer {
        &self.ca.timer
    }

    fn press_timer(&self) -> &DotTimer {
        &self.skill.press
    }

    fn hold_timer(&self) -> &DotTimer {
        &self.skill.hold
    }

    fn burst_timer(&self) -> &DotTimer {
        &self.burst_timer
    }

    fn reset_cd(&mut self) -> () {
        self.skill.reset_cd();
    }
}

// impl Acceleration for FullCharacterTimers {
//     fn reset_cd(&mut self) -> () {
//         self.press_timer.reset();
//         self.hold_timer.reset();
//     }
// }

#[derive(Debug)]
pub struct CharacterTimersBuilder {
    na_timer: Option<LoopTimer>,
    ca_timer: Option<HitsTimer>,
    ca_stamina: Option<StaminaTimer>,
    press_timer: Option<DotTimer>,
    hold_timer:  Option<DotTimer>,
    burst_timer: Option<DotTimer>,
}

impl CharacterTimersBuilder {
    pub fn new() -> Self {
        Self {
            na_timer: None,
            ca_timer: None,
            ca_stamina: None,
            press_timer: None,
            hold_timer:  None,
            burst_timer: None,
        }
    }

    pub fn na(mut self, t: LoopTimer) -> Self {
        self.na_timer = Some(t);
        self
    }

    pub fn ca(mut self, t: HitsTimer) -> Self {
        self.ca_timer = Some(t);
        self
    }

    pub fn ca_stamina(mut self, t: StaminaTimer) -> Self {
        self.ca_stamina = Some(t);
        self
    }

    pub fn press(mut self, t: DotTimer) -> Self {
        self.press_timer = Some(t);
        self
    }

    pub fn hold(mut self, t: DotTimer) -> Self {
        self.hold_timer = Some(t);
        self
    }

    pub fn burst(mut self, t: DotTimer) -> Self {
        self.burst_timer = Some(t);
        self
    }

    pub fn build_burst(self) -> FullCharacterTimers<false, false, 0> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<false>::new(),
            ca: CaTimers::<false>::new(),
            skill: SkillTimers::<0>::new(),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_press(self) -> FullCharacterTimers<false, false, 1> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<false>::new(),
            ca: CaTimers::<false>::new(),
            skill: SkillTimers::<1>::new(press_timer.unwrap()),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_hold(self) -> FullCharacterTimers<false, false, 2> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<false>::new(),
            ca: CaTimers::<false>::new(),
            skill: SkillTimers::<2>::new(press_timer.unwrap(), hold_timer.unwrap()),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_na(self) -> FullCharacterTimers<true, false, 0> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<true>::new(na_timer.unwrap()),
            ca: CaTimers::<false>::new(),
            skill: SkillTimers::<0>::new(),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_na_press(self) -> FullCharacterTimers<true, false, 1> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<true>::new(na_timer.unwrap()),
            ca: CaTimers::<false>::new(),
            skill: SkillTimers::<1>::new(press_timer.unwrap()),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_na_hold(self) -> FullCharacterTimers<true, false, 2> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<true>::new(na_timer.unwrap()),
            ca: CaTimers::<false>::new(),
            skill: SkillTimers::<2>::new(press_timer.unwrap(), hold_timer.unwrap()),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_na_ca(self) -> FullCharacterTimers<true, true, 0> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<true>::new(na_timer.unwrap()),
            ca: CaTimers::<true>::new(ca_timer.unwrap(), ca_stamina.unwrap()),
            skill: SkillTimers::<0>::new(),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_na_ca_press(self) -> FullCharacterTimers<true, true, 1> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<true>::new(na_timer.unwrap()),
            ca: CaTimers::<true>::new(ca_timer.unwrap(), ca_stamina.unwrap()),
            skill: SkillTimers::<1>::new(press_timer.unwrap()),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }

    pub fn build_full(self) -> FullCharacterTimers<true, true, 2> {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            ca_stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        FullCharacterTimers {
            na: NaTimers::<true>::new(na_timer.unwrap()),
            ca: CaTimers::<true>::new(ca_timer.unwrap(), ca_stamina.unwrap()),
            skill: SkillTimers::<2>::new(press_timer.unwrap(), hold_timer.unwrap()),
            burst_timer: burst_timer.unwrap(),
            burst_icd: ICDTimer::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn durationtimer() -> DurationTimer {
        DurationTimer::new(0.3, 2.0)
    }

    #[test]
    fn durationtimer_0() {
        let mut timer = durationtimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        assert_eq!(timer.is_active(), true);
    }

    #[test]
    fn durationtimer_1() {
        let mut timer = durationtimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 2.0);
        assert_eq!(timer.is_active(), false);
    }

    fn hitstimer() -> HitsTimer {
        HitsTimer::new(1.0, 2)
    }

    #[test]
    fn hitstimer_0() {
        let mut timer = hitstimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        assert_eq!(timer.is_active(), true);
    }

    #[test]
    fn hitstimer_1() {
        let mut timer = hitstimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(true), 0.3);
        assert_eq!(timer.is_active(), true);
    }

    #[test]
    fn hitstimer_2() {
        let mut timer = hitstimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(true), 0.3);
        timer.update(gaurd.second(true), 0.3);
        assert_eq!(timer.is_active(), false);
    }

    fn dottimer() -> DotTimer {
        DotTimer::new(3.0, 0.5, 2)
    }

    #[test]
    fn dottimer_0() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        assert_eq!(timer.is_active(), true);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn dottimer_1() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.2);
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn dottimer_2() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.5);
        assert_eq!(timer.is_active(), true);
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn dottimer_3() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.5);
        timer.update(gaurd.second(false), 0.5);
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn dottimer_4() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.5);
        timer.update(gaurd.second(false), 0.5);
        timer.update(gaurd.second(false), 0.5);
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 0);
    }

    fn stacktimer() -> StackTimer {
        StackTimer::new(0.3, 3.0, 2)
    }

    #[test]
    fn stacktimer_0() {
        let mut timer = stacktimer();
        let mut gaurd = TimerGuard::first_ok();
        // too fast to get stacks
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn stacktimer_1() {
        let mut timer = stacktimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.3);
        timer.update(gaurd.second(true), 0.3);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn stacktimer_2() {
        let mut timer = stacktimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.3);
        timer.update(gaurd.second(true), 0.3);
        timer.update(gaurd.second(true), 0.3);
        // cannot exceed the max level
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn stacktimer_3() {
        let mut timer = stacktimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.3);
        timer.update(gaurd.second(true), 0.3);
        assert_eq!(timer.n, 2);
        // expire
        timer.update(gaurd.second(false), 3.0);
        assert_eq!(timer.n, 0);
    }

    fn sigiltimer() -> SigilTimer {
        SigilTimer::new(0.1, 5.0, 3.0, 3)
    }

    #[test]
    fn sigiltimer_1() {
        let mut timer = sigiltimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        // sigil enterd effect CD
        timer.update(gaurd.second(true), 2.0);
        assert_eq!(timer.n, 3);
    }

    #[test]
    fn sigiltimer_2() {
        let mut timer = sigiltimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        // sigil expired effect duration
        timer.update(gaurd.second(true), 3.0);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn sigiltimer_3() {
        let mut timer = sigiltimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        // cannot gain another sigil because it is in CD
        timer.update(gaurd.second(true), 4.0);
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn sigiltimer_4() {
        let mut timer = sigiltimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        timer.update(gaurd.second(true), 0.1);
        // can gain another sigil
        // note the last stack only takes effect
        timer.update(gaurd.second(true), 5.0);
        assert_eq!(timer.n, 1);
        timer.update(gaurd.second(true), 0.1);
        assert_eq!(timer.n, 2);
    }

    fn looptimer() -> LoopTimer {
        LoopTimer::new(3.0, 3)
    }

    #[test]
    fn looptimer_1() {
        let mut timer = looptimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn looptimer_too_early_1() {
        let mut timer = looptimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.3);
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn looptimer_too_early_2() {
        let mut timer = looptimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(true), 0.3);
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn looptimer_2() {
        let mut timer = looptimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(true), 1.0);
        assert_eq!(timer.n, 2);
    }
}
