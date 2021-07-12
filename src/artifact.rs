use crate::state::State;
use crate::fc::{FieldCharacter, SpecialAbility, Enemy, Debuff};
use crate::action::{Attack, TimerGuard, TimerGuardCheck, EffectTimer, StackTimer, DurationTimer};
use crate::types::{AttackType, UnstackableBuff, Preference, Vision};

use AttackType::*;

pub type ArtifactRow = (Artifact, Box<dyn SpecialAbility>);

#[derive(Debug)]
pub struct Artifact {
    pub name: String,
    pub version: f32,
    pub preference: Vec<Preference>,
    pub state: State,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            name: String::from(""),
            version: 1.0,
            preference: Vec::new(),
            state: State::new()
        }
    }
}

impl Artifact {
    pub fn infuse_goblet(&mut self, vision: &Vision) -> &mut Self {
        match &vision {
            Vision::Pyro => self.state.pyro_dmg = 46.6,
            Vision::Cryo => self.state.cryo_dmg = 46.6,
            Vision::Hydro => self.state.hydro_dmg = 46.6,
            Vision::Electro => self.state.electro_dmg = 46.6,
            Vision::Anemo => self.state.anemo_dmg = 46.6,
            Vision::Geo => self.state.geo_dmg = 46.6,
            Vision::Dendro => self.state.dendro_dmg = 46.6,
            Vision::Physical => self.state.physical_dmg = 58.3,
        };
        self
    }

    pub fn all() -> Vec<Box<dyn SpecialAbility>> {
        vec![
            Box::new(BloodstainedChivalry),
            Box::new(TwoBcTwoPf),
            Box::new(ThunderingFury),
            Box::new(ViridescentVenerer),
            Box::new(VVem::new()),
            Box::new(ArchaicPetra),
            Box::new(CrimsonWitchOfFlames),
            Box::new(CrimsonWitchOfFlamesHp),
            Box::new(NoblesseOblige::new()),
            Box::new(TwoGfTwoNo),
            Box::new(GladiatorsFinale::new()),
            Box::new(GladiatorsFinaleDef::new()),
            Box::new(WanderersTroupe),
            Box::new(RetracingBolide),
            Box::new(RetracingBolideDef),
            Box::new(Thundersoother),
            Box::new(Lavawalker),
            Box::new(LavawalkerHp),
            Box::new(TwoGfTwoElemental),
            Box::new(BlizzardStrayer),
            Box::new(HeartOfDepth::new()),
            Box::new(GlacierAndSnowfield::new()),
            Box::new(PaleFlame::new()),
            Box::new(TenacityOfTheMillelith::new()),
            Box::new(ShimenawasReminiscence::new()),
            Box::new(TwoGfTwoShimenawa),
            Box::new(EmblemOfSeveredFate),
        ]
    }

    pub fn setup(version: f32) -> Vec<ArtifactRow> {
        let mut artifacts: Vec<(Artifact, Box<dyn SpecialAbility>)> = Vec::new();
        for ar in Artifact::all() {
            let mut r = ar.artifact();
            if r.version <= version {
                // default setup for all artifacts
                r.state.flat_atk += 311.0;
                r.state.atk += 80.0;
                r.state.cr  += 80.0;
                artifacts.push((r, ar));
            }
        }
        artifacts
    }
}


#[derive(Debug)]
pub struct BloodstainedChivalry;

impl SpecialAbility for BloodstainedChivalry {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Bloodstained Chivalry"),
            version: 1.0,
            preference: vec![Preference::Physical],
            state: State::new().physical_dmg(25.0)
        }
    }
}

#[derive(Debug)]
pub struct TwoBcTwoPf;

impl SpecialAbility for TwoBcTwoPf {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("2 bc 2 pf"),
            version: 1.0,
            preference: vec![Preference::Physical],
            state: State::new().physical_dmg(50.0)
        }
    }
}

#[derive(Debug)]
pub struct ThunderingFury;

impl SpecialAbility for ThunderingFury {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Thundering Fury"),
            version: 1.0,
            preference: vec![Preference::Electro],
            state: State::new().electro_dmg(15.0).transformative_bonus(40.0)
        }
    }
}

#[derive(Debug)]
pub struct ViridescentVenerer;

