use crate::sim2::types::{CharacterAction, FieldEnergy};
use crate::sim2::record::CharacterData;

#[derive(Debug, Clone, Copy)]
pub struct ActionColumn<T> {
    pub burst: T,
    pub press: T,
    pub hold: T,
    pub na: T,
    pub ca: T,
}

impl Default for ActionColumn<f32> {
    fn default() -> Self {
        Self {
            burst: 0.0,
            press: 0.0,
            hold: 0.0,
            na: 0.0,
            ca: 0.0,
        }
    }
}

impl ActionColumn<f32> {
    pub fn add(&mut self, x: f32) -> () {
        self.burst += x;
        self.press += x;
        self.hold += x;
        self.na += x;
        self.ca += x;
    }

    pub fn copy(&mut self, other: &Self) -> () {
        self.burst = other.burst;
        self.press = other.press;
        self.hold = other.hold;
        self.na = other.na;
        self.ca = other.ca;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionState {
    pub current_time: f32,

    // absolute times of last events
    // these times are not affected by atk_spd, reduce_cd
    pub abs_time: ActionColumn<f32>,

    // relatively rel_time times of last events
    // these times may be affected by atk_spd, reduce_cd
    pub rel_time: ActionColumn<f32>,

    // timeline modifiers
    pub atk_spd: f32,
    pub reduce_skill: f32,
    pub energy: f32,
    pub er: f32,
}

// 1. decide an action
// 2. modify states
// 3. generate energy
// 4. update
impl ActionState {
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            abs_time: ActionColumn::<f32>::default(),
            rel_time: ActionColumn::<f32>::default(),
            atk_spd: 0.0,
            reduce_skill: 0.0,
            energy: 0.0,
            er: 0.0,
        }
    }

    pub fn update(&mut self, event: &CharacterAction, current_time: f32, elapsed_time: f32, energy: f32) -> () {
        self.current_time = current_time;
        match event {
            CharacterAction::StandStill => (),
            CharacterAction::Burst => {
                self.abs_time.burst = current_time;
                self.rel_time.burst = 0.0;
                self.energy = 0.0;
            },
            CharacterAction::PressSkill => {
                self.abs_time.press = current_time;
                self.rel_time.press = 0.0;
            },
            CharacterAction::HoldSkill => {
                self.abs_time.hold = current_time;
                self.rel_time.hold = 0.0;
            },
            CharacterAction::Na1 |
            CharacterAction::Na2 |
            CharacterAction::Na3 |
            CharacterAction::Na4 |
            CharacterAction::Na5 |
            CharacterAction::Na6 => {
                self.abs_time.na = current_time;
                self.rel_time.na = 0.0;
            },
            CharacterAction::Ca => {
                self.abs_time.ca = current_time;
                self.rel_time.ca = 0.0;
            },
        }
        self.rel_time.add(elapsed_time);
        self.energy += energy;
        if self.reduce_skill > 0.0 {
            self.rel_time.press += self.reduce_skill;
            self.rel_time.hold += self.reduce_skill;
        }
        if self.atk_spd > 0.0 {
            let t = elapsed_time * self.atk_spd / 100.0;
            self.rel_time.na += t;
            self.rel_time.ca += t;
        }
    }

    pub fn copy(&mut self, other: &Self) -> () {
        self.current_time = other.current_time;
        self.abs_time.copy(&other.abs_time);
        self.rel_time.copy(&other.rel_time);
        self.atk_spd = other.atk_spd;
        self.reduce_skill = other.reduce_skill;
        self.energy = other.energy;
        self.er = other.er;
    }

    pub fn init(&mut self, data: &CharacterData) -> () {
        self.atk_spd = data.weapon.atk_spd + data.artifact.atk_spd;
        self.reduce_skill = 0.0;
        self.er = data.character.er + data.weapon.er + data.artifact.er;
    }

    pub fn er(&self) -> f32 {
        1.0 + self.er / 100.0
    }

    pub fn did_burst(&self) -> bool {
        self.abs_time.burst == self.current_time
    }

    pub fn did_press(&self) -> bool {
        self.abs_time.press == self.current_time
    }

    pub fn did_hold(&self) -> bool {
        self.abs_time.hold == self.current_time
    }

    pub fn did_skill(&self) -> bool {
        self.abs_time.press == self.current_time || self.abs_time.hold == self.current_time
    }

    pub fn did_na(&self) -> bool {
        self.abs_time.na == self.current_time
    }

    pub fn did_ca(&self) -> bool {
        self.abs_time.ca == self.current_time
    }
}

// can be used to implement characters, weapons and artifacts
pub trait Timeline {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction { CharacterAction::StandStill }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {}
}
