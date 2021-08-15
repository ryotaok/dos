use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, Vision, ElementalReaction};
use crate::fc::{SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, NTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct MistsplitterReforged {
    seal_1: DurationTimer,
    seal_2: DurationTimer,
    previous_seal: usize,
    seal: usize,
}

impl MistsplitterReforged {
    pub fn new() -> Self {
        Self {
            seal_1: DurationTimer::new(5.0, &[0.0]),
            seal_2: DurationTimer::new(10.0, &[0.0]),
            previous_seal: 0,
            seal: 0,
        }
    }
}

impl MistsplitterReforged {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mistsplitter Reforged").type_(Sword).version(2.0)
            .base_atk(674.0)
            .cd(44.1)
            .pyro_dmg(12.0).cryo_dmg(12.0).hydro_dmg(12.0).electro_dmg(12.0).anemo_dmg(12.0).geo_dmg(12.0).dendro_dmg(12.0)
    }
}

impl SpecialAbility for MistsplitterReforged {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let mut seal_1 = false;
        let mut seal_2 = false;
        let mut seal_3 = 0;
        unsafe {
            for &a in attack {
                let atk = & *a;
                match (atk.idx == data.idx, &atk.kind) {
                    (true, Na)    => seal_1 = atk.element.aura != Physical,
                    (true, Burst) => seal_2 = true,
                    _ => (),
                }
            }
        }
        self.seal_1.update(time, seal_1);
        self.seal_2.update(time, seal_2);
        if data.state().energy / data.character.energy_cost < 1.0 {
            seal_3 = 1;
        }
        self.previous_seal = self.seal;
        self.seal = self.seal_1.n + self.seal_2.n + seal_3;
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.seal > self.previous_seal {
            match (self.seal, self.previous_seal) {
                (3, 0) => state.elemental_dmg += 28.0,
                (3, 1) => state.elemental_dmg += 20.0,
                (3, 2) => state.elemental_dmg += 12.0,
                (2, 0) => state.elemental_dmg += 16.0,
                (2, 1) => state.elemental_dmg += 8.0,
                (1, 0) => state.elemental_dmg += 8.0,
                _ => (),
            }
        } else if self.seal < self.previous_seal {
            match (self.seal, self.previous_seal) {
                (0, 3) => state.elemental_dmg -= 28.0,
                (1, 3) => state.elemental_dmg -= 20.0,
                (2, 3) => state.elemental_dmg -= 12.0,
                (0, 2) => state.elemental_dmg -= 16.0,
                (1, 2) => state.elemental_dmg -= 8.0,
                (0, 1) => state.elemental_dmg -= 8.0,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.seal_1.reset();
        self.seal_2.reset();
        self.previous_seal = 0;
        self.seal = 0;
    }
}

pub struct ThunderingPulse {
    seal_1: DurationTimer,
    seal_2: DurationTimer,
    previous_seal: usize,
    seal: usize,
}

impl ThunderingPulse {
    pub fn new() -> Self {
        Self {
            seal_1: DurationTimer::new(5.0, &[0.0]),
            seal_2: DurationTimer::new(10.0, &[0.0]),
            previous_seal: 0,
            seal: 0,
        }
    }
}

impl ThunderingPulse {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Thundering Pulse").type_(Bow).version(2.0)
            .base_atk(608.0)
            .atk(20.0).cd(66.2)
    }
}

impl SpecialAbility for ThunderingPulse {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let mut seal_1 = false;
        let mut seal_2 = false;
        let mut seal_3 = 0;
        unsafe {
            for &a in attack {
                let atk = & *a;
                match (atk.idx == data.idx, &atk.kind) {
                    (true, Na)    => seal_1 = true,
                    (true, PressSkill) |
                    (true, HoldSkill) => seal_2 = true,
                    _ => (),
                }
            }
        }
        self.seal_1.update(time, seal_1);
        self.seal_2.update(time, seal_2);
        if data.state().energy / data.character.energy_cost < 1.0 {
            seal_3 = 1;
        }
        self.previous_seal = self.seal;
        self.seal = self.seal_1.n + self.seal_2.n + seal_3;
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.seal > self.previous_seal {
            match (self.seal, self.previous_seal) {
                (3, 0) => state.na_dmg += 40.0,
                (3, 1) => state.na_dmg += 28.0,
                (3, 2) => state.na_dmg += 16.0,
                (2, 0) => state.na_dmg += 24.0,
                (2, 1) => state.na_dmg += 12.0,
                (1, 0) => state.na_dmg += 12.0,
                _ => (),
            }
        } else if self.seal < self.previous_seal {
            match (self.seal, self.previous_seal) {
                (0, 3) => state.na_dmg -= 40.0,
                (1, 3) => state.na_dmg -= 28.0,
                (2, 3) => state.na_dmg -= 16.0,
                (0, 2) => state.na_dmg -= 24.0,
                (1, 2) => state.na_dmg -= 12.0,
                (0, 1) => state.na_dmg -= 12.0,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.seal_1.reset();
        self.seal_2.reset();
        self.previous_seal = 0;
        self.seal = 0;
    }
}

pub struct AmenomaKageuchi {
    skill_timer: DurationTimer,
    energy_timer: NTimer,
}

impl AmenomaKageuchi {
    pub fn new() -> Self {
        Self {
            skill_timer: DurationTimer::new(30.0, &[5.0,5.0,5.0]),
            energy_timer: NTimer::new(&[2.0, 0.1]),
        }
    }
}

impl AmenomaKageuchi {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Amenoma Kageuchi").type_(Sword).version(2.0)
            .base_atk(454.0)
            .atk(55.1)
    }
}

