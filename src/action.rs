use std::mem;
use std::ptr;
use crate::fc::{FieldCharacter, Enemy, Debuff};
use crate::types::{AttackType, Vision, ElementalReactionType, ElementalReaction};
use crate::state::State;

use AttackType::*;
use Vision::*;

#[derive(Debug)]
pub struct Attack {
    // type of this `Attack`. For example, Xiangling's skill summons Guoba to
    // deal DoT Pyro DMG. This DMG is considered as an additional attack and
    // since it is created by her skill, the `kind` is `AttackType::Skill`.
    pub kind: AttackType,

    // Infused element of this `Attack`. This also means elements of `particle`.
    pub element: Vision,

    pub multiplier: f32,

    // a certain amount of energy particles generated by skill hits.
    pub particle: Option<f32>,

    // an ad-hoc `State` specific to this `Attack`. For example, Some abilities
    // increase CR of a specific action: Amber A1 (Every Arrow Finds Its
    // Target), Ganyu A1 (Undivided Heart), Festering Desire.
    pub state: Option<State>,

    pub icd_cleared: bool,

    pub on_field_character_index: usize,
    pub fc_ptr: *const FieldCharacter,
}

trait NewAttack<T> {
    fn new(action: &T, multiplier: f32, fc: &FieldCharacter) -> Attack;
}

impl NewAttack<NormalAttackAction> for Attack {
    fn new(action: &NormalAttackAction, multiplier: f32, fc: &FieldCharacter) -> Self {
        let element = if fc.state.infusion
                      || action.action == Ca && fc.cr.weapon == "Bow" {
            fc.vision
        } else {
            Physical
        };
        Self {
            kind: action.action,
            element,
            multiplier,
            particle: None,
            state: None,
            icd_cleared: action.icd.clear(),
            on_field_character_index: fc.idx.0,
            fc_ptr: fc,
        }
    }
}

impl NewAttack<SkillAction> for Attack {
    fn new(action: &SkillAction, multiplier: f32, fc: &FieldCharacter) -> Self {
        Self {
            kind: Skill,
            element: fc.vision,
            multiplier,
            particle: Some(action.particle),
            state: None,
            icd_cleared: action.icd.clear(),
            on_field_character_index: fc.idx.0,
            fc_ptr: fc,
        }
    }
}

impl NewAttack<BurstAction> for Attack {
    fn new(action: &BurstAction, multiplier: f32, fc: &FieldCharacter) -> Self {
        Self {
            kind: Burst,
            element: fc.vision,
            multiplier,
            particle: None,
            state: None,
            icd_cleared: action.icd.clear(),
            on_field_character_index: fc.idx.0,
            fc_ptr: fc,
        }
    }
}

impl Attack {
    pub fn empty() -> Self {
        Self {
            kind: StandStill,
            element: Physical,
            multiplier: 0.0,
            particle: None,
            state: None,
            on_field_character_index: 0,
            icd_cleared: true,
            fc_ptr: ptr::null(),
        }
    }

    pub fn owned(&self, owner_fc: &FieldCharacter) -> bool {
        self.on_field_character_index == owner_fc.idx.0
    }

    pub fn is_na(&self) -> bool {
        self.kind == AttackType::Na
    }

    // pub fn is_ca(&self) -> bool {
    //     self.kind == AttackType::Ca
    // }

    pub fn is_naca(&self) -> bool {
        self.kind == AttackType::Na || self.kind == AttackType::Ca
    }

    pub fn is_skill(&self) -> bool {
        self.kind == AttackType::Skill
    }

    // pub fn is_burst(&self) -> bool {
    //     self.kind == AttackType::Burst
    // }

    // pub fn is_skilldot(&self) -> bool {
    //     self.kind == AttackType::SkillDot
    // }

    // pub fn is_burstdot(&self) -> bool {
    //     self.kind == AttackType::BurstDot
    // }