impl SpecialAbility for ViridescentVenerer {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Viridescent Venerer"),
            version: 1.0,
            preference: vec![Preference::Anemo],
            state: State::new().anemo_dmg(15.0).transformative_bonus(60.0)
        }
    }

    fn modify(&self, _modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        if owner_fc.vision == Vision::Anemo {
            match &enemy.aura.aura {
                Vision::Pyro |
                Vision::Hydro |
                Vision::Electro |
                Vision::Cryo => enemy.element_res_debuff.push(Debuff::viridescent_venerer()),
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct VVem(ViridescentVenerer);

impl VVem {
    pub fn new() -> Self {
        Self(ViridescentVenerer)
    }
}

impl SpecialAbility for VVem {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Viridescent Venerer (EM)"),
            version: 1.0,
            preference: vec![Preference::Anemo],
            state: State::new().anemo_dmg(15.0).transformative_bonus(60.0).em(6.012 * (53.333+80.0)).atk(-80.0).cr(-80.0)
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }
}

#[derive(Debug)]
pub struct ArchaicPetra;

impl SpecialAbility for ArchaicPetra {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Archaic Petra"),
            version: 1.0,
            preference: vec![Preference::Geo],
            state: State::new().geo_dmg(15.0)
        }
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlames;

impl SpecialAbility for CrimsonWitchOfFlames {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Crimson Witch of Flames"),
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new().pyro_dmg(15.0+7.5).amplifying_bonus(15.0).transformative_bonus(40.0)
        }
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlamesHp;

impl SpecialAbility for CrimsonWitchOfFlamesHp {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Crimson Witch of Flames (HP)"),
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new().pyro_dmg(15.0+7.5).amplifying_bonus(15.0).transformative_bonus(40.0).hp(80.0).atk(-80.0)
        }
    }
}

#[derive(Debug)]
pub struct NoblesseOblige {
    timer: DurationTimer
}

impl NoblesseOblige {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 12.0) }
    }
}

impl SpecialAbility for NoblesseOblige {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Noblesse Oblige"),
            version: 1.0,
            preference: vec![Preference::Supporter],
            state: State::new().burst_dmg(20.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.kind == AttackType::Burst)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::NoblesseOblige() {
                    s.atk += 20.0;
                    s.stacked_buff += UnstackableBuff::NoblesseOblige();
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}


#[derive(Debug)]
pub struct TwoGfTwoNo;

impl SpecialAbility for TwoGfTwoNo {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("2 GF 2 NO"),
            version: 1.0,
            preference: vec![Preference::Supporter],
            state: State::new().burst_dmg(20.0).atk(18.0)
        }
    }
}

#[derive(Debug)]
pub struct GladiatorsFinale {
    bonus: f32,
    checked: bool,
}

impl GladiatorsFinale {
    fn new() -> Self {
        Self { bonus: 0.0, checked: false }
    }
}

impl SpecialAbility for GladiatorsFinale {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Gladiator's Finale"),
            version: 1.0,
            preference: vec![Preference::Melee],
            state: State::new().atk(18.0)
        }
    }

    fn update(&mut self, _gaurd: &mut TimerGuard, _attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, _time: f32) -> () {
        if !self.checked {
            self.checked = true;
            match owner_fc.cr.weapon.as_str() {
                "Sword"    => self.bonus = 35.0,
                "Claymore" => self.bonus = 35.0,
                "Polearm"  => self.bonus = 35.0,
                _ => ()
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        modifiable_state[owner_fc.idx.0].na_dmg += self.bonus;
    }

    fn reset(&mut self) -> () {
        self.bonus = 0.0;
        self.checked = false;
    }
}

#[derive(Debug)]
pub struct GladiatorsFinaleDef(GladiatorsFinale);

impl GladiatorsFinaleDef {
    fn new() -> Self {
        Self(GladiatorsFinale::new())
    }
}

impl SpecialAbility for GladiatorsFinaleDef {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Gladiator's Finale (DEF)"),
            version: 1.0,
            preference: vec![Preference::Melee],
            state: State::new().atk(18.0-80.0).def(110.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, enemy: &Enemy, time: f32) -> () {
        self.0.update(gaurd, attack, owner_fc, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

#[derive(Debug)]
pub struct WanderersTroupe;

impl SpecialAbility for WanderersTroupe {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Wanderer's Troupe"),
            version: 1.0,
            preference: vec![Preference::Ranged],
            state: State::new().ca_dmg(35.0).em(80.0)
        }
    }
}

#[derive(Debug)]
pub struct RetracingBolide;

impl SpecialAbility for RetracingBolide {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Retracing Bolide"),
            version: 1.0,
            preference: vec![Preference::Attacker],
            state: State::new().na_dmg(40.0).ca_dmg(40.0)
        }
    }
}

#[derive(Debug)]
pub struct RetracingBolideDef;

impl SpecialAbility for RetracingBolideDef {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Retracing Bolide (DEF)"),
            version: 1.0,
            preference: vec![Preference::Attacker],
            state: State::new().na_dmg(40.0).ca_dmg(40.0).atk(-80.0).def(110.0)
        }
    }
}

