use std::ptr;

use crate::fc::{CharacterData, FieldCharacterIndex, CharacterAbility, Enemy, Debuff};
use crate::types::{AttackType, Vision, BareElementalGauge, ElementalReactionType, ElementalReaction, GAUGE1A};
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

pub trait ElementalAttackVector {
    fn push_pyro(&mut self, data: &CharacterData, attack: *const Attack) -> ();
    fn push_hydro(&mut self, data: &CharacterData, attack: *const Attack) -> ();
    fn push_electro(&mut self, data: &CharacterData, attack: *const Attack) -> ();
    fn push_cryo(&mut self, data: &CharacterData, attack: *const Attack) -> ();
    fn push_anemo(&mut self, data: &CharacterData, attack: *const Attack) -> ();
    fn push_geo(&mut self, data: &CharacterData, attack: *const Attack) -> ();
    fn push_dendro(&mut self, data: &CharacterData, attack: *const Attack) -> ();
}

impl ElementalAttackVector for Vec<ElementalAttack> {
    fn push_pyro(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::pyro(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }

    fn push_hydro(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::hydro(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }

    fn push_electro(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::electro(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }

    fn push_cryo(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::cryo(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }

    fn push_anemo(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::anemo(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }

    fn push_geo(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::geo(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }

    fn push_dendro(&mut self, data: &CharacterData, attack: *const Attack) -> () {
        self.push(if data.state.infusion {
            ElementalAttack::dendro(attack)
        } else {
            ElementalAttack::physical(attack)
        });
    }
}

impl PartialEq<Vision> for ElementalAttack {
    fn eq(&self, other: &Vision) -> bool {
        self.element.eq(other)
    }
}

#[derive(Debug)]
pub struct ElementalAbsorption {
    element: Option<Vision>,
    timer: DurationTimer,
    attack: Attack,
}

impl ElementalAbsorption {
    pub fn new(idx: FieldCharacterIndex, kind: AttackType, multiplier: f32, timer: DurationTimer) -> Self {
        Self {
            element: None,
            timer,
            attack: Attack {
                kind,
                gauge: &GAUGE1A,
                multiplier,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }

    pub fn icd(&mut self) -> &mut *mut ICDTimer {
        &mut self.attack.icd_timer
    }

    pub fn did_absort(&self) -> bool {
        self.element.is_some()
    }

    pub fn absorb(&mut self, guard: &TimerGuard, enemy: &Enemy, time: f32) -> () {
        use Vision::*;
        self.timer.update(guard, time);
        if self.timer.is_active() {
            if self.element.is_none() {
                match &enemy.aura.aura {
                    Pyro | Hydro | Electro | Cryo => self.element = Some(enemy.aura.aura),
                    _ => (),
                }
            }
        } else if self.element.is_some() {
            self.element = None;
        }
    }

    pub fn attack(&self) -> Option<ElementalAttack> {
        if let Some(e) = self.element {
            Some(ElementalAttack::new(e, &self.attack))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Attack {
    // type of this `Attack`. For example, Xiangling's skill summons Guoba to
    // deal DoT Pyro DMG. This DMG is considered as an additional attack and
    // since it is created by her skill, the `kind` is `AttackType::Skill`.
    pub kind: AttackType,

    // elemental gauge of this `Attack`.
    pub gauge: &'static BareElementalGauge,

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
            ("Kokomi (HP scale)", AttackType::SkillDot) => state.HP(),
            ("Albedo", AttackType::SkillDot) => state.DEF(),
            ("Noelle", AttackType::PressSkill) => state.DEF(),
            _ => state.ATK(),
        };
        let power = atk * bonus * crcd;
        self.multiplier / 100.0 * power * state.get_talent_bonus(&self.kind)
    }

    pub fn incoming_damage(&self, attack_element: &Vision, outgoing_damage: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        let def_down = 1.0 - enemy.get_def_down() / 100.0;
        let enemy_defense = enemy.level / (enemy.level * def_down + enemy.level);
        let resistance = self.resistance(attack_element, &enemy);
        let dmg = outgoing_damage * resistance * enemy_defense;
        self.elemental_reaction(attack_element, dmg, resistance, fc, enemy)
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

    pub fn elemental_reaction(&self, attack_element: &Vision, outgoing_damage: f32, resistance: f32, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
        use ElementalReactionType::*;
        let mut total_dmg = 0.0;
        for _ in 0..self.hits {
            // weapons do not have ICD timers.
            if self.kind != AdditionalAttack && self.icd_cleared() {
                let elemental_reaction = ElementalReaction::new(enemy.aura.aura, *attack_element);
                total_dmg += match elemental_reaction {
                    Overloaded(ref er) |
                    Shatter(ref er) |
                    ElectorCharged(ref er) |
                    Swirl(ref er) |
                    Superconduct(ref er) => outgoing_damage + resistance * er.transformative_reaction(fc.state.em, fc.state.transformative_bonus),
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
                let icd_timer = unsafe { &mut *self.icd_timer };
                icd_timer.count_hit();
            } else {
                if self.kind != AdditionalAttack {
                    let icd_timer = unsafe { &mut *self.icd_timer };
                    icd_timer.count_hit();
                }
                total_dmg += outgoing_damage;
            }
        }
        total_dmg
    }
}

pub trait EffectTimer {
    fn is_cd_off(&self) -> bool;
    fn is_active(&self) -> bool;
    fn n(&self) -> usize;
    fn update(&mut self, guard: &TimerGuard, time: f32) -> ();
    fn reset(&mut self) -> ();
}

#[derive(Debug)]
pub struct NoopTimer;

impl EffectTimer for NoopTimer {
    fn is_cd_off(&self) -> bool { false }
    fn is_active(&self) -> bool { false }
    fn n(&self) -> usize { 0 }
    fn update(&mut self, _guard: &TimerGuard, _time: f32) -> () {}
    fn reset(&mut self) -> () {}
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if guard.second && self.is_cd_off() {
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if guard.second && self.is_cd_off() {
            self.cd = self.cool_down - time;
            self.dr = self.duration - time;
        } else {
            self.cd -= time;
            self.dr -= time;
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if self.n > 0 {
            self.n -= 1;
        }
        if guard.second && self.is_cd_off() {
            self.cd = self.cool_down - time;
            self.n = self.n_hits;
        } else {
            self.cd -= time;
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if 0 < self.n && self.n < self.n_hits && self.dcd_cleared {
            self.n += 1;
            self.dcd = self.dot_cd;
        } else if self.n == self.n_hits {
            self.n = 0;
        }
        if guard.second && self.is_cd_off() {
            self.cd = self.cool_down - time;
            self.dcd = self.dot_cd - time;
            self.n = 1;
            self.dcd_cleared = true;
        } else {
            self.cd -= time;
            self.dcd -= time;
            self.dcd_cleared = self.dcd <= 0.0;
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if guard.second && self.is_cd_off() {
            self.cd = self.cool_down - time;
            self.dr = self.duration - time;
            self.n += 1;
            if self.n > self.level {
                self.n = self.level;
            }
        } else {
            self.cd -= time;
            self.dr -= time;
        }
        if self.dr <= 0.0 {
            self.n = 0;
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
        Self {
            cool_down,
            effect_cd,
            effect_duration,
            max_level,
            cd: 0.0,
            n: 0,
            dr: 0.0,
        }
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if guard.second && self.cd <= 0.0 {
            self.cd = self.cool_down - time;
            // expire
            if self.is_active() && self.dr <= 0.0 {
                self.n = 0;
            } else {
                self.n += 1;
            }
            if self.is_active() {
                self.cd = self.effect_cd - time;
                self.dr = self.effect_duration - time;
            }
            if self.n > self.max_level {
                self.n = self.max_level;
            }
        } else {
            self.cd -= time;
            // expire
            if self.is_active() {
                self.dr -= time;
            }
            if self.is_active() && self.dr <= 0.0 {
                self.n = 0;
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
        // self.cd == self.cool_down
        self.is_cd_off() && self.n > 0
        // self.is_cd_off()
    }

    fn n(&self) -> usize {
        self.n
    }

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if guard.second && self.is_cd_off() {
            self.cd = self.cool_down - time;
            self.n += 1;
            if self.n > self.steps {
                self.n = 1;
            }
        } else {
            self.cd -= time;
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

    fn update(&mut self, guard: &TimerGuard, time: f32) -> () {
        if !guard.check(&*self) {
            return;
        }
        if self.stamina >= 240.0 {
            self.recovery = false;
        }
        if guard.second && self.is_cd_off() {
            // TODO recovery rate of energy
            self.stamina -= self.consumption + time * 25.0;
        } else {
            self.stamina += time * 10.0;
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
        (self.first && self.second) || !timer.is_cd_off() || timer.is_active()
    }
}

// for ad-hoc types which cannot implement `EffectTimer`
impl TimerGuardCheck<()> for TimerGuard {
    fn check(&self, _timer: ()) -> bool {
        (self.first && self.second) || self.third
    }
}

#[derive(Debug)]
pub struct FullCharacterTimers {
    noop: NoopTimer,

    define_na: bool,
    na: LoopTimer,
    pub na_icd: ICDTimer,

    define_ca: bool,
    stamina: StaminaTimer,
    ca: HitsTimer,
    pub ca_icd: ICDTimer,

    define_press: bool,
    define_hold: bool,
    press: DotTimer,
    hold: DotTimer,
    pub skill_icd: ICDTimer,

    burst: DotTimer,
    pub burst_icd: ICDTimer,
}

impl FullCharacterTimers {
    pub fn maybe_attack(&self, fc: &CharacterData, ca: &dyn CharacterAbility) -> Option<AttackType> {
        // na combo blocks other actions.
        // if self.define_na && self.na.n() > 0 && self.na.is_active() {
        //     Some(Na)
        // } else if fc.can_burst() && self.burst.is_cd_off() {
        if fc.can_burst() && self.burst.is_cd_off() {
            Some(Burst)
        } else if self.define_hold && ca.use_hold() && self.should_hold() {
            Some(HoldSkill)
        } else if self.define_press && self.should_press() {
            Some(PressSkill)
        } else if self.define_ca && ca.use_ca() && self.should_ca() {
            Some(Ca)
        } else if self.define_na && self.should_na() {
            Some(Na)
        } else {
            None
        }
    }

    pub fn disable_naca(&mut self) -> () {
        self.define_na = false;
        self.define_ca = false;
    }

    pub fn decelerate_naca(&mut self, r: f32) -> () {
        self.na.cool_down *= r;
        self.ca.cool_down *= r;
    }

    pub fn update(&mut self, guard: &mut TimerGuard, _attack: &[ElementalAttack], fc: &CharacterData, time: f32) -> () {
        if self.define_na {
            self.na.update(guard.check_second(Na), time * (1.0 + fc.state.atk_spd / 100.0));
            self.na_icd.update(time);
        }

        if self.define_ca {
            self.ca.update(guard.check_second(Ca), time);
            self.stamina.update(guard, time);
            self.ca_icd.update(time);
        }

        if self.define_press && self.define_hold {
            self.hold.update(guard.check_second(HoldSkill), time);
            self.press.update(guard.check_second(PressSkill), time);
            self.skill_icd.update(time);
        } else if self.define_press {
            self.press.update(guard.check_second(PressSkill), time);
            self.skill_icd.update(time);
        }

        self.burst.update(guard.check_second(Burst), time);
        self.burst_icd.update(time);
    }

    fn should_hold(&self) -> bool {
        if self.define_hold {
            self.hold.is_cd_off()
        } else {
            false
        }
    }

    fn should_press(&self) -> bool {
        if self.define_press && self.define_hold {
            // TODO Because hold CD is longer than press CD, hold skill needs to be off to use press skill
            self.press.is_cd_off() && self.hold.is_cd_off()
        } else if self.define_press {
            self.press.is_cd_off()
        } else {
            false
        }
    }

    fn should_ca(&self) -> bool {
        if self.define_ca {
            self.ca.is_cd_off() && self.stamina.is_active()
        } else {
            false
        }
    }

    fn should_na(&self) -> bool {
        if self.define_na {
            self.na.is_cd_off()
        } else {
            false
        }
    }

    pub fn na_timer(&self) -> &dyn EffectTimer {
        if self.define_na {
            &self.na
        } else {
            &self.noop
        }
    }

    pub fn ca_timer(&self) -> &dyn EffectTimer {
        if self.define_ca {
            &self.ca
        } else {
            &self.noop
        }
    }

    pub fn press_timer(&self) -> &dyn EffectTimer {
        if self.define_press {
            &self.press
        } else {
            &self.noop
        }
    }

    pub fn hold_timer(&self) -> &dyn EffectTimer {
        if self.define_hold {
            &self.hold
        } else {
            &self.noop
        }
    }

    pub fn burst_timer(&self) -> &dyn EffectTimer {
        &self.burst
    }

    // accumulate

    pub fn reset_cd(&mut self) -> () {
        if self.define_press {
            self.press.reset();
        }
        if self.define_hold {
            self.hold.reset();
        }
    }

    pub fn reduce_cd(&mut self, time: f32) -> () {
        if self.define_press {
            self.press.cd -= time;
            self.press.dcd -= time;
        }
        if self.define_hold {
            self.press.cd -= time;
            self.press.dcd -= time;
        }
    }
}

#[derive(Debug)]
pub struct CharacterTimersBuilder {
    na_timer: Option<LoopTimer>,
    ca_timer: Option<HitsTimer>,
    stamina: Option<StaminaTimer>,
    press_timer: Option<DotTimer>,
    hold_timer:  Option<DotTimer>,
    burst_timer: Option<DotTimer>,
}

impl CharacterTimersBuilder {
    pub fn new() -> Self {
        Self {
            na_timer: None,
            ca_timer: None,
            stamina: None,
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

    pub fn stamina(mut self, t: StaminaTimer) -> Self {
        self.stamina = Some(t);
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

    pub fn build(self) -> FullCharacterTimers {
        let CharacterTimersBuilder {
            na_timer,
            ca_timer,
            stamina,
            press_timer,
            hold_timer,
            burst_timer,
        } = self;
        let mut define_na = false;
        let mut define_ca = false;
        let mut define_press = false;
        let mut define_hold = false;
        let na = if let Some(x) = na_timer {
            define_na = true;
            x
        } else {
            LoopTimer::noop()
        };
        let (ca, stamina) = match (ca_timer, stamina) {
            (Some(ca), Some(stamina)) => {
                define_ca = true;
                (ca, stamina)
            },
            _ => (HitsTimer::noop(), StaminaTimer::noop()),
        };
        let press = if let Some(x) = press_timer {
            define_press = true;
            x
        } else {
            DotTimer::noop()
        };
        let hold = if let Some(x) = hold_timer {
            define_hold = true;
            x
        } else {
            DotTimer::noop()
        };
        FullCharacterTimers {
            noop: NoopTimer,
            define_na,
            na,
            na_icd: ICDTimer::new(),
            define_ca,
            stamina,
            ca,
            ca_icd: ICDTimer::new(),
            define_press,
            press,
            define_hold,
            hold,
            skill_icd: ICDTimer::new(),
            burst: if let Some(x) = burst_timer { x } else { DotTimer::noop() },
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
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        assert_eq!(timer.is_active(), true);
    }

    #[test]
    fn durationtimer_1() {
        let mut timer = durationtimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(false), 2.0);
        assert_eq!(timer.is_active(), false);
    }

    fn hitstimer() -> HitsTimer {
        HitsTimer::new(1.0, 2)
    }

    #[test]
    fn hitstimer_0() {
        let mut timer = hitstimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        assert_eq!(timer.is_active(), true);
    }

    #[test]
    fn hitstimer_1() {
        let mut timer = hitstimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(true), 0.3);
        assert_eq!(timer.is_active(), true);
    }

    #[test]
    fn hitstimer_2() {
        let mut timer = hitstimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(true), 0.3);
        timer.update(guard.second(true), 0.3);
        assert_eq!(timer.is_active(), false);
    }

    #[test]
    fn hitstimer_3() {
        let mut timer = HitsTimer::new(0.0, 1);
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.2);
        assert_eq!(timer.is_active(), true);
        timer.update(guard.second(false), 0.2);
        assert_eq!(timer.is_active(), false);
    }

    fn dottimer() -> DotTimer {
        DotTimer::new(3.0, 0.5, 2)
    }

    #[test]
    fn dottimer_0() {
        let mut timer = dottimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        assert_eq!(timer.is_active(), true);
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn dottimer_1() {
        let mut timer = dottimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(false), 0.2);
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn dottimer_2() {
        let mut timer = dottimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(false), 0.5);
        assert_eq!(timer.is_active(), true);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn dottimer_3() {
        let mut timer = dottimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(false), 0.5);
        timer.update(guard.second(false), 0.5);
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn dottimer_4() {
        let mut timer = dottimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.0);
        timer.update(guard.second(false), 0.5);
        timer.update(guard.second(false), 0.5);
        timer.update(guard.second(false), 0.5);
        assert_eq!(timer.is_active(), false);
        assert_eq!(timer.n, 0);
    }

    fn stacktimer() -> StackTimer {
        StackTimer::new(0.3, 3.0, 2)
    }

    #[test]
    fn stacktimer_0() {
        let mut timer = stacktimer();
        let mut guard = TimerGuard::first_ok();
        // too fast to get stacks
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        assert!(timer.is_active());
        assert_eq!(timer.n, 1);
    }

    #[test]
    fn stacktimer_1() {
        let mut timer = stacktimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.3);
        timer.update(guard.second(true), 0.3);
        assert!(timer.is_active());
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn stacktimer_2() {
        let mut timer = stacktimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.3);
        timer.update(guard.second(true), 0.3);
        timer.update(guard.second(true), 0.3);
        // cannot exceed the max level
        assert!(timer.is_active());
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn stacktimer_3() {
        let mut timer = stacktimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.3);
        timer.update(guard.second(true), 0.3);
        assert!(timer.is_active());
        assert_eq!(timer.n, 2);
        // expire
        timer.update(guard.second(false), 3.0);
        assert!(!timer.is_active());
        assert_eq!(timer.n, 0);
    }

    fn sigiltimer() -> SigilTimer {
        SigilTimer::new(0.1, 5.0, 3.0, 3)
    }

    #[test]
    fn sigiltimer_1() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // sigil enterd effect CD
        assert!(timer.is_active());
        assert_eq!(timer.n, 3);
    }

    #[test]
    fn sigiltimer_2() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // sigil expired effect duration
        timer.update(guard.second(true), 3.0);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn sigiltimer_2_() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // sigil expired effect duration
        timer.update(guard.second(false), 3.0);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn sigiltimer_3() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // cannot gain another sigil because it is in CD
        timer.update(guard.second(true), 4.0);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn sigiltimer_3_() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // cannot gain another sigil because it is in CD
        timer.update(guard.second(false), 4.0);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        assert_eq!(timer.n, 0);
    }

    #[test]
    fn sigiltimer_4() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // can gain another sigil
        // note the last stack only takes effect
        timer.update(guard.second(true), 5.0);
        timer.update(guard.second(true), 0.1);
        assert_eq!(timer.n, 1);
        timer.update(guard.second(true), 0.1);
        assert_eq!(timer.n, 2);
    }

    #[test]
    fn sigiltimer_4_() {
        let mut timer = sigiltimer();
        let mut guard = TimerGuard::first_ok();
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        timer.update(guard.second(true), 0.1);
        // can gain another sigil
        // note the last stack only takes effect
        timer.update(guard.second(false), 5.0);
        timer.update(guard.second(true), 0.1);
        assert_eq!(timer.n, 1);
        timer.update(guard.second(true), 0.1);
        assert_eq!(timer.n, 2);
    }

    // fn looptimer() -> LoopTimer {
    //     LoopTimer::new(3.0, 3)
    // }

    // #[test]
    // fn looptimer_1() {
    //     let mut timer = looptimer();
    //     let mut guard = TimerGuard::first_ok();
    //     timer.update(guard.second(true), 0.0);
    //     assert_eq!(timer.n, 1);
    // }

    // #[test]
    // fn looptimer_too_early_1() {
    //     let mut timer = looptimer();
    //     let mut guard = TimerGuard::first_ok();
    //     timer.update(guard.second(true), 0.0);
    //     timer.update(guard.second(false), 0.3);
    //     assert_eq!(timer.n, 1);
    // }

    // #[test]
    // fn looptimer_too_early_2() {
    //     let mut timer = looptimer();
    //     let mut guard = TimerGuard::first_ok();
    //     timer.update(guard.second(true), 0.0);
    //     timer.update(guard.second(true), 0.3);
    //     assert_eq!(timer.n, 1);
    // }

    // #[test]
    // fn looptimer_2() {
    //     let mut timer = looptimer();
    //     let mut guard = TimerGuard::first_ok();
    //     timer.update(guard.second(true), 0.0);
    //     timer.update(guard.second(true), 1.0);
    //     assert_eq!(timer.n, 1);
    // }

    // #[test]
    // fn looptimer_3() {
    //     let mut timer = looptimer();
    //     let mut guard = TimerGuard::first_ok();
    //     timer.update(guard.second(true), 0.0);
    //     timer.update(guard.second(true), 1.0);
    //     timer.update(guard.second(true), 0.0);
    //     assert_eq!(timer.n, 2);
    // }
}
