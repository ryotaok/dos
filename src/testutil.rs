use std::mem;
// use std::cell::{RefCell};
// use rand::prelude::*;

use crate::state::State;
use crate::artifact::Artifact;
use crate::fc;
use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, Enemy, FieldCharacter, FieldAction, SpecialAbility, FieldCharacterData, FieldAbility};
use crate::types::{Vision, ElementalGaugeDecay};
use crate::action::{Attack, TimerGuard, NormalAttackAction, SkillAction, BurstAction};

pub fn revert<T>(mut xs: Vec<T>, a: usize, b: usize, c: usize) -> (Vec<T>, Vec<T>, Vec<T>) {
    if a + b + c > xs.len() {
        panic!("cannot revert");
    }
    let mut av: Vec<T> = Vec::with_capacity(a);
    let mut bv: Vec<T> = Vec::with_capacity(b);
    let mut cv: Vec<T> = Vec::with_capacity(c);
    match (a, b, c) {
        (0, 0, 0) => (av, bv, cv),
        (_, 0, 0) => { for x in xs.drain(..) { av.push(x); } (av, bv, cv ) },
        (0, _, 0) => { for x in xs.drain(..) { bv.push(x); } (av, bv, cv ) },
        (0, 0, _) => { for x in xs.drain(..) { cv.push(x); } (av, bv, cv ) },
        (0, s, t) => {
            for x in xs.drain(..s) { bv.push(x); }
            for x in xs.drain(..t) { cv.push(x); }
            (av, bv, cv)
        }
        (s, 0, t) => {
            for x in xs.drain(..s) { av.push(x); }
            for x in xs.drain(..t) { cv.push(x); }
            (av, bv, cv)
        }
        (s, t, 0) => {
            for x in xs.drain(..s) { av.push(x); }
            for x in xs.drain(..t) { bv.push(x); }
            (av, bv, cv)
        }
        _ => {
            for x in xs.drain(..a) { av.push(x); }
            for x in xs.drain(..b) { bv.push(x); }
            for x in xs.drain(..c) { cv.push(x); }
            (av, bv, cv)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revert_1() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 1, 2, 2);
        assert_eq!(av, [1]);
        assert_eq!(bv, [2,3]);
        assert_eq!(cv, [4,5]);
    }

    #[test]
    fn revert_2() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 0, 3, 2);
        assert_eq!(av, []);
        assert_eq!(bv, [1,2,3]);
        assert_eq!(cv, [4,5]);
    }

    #[test]
    fn revert_3() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 0, 5, 0);
        assert_eq!(av, []);
        assert_eq!(bv, [1,2,3,4,5]);
        assert_eq!(cv, []);
    }

    #[test]
    fn revert_4() {
        let v = vec![1,2,3,4,5];
        let (av, bv, cv) = revert(v, 0, 0, 0);
        assert_eq!(av, []);
        assert_eq!(bv, []);
        assert_eq!(cv, []);
    }
}

#[derive(Debug)]
pub struct Srdst<T> {
    pub source: Vec<T>,
    pub destination: Vec<T>,
}

impl<T> Srdst<T> {
    pub fn new(source: Vec<T>) -> Self {
        let destination: Vec<T> = Vec::with_capacity(source.capacity());
        Self { source, destination }
    }

    pub fn swap(&mut self) -> () {
        mem::swap(&mut self.source, &mut self.destination);
    }
}


// #[cfg(test)]
// #[cfg(not(test))]
pub fn chance() -> f32 {
    // rand::random::<f32>()
    0.0
}

pub struct TestCharacter {
    pub vision: String,
}

impl TestCharacter {
    pub fn new() -> Self {
        Self { vision: String::from("Pyro") }
    }
}

impl SpecialAbility for TestCharacter {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name(&String::from("simple")).vision(&self.vision).weapon(&String::from("Sword"))
            .base_hp(100.0).base_atk(100.0).base_def(100.0)
            .cr(0.0).cd(0.0)
            .na_1(100.0).na_2(100.0).na_3(100.0).na_4(100.0).na_5(100.0).na_6(0.0).na_time(3.0)
            .press_cd(6.0).press_particle(2.0).press_dmg(200.0)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(300.0)
    }
}

pub struct TestWeapon;

