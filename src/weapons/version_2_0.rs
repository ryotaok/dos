use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, Vision, ElementalReaction};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer, DurationTimer, DotTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct MistsplitterReforged {
    seal_1: usize,
    seal_1_duration: f32,
    seal_2: usize,
    seal_2_duration: f32,
    seal_3: usize,
}

impl MistsplitterReforged {
    pub fn new() -> Self {
        Self {
            seal_1: 0,
            seal_1_duration: 0.0,
            seal_2: 0,
            seal_2_duration: 0.0,
            seal_3: 0,
        }
    }
}

impl WeaponAbility for MistsplitterReforged {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Mistsplitter Reforged").type_(Sword).version(2.0)
            .base_atk(674.0)
            .cd(44.1)
            .dmg_pyro(12.0).dmg_cryo(12.0).dmg_hydro(12.0).dmg_electro(12.0).dmg_anemo(12.0).dmg_geo(12.0).dmg_dendro(12.0)
    }
}

impl SpecialAbility for MistsplitterReforged {
    fn update(&mut self, _guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.seal_1_duration -= time;
        self.seal_2_duration -= time;
        if self.seal_1_duration <= 0.0 {
            self.seal_1 = 0;
        }
        if self.seal_2_duration <= 0.0 {
            self.seal_2 = 0;
        }
        let mut seal_1 = false;
        let mut seal_2 = false;
        unsafe {
            for &a in attack {
                match (*a.atk).kind {
                    Na    => seal_1 = a.element != Physical,
                    Burst => seal_2 = true,
                    _ => (),
                }
            }
        }
        if seal_1 {
            self.seal_1 = 1;
            self.seal_1_duration = 5.0;
        }
        if seal_2 {
            self.seal_2 = 1;
            self.seal_2_duration = 10.0;
        }
        if data.state.energy / data.state.energy_cost < 1.0 {
            self.seal_3 = 1;
        } else {
            self.seal_3 = 0;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let level = self.seal_1 + self.seal_2 + self.seal_3;
        match level {
            3 => modifiable_state[data.idx.0].elemental_dmg += 28.0,
            2 => modifiable_state[data.idx.0].elemental_dmg += 16.0,
            1 => modifiable_state[data.idx.0].elemental_dmg += 8.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.seal_1 = 0;
        self.seal_1_duration = 0.0;
        self.seal_2 = 0;
        self.seal_2_duration = 0.0;
        self.seal_3 = 0;
    }
}

pub struct ThunderingPulse {
    seal_1: usize,
    seal_1_duration: f32,
    seal_2: usize,
    seal_2_duration: f32,
    seal_3: usize,
}

impl ThunderingPulse {
    pub fn new() -> Self {
        Self {
            seal_1: 0,
            seal_1_duration: 0.0,
            seal_2: 0,
            seal_2_duration: 0.0,
            seal_3: 0,
        }
    }
}

impl WeaponAbility for ThunderingPulse {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Thundering Pulse").type_(Bow).version(2.0)
            .base_atk(608.0)
            .atk(20.0).cd(66.2)
    }
}

impl SpecialAbility for ThunderingPulse {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.seal_1_duration -= time;
        self.seal_2_duration -= time;
        if self.seal_1_duration <= 0.0 {
            self.seal_1 = 0;
        }
        if self.seal_2_duration <= 0.0 {
            self.seal_2 = 0;
        }
        let mut seal_1 = false;
        let mut seal_2 = false;
        match &guard.kind {
            Na    => seal_1 = true,
            PressSkill | HoldSkill => seal_2 = true,
            _ => (),
        }
        if seal_1 {
            self.seal_1 = 1;
            self.seal_1_duration = 5.0;
        }
        if seal_2 {
            self.seal_2 = 1;
            self.seal_2_duration = 10.0;
        }
        if data.state.energy / data.state.energy_cost < 1.0 {
            self.seal_3 = 1;
        } else {
            self.seal_3 = 0;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let level = self.seal_1 + self.seal_2 + self.seal_3;
        match level {
            3 => modifiable_state[data.idx.0].na_dmg += 40.0,
            2 => modifiable_state[data.idx.0].na_dmg += 24.0,
            1 => modifiable_state[data.idx.0].na_dmg += 12.0,
            _ => (),
        };
    }

    fn reset(&mut self) -> () {
        self.seal_1 = 0;
        self.seal_1_duration = 0.0;
        self.seal_2 = 0;
        self.seal_2_duration = 0.0;
        self.seal_3 = 0;
    }
}

pub struct AmenomaKageuchi {
    succession_seed: usize,
    energy: f32,
    skill_timer: HitsTimer,
    energy_timer: DotTimer,
}

impl AmenomaKageuchi {
    pub fn new() -> Self {
        Self {
            succession_seed: 0,
            energy: 0.0,
            skill_timer: HitsTimer::new(5.0, 1),
            energy_timer: DotTimer::new(0.0, 2.0, 3),
        }
    }
}

impl WeaponAbility for AmenomaKageuchi {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Amenoma Kageuchi").type_(Sword).version(2.0)
            .base_atk(454.0)
            .atk(55.1)
    }
}

