use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.0

pub struct Chongyun {
    skill_infusion: DurationTimer,
    skill_timer: DurationTimer,
    skill_expire: bool, // a4
}

impl Chongyun {
    pub fn new() -> Self {
        Self {
            skill_infusion: DurationTimer::new(15.0, 3.0),
            skill_timer: DurationTimer::new(15.0, 10.0),
            skill_expire: false,
        }
    }
}

impl SpecialAbility for Chongyun {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Chongyun").vision("Cryo").weapon("Claymore").release_date("2020-09-28").version(1.0)
            .base_hp(10984.0).base_atk(223.0).base_def(648.0)
            .atk(24.0)
            .na_1(138.38).na_2(124.78).na_3(158.78).na_4(200.09).na_time(2.983)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(1.9)
            .press_cd(15.0).press_particle(4.0).press_dmg(309.67)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(256.32 * 3.0)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.skill_infusion.update(gaurd.second(attack.iter().any(|a| a.kind == Skill)), time);
        let before = self.skill_timer.is_active();
        self.skill_timer.update(gaurd, time);
        let after = self.skill_timer.is_active();
        self.skill_expire = before && !after;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_expire {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Cryo,
                multiplier: 309.67,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        // a1
        if self.skill_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.atk_spd += 8.0; // TODO only melee characters
            }
        }
        if self.skill_infusion.is_active() {
            for s in modifiable_state.iter_mut() {
                s.infusion = true;
            }
        }
        if self.skill_expire {
            enemy.element_res_debuff.push(Debuff::chongyun_a4());
        }
    }

    fn reset(&mut self) -> () {
        self.skill_infusion.reset();
        self.skill_timer.reset();
        self.skill_expire = false;
    }
}

pub struct Kaeya {
    burst_aa: DotTimer,
    skill_a4: bool,
}

impl Kaeya {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(15.0, 0.66666, 12),
            skill_a4: false
        }
    }
}

impl SpecialAbility for Kaeya {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Kaeya").vision("Cryo").weapon("Sword").release_date("2020-09-28").version(1.0)
            .base_hp(11636.0).base_atk(223.0).base_def(792.0)
            .er(26.7)
            .na_1(106.25).na_2(102.17).na_3(129.03).na_4(140.08).na_5(174.42).na_time(2.734)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(6.0).press_particle(2.5).press_dmg(344.16)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(0.0)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.skill_a4 = attack.iter().any(|a| a.kind == Skill && a.element == Cryo) && enemy.aura.aura == Hydro;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Cryo,
                multiplier: 139.92,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
        if self.skill_a4 {
            atk_queue.push(Attack {
                kind: StandStill,
                element: Cryo,
                multiplier: 0.0,
                particle: Some(2.0),
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.skill_a4 = false;
    }
}

pub struct Qiqi {
    skill_aa: DotTimer,
}

impl Qiqi {
    pub fn new() -> Self {
        Self {
            skill_aa: DotTimer::new(30.0, 3.0, 4),
        }
    }
}

impl SpecialAbility for Qiqi {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Qiqi").vision("Cryo").weapon("Sword").release_date("2020-09-28").version(1.0)
            .base_hp(12368.0).base_atk(287.0).base_def(922.0)
            .na_1(74.63).na_2(76.84).na_3(47.77*2.0).na_4(48.79*2.0).na_5(124.61).na_time(2.25)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(30.0).press_particle(0.0).press_dmg(172.8)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(512.64)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.skill_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Skill)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Pyro,
                multiplier: 64.8*2.0,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.skill_aa.reset();
    }
}