    pub fn outgoing_damage(&mut self, fc: &FieldCharacter) -> f32 {
        let mut state: Option<State> = None;
        mem::swap(&mut state, &mut self.state);
        // use ad-hoc state if available
        if let Some(mut state) = state {
            state.merge(&fc.state);
            self.outgoing_damage_inner(&state, fc)
        } else {
            self.outgoing_damage_inner(&fc.state, fc)
        }
    }

    fn outgoing_damage_inner(&self, state: &State, fc: &FieldCharacter) -> f32 {
        let bonus = state.DMG_bonus(&self.kind, &self.element);
        let crcd = state.CRCD();
        let atk = match (fc.cr.name.as_str(), self.kind) {
            ("Albedo", AttackType::SkillDot) => state.DEF(),
            ("Noelle", AttackType::Skill) => state.DEF(),
            _ => state.ATK(),
        };
        let power = atk * bonus * crcd;
        self.multiplier / 100.0 * power
    }

    pub fn incoming_damage(&self, outgoing_damage: f32, fc: &FieldCharacter, enemy: &mut Enemy) -> f32 {
        let res = if self.element == Vision::Physical {
            let physical_res = enemy.get_physical_res();
            if physical_res > enemy.physical_res {
                -0.5 * (physical_res - enemy.physical_res)
            } else {
                enemy.physical_res - physical_res
            }
        } else {
            let element_res = enemy.get_element_res();
            if element_res > enemy.element_res {
                -0.5 * (element_res - enemy.element_res)
            } else {
                enemy.element_res - element_res
            }
        };
        let def_down = 1.0 + enemy.get_def_down() / 100.0;
        let level_multiplier = enemy.level / (enemy.level * def_down + enemy.level);
        self.elemental_reaction(outgoing_damage * (100.0 - res) / 100.0 * level_multiplier, fc, enemy)
    }

    // https://genshin-impact.fandom.com/wiki/Elemental_Reactions
    pub fn elemental_reaction(&self, outgoing_damage: f32, fc: &FieldCharacter, enemy: &mut Enemy) -> f32 {
        use ElementalReactionType::*;
        if self.icd_cleared {
            let elemental_reaction = ElementalReaction::new(enemy.aura.aura, self.element);
            let dmg = match elemental_reaction {
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
            enemy.aura.trigger(self);
            if let Freeze(_) = elemental_reaction {
                enemy.isfrozen = true;
            }
            if let Superconduct(_) = elemental_reaction {
                enemy.physical_res_debuff.push(Debuff::superconduct());
            }
            dmg
        } else {
            outgoing_damage
        }
    }
}

pub trait EffectTimer {
    fn is_cd_off(&self) -> bool;
    fn is_active(&self) -> bool;
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
}

impl EffectTimer for HitsTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.n > 0
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
    n_hits: i32,
    cd: f32,
    dcd: f32,
    dcd_cleared: bool,
    n: i32,
}

impl DotTimer {
    pub fn new(cool_down: f32, dot_cd: f32, n_hits: i32) -> Self {
        Self { cool_down, dot_cd, n_hits, cd: 0.0, dcd: 0.0, dcd_cleared: false, n: 0 }
    }
}

impl EffectTimer for DotTimer {
    fn is_cd_off(&self) -> bool {
        self.cd <= 0.0
    }