impl SpecialAbility for AmenomaKageuchi {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == PressSkill || guard.kind == HoldSkill;
        self.skill_timer.update(guard.second(should_update), time);
        if self.skill_timer.is_active() {
            self.succession_seed += 1;
        }
        self.energy_timer.update(guard.check_second(Burst), time);
        if self.energy_timer.n() == 1 {
            self.energy = 12.0 * self.succession_seed as f32;
            self.succession_seed = 0;
        } else if self.energy_timer.n() == 3 {
            self.energy = 0.0;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.energy_timer.n() == 2 {
            modifiable_state[data.idx.0].energy += self.energy;
        }
    }

    fn reset(&mut self) -> () {
        self.succession_seed = 0;
        self.skill_timer.reset();
        self.energy_timer.reset();
    }
}

pub struct KatsuragikiriNagamasa {
    energy_timer: DotTimer,
}

impl KatsuragikiriNagamasa {
    pub fn new() -> Self {
        Self {
            energy_timer: DotTimer::new(10.0, 2.0, 3),
        }
    }
}

impl WeaponAbility for KatsuragikiriNagamasa {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Katsuragikiri Nagamasa").type_(Claymore).version(2.0)
            .base_atk(510.0)
            .er(45.9)
            .dmg_skill(12.0)
    }
}

impl SpecialAbility for KatsuragikiriNagamasa {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let mut should_update = false;
        unsafe {
            for a in attack {
                match &(*a.atk).kind {
                    PressSkill | HoldSkill | SkillDot => {
                        should_update = true;
                        break;
                    },
                    _ => (),
                }
            }
        }
        self.energy_timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match self.energy_timer.n() {
            1 => state.energy += 5.0 - 3.0,
            2 => state.energy += 5.0,
            3 => state.energy += 5.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.energy_timer.reset();
    }
}

pub struct KitainCrossSpear {
    energy_timer: DotTimer,
}

impl KitainCrossSpear {
    pub fn new() -> Self {
        Self {
            energy_timer: DotTimer::new(10.0, 2.0, 3),
        }
    }
}

impl WeaponAbility for KitainCrossSpear {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Kitain Cross Spear").type_(Polearm).version(2.0)
            .base_atk(565.0)
            .em(110.0)
            .dmg_skill(12.0)
    }
}

impl SpecialAbility for KitainCrossSpear {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let mut should_update = false;
        unsafe {
            for a in attack {
                match &(*a.atk).kind {
                    PressSkill | HoldSkill | SkillDot => {
                        should_update = true;
                        break;
                    },
                    _ => (),
                }
            }
        }
        self.energy_timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match self.energy_timer.n() {
            1 => state.energy += 5.0 - 3.0,
            2 => state.energy += 5.0,
            3 => state.energy += 5.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.energy_timer.reset();
    }
}

pub struct Hamayumi;

impl WeaponAbility for Hamayumi {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Hamayumi").type_(Bow).version(2.0)
            .base_atk(454.0)
            .atk(55.1)
            .dmg_na(32.0).dmg_ca(24.0)
    }
}

impl SpecialAbility for Hamayumi {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if data.can_burst()  {
            let state = &mut modifiable_state[data.idx.0];
            state.na_dmg += 32.0;
            state.ca_dmg += 24.0;
        }
    }
}

pub struct HakushinRing {
    bonus_element: Vision,
    timer: DurationTimer,
}

impl HakushinRing {
    pub fn new() -> Self {
        Self {
            bonus_element: Physical,
            timer: DurationTimer::new(0.0, 6.0),
        }
    }
}

impl WeaponAbility for HakushinRing {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Hakushin Ring").type_(Catalyst).version(2.0)
            .base_atk(565.0)
            .er(30.6)
            .dmg_na(32.0).dmg_ca(24.0)
    }
}

impl SpecialAbility for HakushinRing {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        // TODO should include electro, anemo and geo?
        for a in attack {
            if a.element == Electro {
                let er = ElementalReaction::new(enemy.aura.aura, a.element);
                if er.is_electro() {
                    self.timer.update(guard.second(true), time);
                    self.bonus_element = enemy.aura.aura;
                    break;
                }
            }
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            match self.bonus_element {
                Pyro => for s in modifiable_state {
                    s.pyro_dmg += 20.0;
                },
                Hydro => for s in modifiable_state {
                    s.hydro_dmg += 20.0;
                },
                Cryo => for s in modifiable_state {
                    s.cryo_dmg += 20.0;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.bonus_element = Physical;
        self.timer.reset();
    }
}