impl SpecialAbility for TestWeapon {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord {
            name: String::from("simple"), type_: String::from("Sword"), version: 1.0,
            base_atk: 0.0,
            atk: 0.0, hp: 0.0, def: 0.0, cr: 0.0, cd: 0.0, er: 0.0, em: 0.0, atk_spd: 0.0,
            dmg_na: 0.0, dmg_ca: 0.0, dmg_skill: 0.0, dmg_burst: 0.0,
            dmg_phy: 0.0, dmg_pyro: 0.0, dmg_cryo: 0.0, dmg_hydro: 0.0, dmg_electro: 0.0, dmg_anemo: 0.0, dmg_geo: 0.0, dmg_dendro: 0.0,
        }
    }
}

pub struct TestArtifact(pub State);

impl SpecialAbility for TestArtifact {
    fn artifact(&self) -> Artifact {
        let mut state = State::new();
        state.merge(&self.0);
        Artifact {
            name: String::from("simple"),
            version: 1.0,
            preference: Vec::new(),
            state
        }
    }
}

pub struct TestAbility<T: SpecialAbility>(pub T);

impl<T: SpecialAbility> SpecialAbility for TestAbility<T> {
    fn character(&self) -> CharacterRecord { TestCharacter::new().character() }
    fn weapon(&self) -> WeaponRecord { TestWeapon.weapon() }
    fn artifact(&self) -> Artifact { Default::default() }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, owner_fc, fa, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }

    fn accelerate(&self, na: &mut NormalAttackAction, skill: &mut SkillAction, burst: &mut BurstAction) -> () {
        self.0.accelerate(na, skill, burst);
    }

    fn intensify(&self, attack: &mut Attack, owner_fc: &FieldCharacter, enemy: &Enemy) -> () {
        self.0.intensify(attack, owner_fc, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct TestEnvironment;

impl TestEnvironment {
    pub fn enemy() -> Enemy {
        Enemy::simple()
    }

    pub fn fc(state: State) -> FieldCharacterData {
        TestEnvironment::fc_n(FieldCharacterIndex(0), state)
    }

    pub fn fc1(state: State) -> FieldCharacterData {
        TestEnvironment::fc_n(FieldCharacterIndex(1), state)
    }

    pub fn fc_n(idx: FieldCharacterIndex, state: State) -> FieldCharacterData {
        let ca = TestCharacter { vision: String::from("Pyro") };
        let cr = ca.character();
        let vision = Vision::from(&cr.vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        FieldCharacter::new(idx, cr, vision, wa.weapon(), aa.artifact()).to_data(FieldAbility {
            character: Box::new(ca),
            weapon: Box::new(wa),
            artifact: Box::new(aa),
        })
    }

    pub fn fc_artifact<T: 'static +  SpecialAbility>(idx: FieldCharacterIndex, aa: T) -> FieldCharacterData {
        let ca = TestCharacter { vision: String::from("Pyro") };
        let cr = ca.character();
        let vision = Vision::from(&cr.vision);
        let wa = TestWeapon;
        FieldCharacter::new(idx, cr, vision, wa.weapon(), aa.artifact()).to_data(FieldAbility {
            character: Box::new(ca),
            weapon: Box::new(wa),
            artifact: Box::new(aa),
        })
    }

    pub fn vision(state: State, vision: &str) -> FieldCharacterData {
        let ca = TestCharacter { vision: String::from(vision) };
        let cr = ca.character();
        let vision = Vision::from(&cr.vision);
        let wa = TestWeapon;
        let aa = TestArtifact(state);
        FieldCharacter::new(FieldCharacterIndex(0), cr, vision, wa.weapon(), aa.artifact()).to_data(FieldAbility {
            character: Box::new(ca),
            weapon: Box::new(wa),
            artifact: Box::new(aa),
        })
    }

    pub fn no_skill(character: Box<dyn SpecialAbility>, weapon: Box<dyn SpecialAbility>, artifact: Box<dyn SpecialAbility>) -> FieldCharacterData {
        let idx = FieldCharacterIndex(0);
        let cr = character.character();
        let wr = weapon.weapon();
        let ar = artifact.artifact();
        let vision = Vision::from(&cr.vision);
        let burst = cr.burst_action();
        let skill = SkillAction::noop(vision);
        let normal_action = cr.normal_action();
        let mut state = State::new();
        state.merge(&cr.state());
        state.merge(&wr.state());
        state.merge(&ar.state);
        (
            FieldCharacter { idx, cr, wr, ar, state, vision },
            FieldAbility { character, weapon, artifact },
            Vec::new(),
            FieldAction { burst, skill, na: normal_action },
        )
    }
}
