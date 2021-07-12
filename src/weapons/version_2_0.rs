use crate::state::State;
use crate::fc::{SpecialAbility, FieldCharacter, WeaponRecord, Enemy};
use crate::action::{Attack, TimerGuard};
use crate::types::{AttackType, Vision};

use AttackType::*;
use Vision::*;

pub struct MistsplittersReforged {
    seal_1: usize,
    seal_1_duration: f32,
    seal_2: usize,
    seal_2_duration: f32,
    seal_3: usize,
}

impl MistsplittersReforged {
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

impl SpecialAbility for MistsplittersReforged {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Mistsplitter's Reforged").type_("Sword").version(2.0)
            .base_atk(674.0)
            .cd(44.1)
            .dmg_pyro(12.0).dmg_cryo(12.0).dmg_hydro(12.0).dmg_electro(12.0).dmg_anemo(12.0).dmg_geo(12.0).dmg_dendro(12.0)
    }

    fn update(&mut self, _gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
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
        for a in attack {
            match a.kind {
                Na    => seal_1 = a.element != Physical,
                Burst => seal_2 = true,
                _ => (),
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
        if owner_fc.state.energy.0 / owner_fc.state.energy_cost < 1.0 {
            self.seal_3 = 1;
        } else {
            self.seal_3 = 0;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        let level = self.seal_1 + self.seal_2 + self.seal_3;
        match level {
            3 => modifiable_state[owner_fc.idx.0].elemental_dmg += 28.0,
            2 => modifiable_state[owner_fc.idx.0].elemental_dmg += 16.0,
            1 => modifiable_state[owner_fc.idx.0].elemental_dmg += 8.0,
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

impl SpecialAbility for ThunderingPulse {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Thundering Pulse").type_("Bow").version(2.0)
            .base_atk(608.0)
            .atk(20.0).cd(66.2)
    }

    fn update(&mut self, _gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
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
        for a in attack {
            match a.kind {
                Na    => seal_1 = true,
                Skill => seal_2 = true,
                _ => (),
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
        if owner_fc.state.energy.0 / owner_fc.state.energy_cost < 1.0 {
            self.seal_3 = 1;
        } else {
            self.seal_3 = 0;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        let level = self.seal_1 + self.seal_2 + self.seal_3;
        match level {
            3 => modifiable_state[owner_fc.idx.0].na_dmg += 40.0,
            2 => modifiable_state[owner_fc.idx.0].na_dmg += 24.0,
            1 => modifiable_state[owner_fc.idx.0].na_dmg += 12.0,
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