#[derive(Debug)]
pub struct Thundersoother;

impl SpecialAbility for Thundersoother {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Thundersoother"),
            version: 1.0,
            preference: vec![Preference::Electro],
            state: State::new()
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Electro => modifiable_state[owner_fc.idx.0].all_dmg += 35.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct Lavawalker;

impl SpecialAbility for Lavawalker {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Lavawalker"),
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new()
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Pyro => modifiable_state[owner_fc.idx.0].all_dmg += 35.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct LavawalkerHp;

impl SpecialAbility for LavawalkerHp {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Lavawalker (HP)"),
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new().atk(-80.0).hp(80.0)
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Pyro => modifiable_state[owner_fc.idx.0].all_dmg += 35.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct TwoGfTwoElemental;

impl SpecialAbility for TwoGfTwoElemental {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("2 GF 2 Elemental"),
            version: 1.0,
            preference: Vec::new(),
            state: State::new().atk(18.0).elemental_dmg(15.0)
        }
    }
}

#[derive(Debug)]
pub struct BlizzardStrayer;

impl SpecialAbility for BlizzardStrayer {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Blizzard Strayer"),
            version: 1.2,
            preference: vec![Preference::Cryo, Preference::Hydro],
            state: State::new().cryo_dmg(15.0)
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        match (enemy.isfrozen, &enemy.aura.aura) {
            (true,  Vision::Cryo) => modifiable_state[owner_fc.idx.0].cr += 40.0,
            (false, Vision::Cryo) => modifiable_state[owner_fc.idx.0].cr += 20.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct HeartOfDepth {
    timer: DurationTimer
}

impl HeartOfDepth {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 15.0) }
    }
}

impl SpecialAbility for HeartOfDepth {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Heart of Depth"),
            version: 1.2,
            preference: vec![Preference::Hydro],
            state: State::new().hydro_dmg(15.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.kind == AttackType::Skill)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].na_dmg += 30.0;
            modifiable_state[owner_fc.idx.0].ca_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}



#[derive(Debug)]
pub struct GlacierAndSnowfield {
    timer: DurationTimer
}

impl GlacierAndSnowfield {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 10.0) }
    }
}

impl SpecialAbility for GlacierAndSnowfield {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Glacier and Snowfield"),
            version: 99.0,
            preference: vec![Preference::Cryo],
            state: State::new().cryo_dmg(15.0).amplifying_bonus(15.0).transformative_bonus(100.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.kind == AttackType::Burst)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[owner_fc.idx.0].cryo_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct PaleFlame {
    timer: StackTimer
}

impl PaleFlame {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(0.3, 7.0, 2) }
    }
}

impl SpecialAbility for PaleFlame {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Pale Flame"),
            version: 1.5,
            preference: vec![Preference::Physical],
            state: State::new().physical_dmg(25.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.kind == Skill || a.kind == SkillDot)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            match self.timer.n {
                2 => {
                    modifiable_state[owner_fc.idx.0].atk += 18.0;
                    modifiable_state[owner_fc.idx.0].physical_dmg += 25.0;
                },
                1 => modifiable_state[owner_fc.idx.0].atk += 9.0,
                _ => (),
            };
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct TenacityOfTheMillelith {
    timer: DurationTimer
}

impl TenacityOfTheMillelith {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.5, 3.0) }
    }
}