impl SpecialAbility for AmenomaKageuchi {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if self.energy_timer.ping && self.energy_timer.n == 0 {
            self.skill_timer.reset();
        }
        let check_idx = event.idx == data.idx;
        self.skill_timer.update(time, check_idx && (event.kind == PressSkill || event.kind == HoldSkill));
        self.energy_timer.update(time, check_idx && event.kind == Burst);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.energy_timer.ping && self.energy_timer.n == 2 {
            let state = &mut modifiable_state[data.idx.0];
            state.energy += state.ER() * 12.0 * self.skill_timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        self.energy_timer.reset();
    }
}

pub struct KatsuragikiriNagamasa {
    timer: NTimer,
}

impl KatsuragikiriNagamasa {
    pub fn new() -> Self {
        Self {
            timer: NTimer::new(&[2.0,2.0,2.0, 4.0]),
        }
    }
}

impl KatsuragikiriNagamasa {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Katsuragikiri Nagamasa").type_(Claymore).version(2.0)
            .base_atk(510.0)
            .er(45.9)
            .skill_dmg(12.0)
    }
}

impl SpecialAbility for KatsuragikiriNagamasa {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let mut should_update = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && (atk.kind == PressSkill || atk.kind == HoldSkill || atk.kind == SkillDot) {
                    should_update = true;
                    break;
                }
            }
        }
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.ping {
            let state = &mut modifiable_state[data.idx.0];
            match self.timer.n {
                1 => state.energy += state.ER() * (5.0 - 3.0),
                2 |
                3 => state.energy += state.ER() * 5.0,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct KitainCrossSpear {
    timer: NTimer,
}

impl KitainCrossSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Kitain Cross Spear").type_(Polearm).version(2.0)
            .base_atk(565.0)
            .em(110.0)
            .skill_dmg(12.0)
    }

    pub fn new() -> Self {
        Self {
            timer: NTimer::new(&[2.0,2.0,2.0, 4.0]),
        }
    }
}

impl SpecialAbility for KitainCrossSpear {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let mut should_update = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && (atk.kind == PressSkill || atk.kind == HoldSkill || atk.kind == SkillDot) {
                    should_update = true;
                    break;
                }
            }
        }
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.ping {
            let state = &mut modifiable_state[data.idx.0];
            match self.timer.n {
                1 => state.energy += state.ER() * (5.0 - 3.0),
                2 |
                3 => state.energy += state.ER() * 5.0,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct Hamayumi {
    ping: bool,
    condition: bool,
}

impl Hamayumi {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Hamayumi").type_(Bow).version(2.0)
            .base_atk(454.0)
            .atk(55.1)
            .na_dmg(32.0).ca_dmg(24.0)
    }

    pub fn new() -> Self {
        Self {
            ping: false,
            condition: false,
        }
    }
}

impl SpecialAbility for Hamayumi {
    fn update(&mut self, _time: f32, _event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        match (self.condition, data.can_burst()) {
            (true, false) => {
                self.ping = true;
                self.condition = false;
            },
            (false, true) => {
                self.ping = true;
                self.condition = true;
            },
            _ => self.ping = false,
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        match (self.ping, self.condition) {
            (true, true) => {
                let state = &mut modifiable_state[data.idx.0];
                state.na_dmg += 32.0;
                state.ca_dmg += 24.0;
            },
            (true, false) => {
                let state = &mut modifiable_state[data.idx.0];
                state.na_dmg -= 32.0;
                state.ca_dmg -= 24.0;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.ping = false;
        self.condition = false;
    }
}

pub struct HakushinRing {
    timer: DurationTimer,
}

impl HakushinRing {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(6.0, &[0.0]),
        }
    }
}

impl HakushinRing {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Hakushin Ring").type_(Catalyst).version(2.0)
            .base_atk(565.0)
            .er(30.6)
    }
}

impl SpecialAbility for HakushinRing {
    fn update(&mut self, time: f32, _event: &AttackEvent, _data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        // TODO should include electro, anemo and geo?
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.element.aura == Electro {
                    let er = ElementalReaction::new(enemy.aura.aura, atk.element.aura);
                    if er.is_electro() {
                        self.timer.update(time, true);
                        break;
                    }
                }
            }
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _data: &CharacterData, _enemy: &mut Enemy) -> () {
        // TODO
        // match (self.timer.ping, self.timer.n) {
        //     (true, 1) => match self.bonus_element {
        //         Pyro => for s in modifiable_state {
        //             s.pyro_dmg += 20.0;
        //         },
        //         Hydro => for s in modifiable_state {
        //             s.hydro_dmg += 20.0;
        //         },
        //         Cryo => for s in modifiable_state {
        //             s.cryo_dmg += 20.0;
        //         },
        //         _ => (),
        //     },
        //     None => expr,
        // }
        match (self.timer.ping, self.timer.n) {
            (true, 1) => for s in modifiable_state {
                s.pyro_dmg += 20.0;
                s.hydro_dmg += 20.0;
                s.elemental_dmg += 20.0;
                s.cryo_dmg += 20.0;
            },
            (true, 0) => for s in modifiable_state {
                s.pyro_dmg -= 20.0;
                s.hydro_dmg -= 20.0;
                s.elemental_dmg -= 20.0;
                s.cryo_dmg -= 20.0;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