    fn is_active(&self) -> bool {
        self.n > 0 && self.dcd_cleared
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
            self.n = self.n_hits + 1;
            self.dcd_cleared = false;
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
pub struct ICDTimer {
    cd: f32,
    n_hits: usize,
    counting: bool,
    action_1: AttackType,
    action_2: AttackType,
}

impl ICDTimer {
    pub fn na() -> Self {
        Self {
            cd: 0.0,
            n_hits: 0,
            counting: false,
            action_1: Na,
            action_2: Na,
        }
    }

    pub fn ca() -> Self {
        Self {
            cd: 0.0,
            n_hits: 0,
            counting: false,
            action_1: Ca,
            action_2: Ca,
        }
    }

    pub fn skill() -> Self {
        Self {
            cd: 0.0,
            n_hits: 0,
            counting: false,
            action_1: Skill,
            action_2: SkillDot,
        }
    }

    pub fn burst() -> Self {
        Self {
            cd: 0.0,
            n_hits: 0,
            counting: false,
            action_1: Burst,
            action_2: BurstDot,
        }
    }

    pub fn clear(&self) -> bool {
        self.cd == 0.0 || self.n_hits == 0
    }

    pub fn update(&mut self, multiplier: f32, attack: &Attack, aa: &[Attack], time: f32) -> () {
        if self.cd >= 2.5 {
            self.n_hits = 0;
            self.cd = 0.0;
            self.counting = false;
        }
        if self.n_hits >= 3 {
            self.n_hits = 0;
        }
        // TODO counter increases if attack is infused (dont function on physical attack)
        if attack.kind == self.action_1 && multiplier > 0.0 {
            // TODO n_hits of each action?
            self.n_hits += 1;
            self.counting = true;
        } else if aa.iter().any(|a| a.kind == self.action_1 || a.kind == self.action_2) {
            self.n_hits += 1;
            self.counting = true;
        }
        if self.counting {
            self.cd += time;
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

    pub fn with_first(attack: &Attack, fc: &FieldCharacter) -> Self {
        Self {
            kind: attack.kind, // should be cheap
            first: attack.on_field_character_index == fc.idx.0,
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

    pub fn third(&mut self, third: bool) -> &mut Self {
        self.third = third;
        self
    }
}

impl TimerGuardCheck<&NormalAttackAction> for TimerGuard {
    fn check(&self, timer: &NormalAttackAction) -> bool {
        self.first
        && (self.second || self.kind == timer.action)
        || timer.cd > 0.0
    }
}

impl TimerGuardCheck<&SkillAction> for TimerGuard {
    fn check(&self, timer: &SkillAction) -> bool {
        self.first
        && (self.second || self.kind == AttackType::Skill)
        || timer.cd > 0.0
    }
}

impl TimerGuardCheck<&BurstAction> for TimerGuard {
    fn check(&self, timer: &BurstAction) -> bool {
        self.first
        && (self.second || self.kind == AttackType::Burst)
        || timer.cd > 0.0
    }
}

// impl TimerGuardCheck<&CDTimer> for TimerGuard {
//     fn check(&self, timer: &CDTimer) -> bool {
//         self.first && self.second || !timer.is_cd_off()
//     }
// }

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

// 
// Actions
// 

#[derive(Debug)]
pub struct NormalAttackAction {
    action: AttackType,
    element: Vision,

    states: Vec<f32>,
    idx: usize,

    cool_down: f32,
    cd: f32,
    pub spd: f32,

    pub icd: ICDTimer,
}

impl NormalAttackAction {
    pub fn na(element: Vision, cd: f32, states: Vec<f32>) -> Self {
        Self {
            action: AttackType::Na,
            element,
            cool_down: cd,
            states,
            cd: 0.0,
            idx: 0,
            spd: 0.0,
            icd: ICDTimer::na(),
        }
    }

    pub fn ca(element: Vision, cd: f32, value: f32) -> Self {
        Self {
            action: AttackType::Ca,
            element,
            cool_down: cd,
            states: vec![value],
            cd: 0.0,
            idx: 0,
            spd: 0.0,
            icd: ICDTimer::ca(),
        }
    }

    pub fn noop() -> Self {
        Self {
            action: AttackType::StandStill,
            element: Vision::Physical,
            cool_down: 10.0_f32.powf(6.0),
            states: vec![0.0],
            cd: 10.0_f32.powf(6.0),
            idx: 0,
            spd: 0.0,
            icd: ICDTimer::na(),
        }
    }

    pub fn value(&self, fc: &FieldCharacter) -> Option<Attack> {
        if self.cd > 0.0 {
            None
        } else {
            Some(Attack::new(self, self.states[self.idx], fc))
        }
    }

    pub fn update(&mut self, gaurd: &TimerGuard, attack: &Attack, aa: &[Attack], time: f32) {
        if !gaurd.check(&*self) {
            return;
        }
        if (attack.kind == AttackType::Na || attack.kind == AttackType::Ca) && self.cd <= 0.0 {
            self.cd = self.cool_down;
            if self.idx == self.states.len() - 1 {
                self.idx = 0;
            } else {
                self.idx += 1;
            }
        }
        // TODO this should be ok because it always has some non-negative value
        self.icd.update(1.0, attack, aa, time);
        // once `spd` was used, clear it
        self.cd -= time * (1.0 + self.spd / 100.0);
        self.spd = 0.0;
    }
}

#[derive(Debug)]
pub struct SkillAction {
    element: Vision,
    particle: f32,
    value: f32,

    cool_down: f32,
    pub cd: f32,
    pub spd: f32,

    pub icd: ICDTimer,
}

impl SkillAction {
    pub fn new(element: Vision, particle: f32, cool_down: f32, value: f32) -> Self {
        Self {
            element,
            particle,
            cool_down,
            value,
            cd: 0.0,
            spd: 0.0,
            icd: ICDTimer::skill(),
        }
    }

    pub fn noop(element: Vision) -> Self {
        Self {
            element,
            particle: 0.0,
            cool_down: 10.0_f32.powf(6.0),
            value: 0.0,
            cd: 10.0_f32.powf(6.0),
            spd: 0.0,
            icd: ICDTimer::skill(),
        }
    }

    pub fn value(&self, fc: &FieldCharacter) -> Option<Attack> {
        if self.cd > 0.0 {
            None
        } else {
            Some(Attack::new(self, self.value, fc))
        }
    }

    pub fn update(&mut self, gaurd: &TimerGuard, attack: &Attack, aa: &[Attack], time: f32) {
        if !gaurd.check(&*self) {
            return;
        }
        if attack.kind == AttackType::Skill && self.cd <= 0.0 {
            self.cd = self.cool_down;
        }
        self.icd.update(self.value, attack, aa, time);
        // once `spd` was used, clear it
        self.cd -= time * (1.0 + self.spd / 100.0);
        self.spd = 0.0;
    }
}

#[derive(Debug)]
pub struct BurstAction {
    element: Vision,
    energy_cost: f32,
    value: f32,

    cool_down: f32,
    cd: f32,

    pub icd: ICDTimer,
}

impl BurstAction {
    pub fn new(element: Vision, cool_down: f32, energy_cost: f32, value: f32) -> Self {
        Self {
            element,
            energy_cost,
            cool_down,
            value,
            cd: 0.0,
            icd: ICDTimer::burst(),
        }
    }

    pub fn value(&self, fc: &FieldCharacter) -> Option<Attack> {
        if self.cd > 0.0 || fc.state.energy.0 < self.energy_cost {
            None
        } else {
            Some(Attack::new(self, self.value, fc))
        }
    }

    pub fn update(&mut self, gaurd: &TimerGuard, attack: &Attack, aa: &[Attack], time: f32) {
        if !gaurd.check(&*self) {
            return;
        }
        if attack.kind == AttackType::Burst && self.cd <= 0.0 {
            self.cd = self.cool_down;
        }
        self.icd.update(self.value, attack, aa, time);
        self.cd -= time;
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
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 3);
    }

    #[test]
    fn dottimer_1() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.5);
        assert_eq!(timer.is_active(), true);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn dottimer_2() {
        let mut timer = dottimer();
        let mut gaurd = TimerGuard::first_ok();
        timer.update(gaurd.second(true), 0.0);
        timer.update(gaurd.second(false), 0.5);
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
}