impl SpecialAbility for TenacityOfTheMillelith {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Tenacity of the Millelith"),
            version: 1.5,
            preference: vec![Preference::Supporter],
            state: State::new().hp(20.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(gaurd.second(attack.iter().any(|a| a.kind == Skill || a.kind == SkillDot)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::TenacityOfTheMillelith() {
                    s.atk += 20.0;
                    s.stacked_buff += UnstackableBuff::TenacityOfTheMillelith();
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct ShimenawasReminiscence {
    first_activation: bool,
    cd: f32,
    duration: f32,
    _cd: f32,
    _dr: f32,
}

impl ShimenawasReminiscence {
    fn new() -> Self {
        Self { first_activation: false, cd: 0.0, duration: 10.0, _cd: 0.0, _dr: 0.0 }
    }
}

// 4 Piece: When casting an Elemental Skill, if the character has 15 or more
// Energy, they lose 15 Energy and Normal/Charged/ Plunging Attack DMG is
// increased by 50% for 10s.
impl SpecialAbility for ShimenawasReminiscence {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Shimenawa's Reminiscence"),
            version: 2.0,
            preference: vec![Preference::Attacker],
            state: State::new().atk(18.0)
        }
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let activation = attack.iter().any(|a| a.kind == AttackType::Skill) && owner_fc.state.energy.0 >= 15.0;
        gaurd.second(activation);
        gaurd.third(self._cd > 0.0);
        if !gaurd.check(()) {
            return;
        }
        if activation && self._cd <= 0.0 {
            self._cd = self.cd;
            self._dr = self.duration;
        }
        // notify the first time activation
        self.first_activation = self._dr == self.duration;
        self._cd -= time;
        self._dr -= time;
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self._dr > 0.0 && owner_fc.state.energy.0 >= 15.0 && self.first_activation {
            modifiable_state[owner_fc.idx.0].energy.0 -= 15.0;
            modifiable_state[owner_fc.idx.0].na_dmg += 50.0;
            modifiable_state[owner_fc.idx.0].ca_dmg += 50.0;
        } else if self._dr > 0.0 {
            modifiable_state[owner_fc.idx.0].na_dmg += 50.0;
            modifiable_state[owner_fc.idx.0].ca_dmg += 50.0;
        }
    }

    fn reset(&mut self) -> () {
        self._cd = 0.0;
        self._dr = 0.0;
    }
}

#[derive(Debug)]
pub struct TwoGfTwoShimenawa;

impl SpecialAbility for TwoGfTwoShimenawa {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("2 GF 2 Shimenawa"),
            version: 2.0,
            preference: Vec::new(),
            state: State::new().atk(36.0)
        }
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFate;

// 4 Piece: Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum
// 75% DMG increase can be obtained in this way.
impl SpecialAbility for EmblemOfSeveredFate {
    fn artifact(&self) -> Artifact {
        Artifact {
            name: String::from("Seal of Insulation"),
            version: 2.0,
            preference: Vec::new(),
            state: State::new().er(20.0)
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        // the maximum DMG bonus is obtained if ER is 300%.
        // `State.er` does not contain base 100% of characters.
        let er = 100.0 + owner_fc.state.er;
        modifiable_state[owner_fc.idx.0].burst_dmg += if er > 300.0 {
            75.0
        } else {
            er * 0.25
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulate::simulate;
    use crate::types::{ElementalGauge, ElementalGaugeDecay};
    use crate::fc::{FieldCharacterIndex, FieldAbility};
    use crate::testutil::{TestEnvironment, TestCharacter, TestWeapon};

    // fc0 triggers burst, which is invariant to fc1 who equips an artifact
    // that can be triggered by own burst.
    #[test]
    fn invariance_0() {
        let mut members = vec![
            TestEnvironment::fc(State::new()),
            TestEnvironment::fc_artifact(FieldCharacterIndex(1), NoblesseOblige::new()),
            ];
        members[0].0.state.energy.0 += members[0].0.state.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // (burst skill na na na) and (skill na na na)
        let expect = 0.5 * (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 0.5 * (200.0 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn invariance_1() {
        let mut members = vec![
            TestEnvironment::fc_artifact(FieldCharacterIndex(0), NoblesseOblige::new()),
            TestEnvironment::fc1(State::new()),
            ];
        members[0].0.state.energy.0 += members[0].0.state.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // (burst skill na na na) and (skill na na na)
        let expect = 0.5 * 1.2 * (360.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 0.5 * 1.2 * (200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn noblesse_oblige_unstackable() {
        let mut members = vec![
            TestEnvironment::fc_artifact(FieldCharacterIndex(0), NoblesseOblige::new()),
            TestEnvironment::fc_artifact(FieldCharacterIndex(1), NoblesseOblige::new()),
            ];
        members[0].0.state.energy.0 += members[0].0.state.energy_cost;
        members[1].0.state.energy.0 += members[1].0.state.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // (burst skill na na na) and (burst skill na na na)
        let expect = 0.5 * 1.2 * (360.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 0.5 * 1.2 * (360.0 + 200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn viridescent_venerer() {
        let mut members = vec![
            FieldAbility::boxed(
                TestCharacter { vision: String::from("Anemo") },
                TestWeapon,
                ViridescentVenerer,
            ).to_data(FieldCharacterIndex(0))
        ];
        members[0].0.ar.state.infusion = true;
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Vision::Pyro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        let expect = 0.5 * (
            // skill (level multiplier * reaction multiplier * bonus (* bypass enemy defense))
              725.36 * 1.2 * 1.6 * 2.0 + 200.0 * 1.15 * 1.2
            // na
            + 725.36 * 1.2 * 1.6 * 2.0 + 100.0 * 1.15 * 1.2
            // na (action multiplier * vv 2 set bonus * vv 4 set RES down)
            + 100.0 * 1.15 * 1.2
            // na
            + 100.0 * 1.15 * 1.2
        );
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }
}
