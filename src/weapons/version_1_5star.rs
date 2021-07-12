use crate::state::State;
use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, StackTimer};
use crate::testutil;

use AttackType::*;
use Vision::*;

// version 1.0

pub struct SkywardBlade {
    burst_timer: DurationTimer,
    na: HitsTimer,
}

impl SkywardBlade {
    pub fn new() -> Self {
        Self {
            burst_timer: DurationTimer::new(0.0, 12.0),
            na: HitsTimer::new(0.001, 1),
        }
    }
}

impl SpecialAbility for SkywardBlade {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Blade").type_("Sword").version(1.0)
            .base_atk(608.0)
            .cr(4.0).er(55.1)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let mut na_or_ca = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Na | Ca => na_or_ca = true,
                Burst   => burst = true,
                _ => (),
            }
        }
        self.burst_timer.update(gaurd.second(burst), time);
        self.na.update(gaurd.second(na_or_ca), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_timer.is_active() && self.na.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 20.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk_spd += 10.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.na.reset();
    }
}

pub struct AquilaFavonia {
    na: HitsTimer,
}

impl AquilaFavonia {
    pub fn new() -> Self {
        Self {
            na: HitsTimer::new(15.0, 1),
        }
    }
}

impl SpecialAbility for AquilaFavonia {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Aquila Favonia").type_("Sword").version(1.0)
            .base_atk(674.0)
            .atk(20.0)
            .dmg_phy(41.3)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.na.update(gaurd.second(attack.iter().any(|a| a.kind == Na || a.kind == Ca)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.na.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 200.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.na.reset();
    }
}

pub struct SkywardPride {
    burst_timer: DurationTimer,
    na: HitsTimer,
}

impl SkywardPride {
    pub fn new() -> Self {
        Self {
            burst_timer: DurationTimer::new(0.0, 20.0),
            na: HitsTimer::new(20.0, 8),
        }
    }
}

impl SpecialAbility for SkywardPride {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Pride").type_("Claymore").version(1.0)
            .base_atk(674.0)
            .er(36.8)
            .dmg_na(8.0).dmg_ca(8.0).dmg_skill(8.0).dmg_burst(8.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.na.update(gaurd.second(attack.iter().any(|a| a.kind == Na || a.kind == Ca)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, _fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_timer.is_active() && self.na.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 80.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.na.reset();
    }
}

pub struct WolfsGravestone;

impl SpecialAbility for WolfsGravestone {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Wolf's Gravestone").type_("Claymore").version(1.0)
            .base_atk(608.0)
            .atk(49.6 + 20.0)
    }

    // TODO Box::new(FixedStack::new(MovementActivator, 30.0, 12.0, Atk(40.0)))
}

pub struct SkywardSpine {
    timer: HitsTimer,
}

impl SkywardSpine {
    pub fn new() -> Self {
        Self {
            timer: HitsTimer::new(2.0, 1),
        }
    }
}

impl SpecialAbility for SkywardSpine {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Spine").type_("Polearm").version(1.0)
            .base_atk(674.0)
            .cr(8.0).er(36.8).atk_spd(12.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(testutil::chance() < 0.5 && attack.iter().any(|a| a.kind == Na || a.kind == Ca)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 40.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct PrimordialJadeWingedSpear {
    timer: StackTimer,
}

impl PrimordialJadeWingedSpear {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(0.3, 6.0, 7) }
    }
}

impl SpecialAbility for PrimordialJadeWingedSpear {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Winged-Spear").type_("Polearm").version(1.0)
            .base_atk(674.0)
            .cr(22.1)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a|
            match &a.kind {
                Na | Ca | Skill | SkillDot | Burst | BurstDot => a.owned(owner_fc),
                _ => false,
            }
        );
        self.timer.update(gaurd.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].atk += 3.2 * self.timer.n as f32;
            modifiable_state[owner_fc.idx.0].all_dmg += if self.timer.n == 7 { 12.0 } else { 0.0 };
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SkywardHarp {
    timer: HitsTimer,
}

impl SkywardHarp {
    pub fn new() -> Self {
        Self {
            timer: HitsTimer::new(4.0, 1),
        }
    }
}

impl SpecialAbility for SkywardHarp {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Harp").type_("Bow").version(1.0)
            .base_atk(674.0)
            .cr(22.1).cd(20.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a|
            match &a.kind {
                Na | Ca | Skill | SkillDot | Burst | BurstDot => a.owned(owner_fc),
                _ => false,
            }
        );
        self.timer.update(gaurd.second(testutil::chance() < 0.6 && should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 125.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct AmosBow;

impl SpecialAbility for AmosBow {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Amos' Bow").type_("Bow").version(1.0)
            .base_atk(608.0)
            .atk(49.6)
            .dmg_na(12.0 + 40.0).dmg_ca(12.0 + 40.0)
    }
}

pub struct SkywardAtlas {
    timer: DotTimer,
}

impl SkywardAtlas {
    pub fn new() -> Self {
        Self {
            timer: DotTimer::new(30.0, 2.0, 8),
        }
    }
}

impl SpecialAbility for SkywardAtlas {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Atlas").type_("Catalyst").version(1.0)
            .base_atk(674.0)
            .atk(33.1)
            .dmg_pyro(12.0).dmg_cryo(12.0).dmg_hydro(12.0).dmg_electro(12.0).dmg_anemo(12.0).dmg_geo(12.0).dmg_dendro(12.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(testutil::chance() < 0.5 && attack.iter().any(|a| a.is_na())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(Attack {
                kind: AdditionalAttack,
                element: Physical,
                multiplier: 160.0,
                particle: None,
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct LostPrayerToTheSacredWinds {
    timer: StackTimer,
}

impl LostPrayerToTheSacredWinds {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(4.0, 8.0, 4) }
    }
}

impl SpecialAbility for LostPrayerToTheSacredWinds {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lost Prayer to the Sacred Winds").type_("Catalyst").version(1.0)
            .base_atk(608.0)
            .cr(33.1)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, _attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(true), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].elemental_dmg += 8.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
