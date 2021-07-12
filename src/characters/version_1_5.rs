use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer, SigilTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

pub struct Yanfei {
    burst_duration: DurationTimer,
    burst_grant_interval: DotTimer,
    scarlet_seal: SigilTimer,
}

impl Yanfei {
    pub fn new() -> Self {
        Self {
            burst_duration: DurationTimer::new(20.0, 15.0),
            burst_grant_interval: DotTimer::new(20.0, 1.0, 15), // scarlet seal grant interval
            scarlet_seal: SigilTimer::new(0.0, 1.5, 0.0, 3), // TODO inaccurate
        }
    }
}

impl SpecialAbility for Yanfei {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Yanfei").vision("Pyro").weapon("Catalyst").release_date("2020-12-23").version(1.5)
            .base_hp(9352.0).base_atk(240.0).base_def(587.0)
            .dmg_pyro(24.0)
            .na_1(105.01).na_2(93.83).na_3(136.82).na_4(0.0).na_5(0.0).na_6(0.0).na_time(1.34)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(9.0).press_particle(3.0).press_dmg(305.28)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(328.32)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        for a in attack {
            match &a.kind {
                Na    => self.scarlet_seal.update(gaurd.second(true), time),
                Skill => {
                    // gain 3 scarlet seals on skill
                    for _ in 0..3 {
                        if self.scarlet_seal.n != 3 {
                            self.scarlet_seal.update(gaurd.second(true), 0.0);
                        }
                    }
                },
                _ => self.scarlet_seal.update(gaurd.second(false), time),
            };
        }
        self.burst_duration.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.burst_grant_interval.update(gaurd, time);
        if self.burst_grant_interval.is_active() && self.scarlet_seal.n != 3 {
            self.scarlet_seal.update(gaurd.second(true), 0.0);
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.scarlet_seal.is_active() {
            atk_queue.push(Attack {
                kind: Ca,
                element: Pyro,
                multiplier: 272.92 + 64.0,// TODO a4
                particle: None,
                state: None,
                icd_cleared: fa.na.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            });
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.burst_duration.is_active() {
            modifiable_state[owner_fc.idx.0].ca_dmg += 54.4;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_duration.reset();
        self.burst_grant_interval.reset();
        self.scarlet_seal.reset();
    }
}

pub struct Eula {
    grimheart: SigilTimer,
    lightfall_sword_stack: f32,
    lightfall_sword_timer: DurationTimer,
    lightfall_sword_expire: bool,
}

impl Eula {
    pub fn new() -> Self {
        Self {
            // the 3rd skill hit will automatically trigger the hold skill
            grimheart: SigilTimer::new(0.0, 2.0, 0.0, 3),
            lightfall_sword_stack: 0.0,
            lightfall_sword_timer: DurationTimer::new(20.0, 7.0),
            lightfall_sword_expire: false,
        }
    }
}

impl SpecialAbility for Eula {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Eula").vision("Cryo").weapon("Claymore").release_date("2021-01-12").version(1.5)
            .base_hp(13226.0).base_atk(342.0).base_def(751.0)
            .cd(88.4)
            .na_1(117.38).na_2(184.93).na_3(112.28*2.0).na_4(222.67).na_5(142.0*2.0).na_time(3.85)
            // .na_0(83.65).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(4.0).press_particle(1.5).press_dmg(263.52)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(617.44)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        // update lightfall_sword_timer
        let before = self.lightfall_sword_timer.is_active();
        self.lightfall_sword_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        let after  = self.lightfall_sword_timer.is_active();
        self.lightfall_sword_expire = before && !after;

        // accumulate stacks
        if self.lightfall_sword_timer.is_active() {
            for a in attack {
                match &a.kind {
                    Na | Ca | Skill | SkillDot | Burst => self.lightfall_sword_stack += 1.0,
                    _ => (),
                };
            }
        // do not clear the stacks on expire
        } else if !self.lightfall_sword_timer.is_active() && !self.lightfall_sword_expire {
            self.lightfall_sword_stack = 0.0;
        }

        // a4
        self.grimheart.update(gaurd.second(attack.iter().any(|a| a.kind == Skill || a.kind == Burst)), time);
        // assume consuming two stacks of `grimheart` and a1
        if self.grimheart.is_active() {
            self.lightfall_sword_stack += 3.0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.lightfall_sword_expire {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Physical,
                multiplier: 725.56 + 148.24 * self.lightfall_sword_stack,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            });
        }
        if self.grimheart.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Cryo,
                multiplier: 172.8 * 2.0,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            });
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Physical,
                multiplier: 725.56 * 0.5, // a1
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            });
        }
    }

    fn modify(&self, _modifiable_state: &mut [State], _owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        if self.grimheart.is_active() {
            enemy.element_res_debuff.push(Debuff::eula_cryo());
            enemy.physical_res_debuff.push(Debuff::eula_physical());
        }
    }

    fn reset(&mut self) -> () {
        self.grimheart.reset();
        self.lightfall_sword_stack = 0.0;
        self.lightfall_sword_timer.reset();
        self.lightfall_sword_expire = false;
    }
}
