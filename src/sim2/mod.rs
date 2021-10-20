use std::error::Error;
use std::env;
use std::io;
use std::process;
use std::cmp::Ordering;
use std::mem::MaybeUninit;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration};

pub mod artifact;
pub mod attack;
pub mod element;
pub mod record;
pub mod simulate;
pub mod state;
pub mod testutil;
pub mod timeline;
pub mod types;
pub mod characters;
pub mod weapons;

use crate::sim1::permutools::Permutation3;
use crate::sim1::cli::Args;

use crate::sim2::state::State;
use crate::sim2::timeline::ActionState;
use crate::sim2::attack::Attack;
use crate::sim2::simulate::History;
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, Preference};
use crate::sim2::record::{TimelineMember, FieldMember, CharacterData, CharacterRecord, WeaponRecord, Artifact, Enemy};

#[derive(Debug, Eq)]
struct Recorder {
    head: Vec<&'static str>,
    data: Vec<usize>,
}

impl Ord for Recorder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.last().unwrap().cmp(&other.data.last().unwrap())
    }
}

impl PartialOrd for Recorder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Recorder {
    fn eq(&self, other: &Self) -> bool {
        self.data.last().unwrap() == other.data.last().unwrap()
    }
}

trait NewRecorder<T> {
    fn new(item: T) -> Recorder;
}

impl NewRecorder<Vec<&'static str>> for Recorder {
    fn new(item: Vec<&'static str>) -> Self {
        Self {
            head: item,
            data: Vec::new()
        }
    }
}

impl NewRecorder<&CharacterData<'_>> for Recorder {
    fn new(item: &CharacterData) -> Self {
        Self {
            head: vec![item.character.name, item.weapon.name, item.artifact.name],
            data: Vec::new()
        }
    }
}

impl NewRecorder<&(&CharacterRecord, &WeaponRecord, &Artifact)> for Recorder {
    fn new(item: &(&CharacterRecord, &WeaponRecord, &Artifact)) -> Self {
        let (cr, wr, ar) = item;
        Self {
            head: vec![cr.name, wr.name, ar.name],
            data: Vec::new()
        }
    }
}

impl Recorder {
    fn record(&mut self, time: f32, value: f32) -> () {
        if self.data.len() == time.floor() as usize {
            self.data.push(value.floor() as usize);
        }
    }

    fn make_row(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(self.head.len() + self.data.len());
        // result.append(&mut self.head);
        for x in &self.head {
            result.push(x.to_string());
        }
        for x in &self.data {
            result.push(x.to_string());
        }
        result
    }
}

fn combination_filter_attacker(cr: &CharacterRecord, wr: &WeaponRecord, ar: &Artifact, args: &Args) -> bool {
    if cr.version > args.character_version ||
       wr.version > args.weapon_version ||
       ar.version > args.artifact_version {
        return false;
    }

    // check weapon
    if cr.weapon != wr.type_ {
        return false;
    }

    // check artifact
    let physical_attack = ar.is_physical_goblet_user(&cr.name);
    let mut result = if ar.preference.len() == 0 {
        true
    } else {
        false
    };
    for p in ar.preference.iter() {
        if p == &cr.vision
        || (physical_attack && p == &Preference::Physical)
        || p == &cr.weapon
        || p == &Preference::Attacker {
            result = true;
            break;
        }
    }

    result
}

fn combination_filter_supporter(cr: &CharacterRecord, wr: &WeaponRecord, ar: &Artifact, args: &Args) -> bool {
    if cr.version > args.character_version ||
       wr.version > args.weapon_version ||
       ar.version > args.artifact_version {
        return false;
    }

    // check weapon
    if cr.weapon != wr.type_ {
        return false;
    }

    // check artifact
    let mut result = if ar.preference.len() == 0 {
        true
    } else {
        false
    };
    for p in ar.preference {
        if *p == cr.vision
        || *p == cr.weapon
        || *p == Preference::Supporter {
            result = true;
            break;
        }
    }

    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn filter_1() {
//         let c = characters::pyro::Diluc::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::ViridescentVenerer::record();
//         let args = Args::default();
//         assert!(!combination_filter_attacker(&c, &w, &a, &args));
//         assert!(!combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_2() {
//         let c = characters::pyro::Diluc::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::GladiatorsFinale::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_3() {
//         let c = characters::electro::Razor::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::PaleFlame::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         // assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_4() {
//         let c = characters::electro::Razor::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::ThunderingFury::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_5() {
//         let c = characters::hydro::Xingqiu::record();
//         let w = weapons::sword_4star::PrototypeRancourR5::record();
//         let a = artifact::BlizzardStrayer::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_6() {
//         let c = characters::cryo::Kaeya::record();
//         let w = weapons::sword_4star::PrototypeRancourR5::record();
//         let a = artifact::BlizzardStrayer::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }
// }

fn permu3(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let input_characters: Vec<(CharacterRecord, characters::CharacterUnion)> = characters::all().drain(start..end).collect();
    let mut member1 = Permutation3::new(
        input_characters,
        weapons::all(),
        artifact::all(),
    );

    let mut items: Vec<Recorder> = Vec::new();
    for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
        if !combination_filter_attacker(&cr1, &wr1, &ar1, args) {
            member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
            continue;
        }

        let mut enemy = Enemy::hilichurl();
        let mut recorder = Recorder::new(&(&cr1, &wr1, &ar1));
        let mut history = History::<1>::new(args.simulation_time, args.unit_time);
        let dmg: f32;

        ar1.flat_atk = 311.;
        ar1.infuse_goblet(&cr1.vision, &cr1.name);

        let mut data = [CharacterData::new(0, &cr1, &wr1, &ar1); 1];
        {
            let mut members = [TimelineMember {
                character: ca1.timeline(),
                weapon: wa1.timeline(),
                artifact: aa1.timeline(),
            }; 1];
            let mut states = [ActionState::new(); 1];
            states[0].energy = if args.start_energy < 0 {
                cr1.energy_cost
            } else {
                args.start_energy as f32
            };
            simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
            members[0].character.reset_timeline();
            members[0].weapon.reset_timeline();
            members[0].artifact.reset_timeline();
        }
        {
            let mut members = [FieldMember {
                character: ca1.field(),
                weapon: wa1.field(),
                artifact: aa1.field(),
            }; 1];
            dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
            members[0].character.reset_modify();
            members[0].weapon.reset_modify();
            members[0].artifact.reset_modify();
        }

        // destruct objects
        ar1.dry_goblet();
        member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
        recorder.data.push(dmg as usize);
        items.push(recorder);
    }
    tx.send(items).unwrap();
}

fn permu6(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let input_characters: Vec<(CharacterRecord, characters::CharacterUnion)> = characters::all().drain(start..end).collect();
    let mut member1 = Permutation3::new(
        input_characters,
        weapons::all(),
        artifact::all(),
    );
    let mut member2 = Permutation3::new(
        characters::all(),
        weapons::all(),
        artifact::all(),
    );

    for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
        if !combination_filter_attacker(&cr1, &wr1, &ar1, args) {
            member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
            continue;
        }
        ar1.flat_atk = 311.;
        ar1.infuse_goblet(&cr1.vision, &cr1.name);
        let mut items: Vec<Recorder> = Vec::new();
        for ((cr2, mut ca2), (wr2, mut wa2), (mut ar2, mut aa2)) in member2.iter() {
            if cr1.name == cr2.name || !combination_filter_attacker(&cr2, &wr2, &ar2, args) {
                member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
                continue;
            }

            let mut enemy = Enemy::hilichurl();
            let mut history = History::<2>::new(args.simulation_time, args.unit_time);
            let dmg: f32;

            // supporter role
            ar2.atk_spd = -80.;
            ar2.flat_atk = 311.;
            ar2.infuse_goblet(&cr2.vision, &cr2.name);

            let mut data = [CharacterData::new(0, &cr1, &wr1, &ar1),CharacterData::new(1, &cr2, &wr2, &ar2),];
            let mut head: Vec<&'static str> = Vec::new();
            for i in data.iter() {
                head.push(i.character.name);
                head.push(i.weapon.name);
                head.push(i.artifact.name);
            }
            let mut recorder = Recorder::new(head);
            {
                let mut members = [TimelineMember {
                    character: ca1.timeline(),
                    weapon: wa1.timeline(),
                    artifact: aa1.timeline(),
                }, TimelineMember {
                    character: ca2.timeline(),
                    weapon: wa2.timeline(),
                    artifact: aa2.timeline(),
                }, ];
                let mut states = [ActionState::new(); 2];
                states[0].energy = if args.start_energy < 0 {
                    cr1.energy_cost
                } else {
                    args.start_energy as f32
                };
                states[1].energy = if args.start_energy < 0 {
                    cr2.energy_cost
                } else {
                    args.start_energy as f32
                };
                simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
                for m in members.iter_mut() {
                    m.character.reset_timeline();
                    m.weapon.reset_timeline();
                    m.artifact.reset_timeline();
                }
            }
            {
                let mut members = [FieldMember {
                    character: ca1.field(),
                    weapon: wa1.field(),
                    artifact: aa1.field(),
                }, FieldMember {
                    character: ca2.field(),
                    weapon: wa2.field(),
                    artifact: aa2.field(),
                }, ];
                dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
                for m in members.iter_mut() {
                    m.character.reset_modify();
                    m.weapon.reset_modify();
                    m.artifact.reset_modify();
                }
            }

            // destruct objects
            ar2.dry_goblet();
            member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
            recorder.data.push(dmg as usize);
            items.push(recorder);
        }
        ar1.dry_goblet();
        member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
        tx.send(items).unwrap();
    }
}

// fn debugging(args: &Args, debug_args: Vec<String>) -> () {
//     let idx = FieldCharacterIndex(0);
//     let mut timers = ICDTimers::new();
//     let mut builder = FieldAbilityBuilder::new();
//     let mut characters = AllCharacters::new(idx, &timers);
//     let mut weapons = AllWeapons::new(idx, &timers);
//     let mut artifacts = AllArtifacts::new(idx);
//     let mut character_name = MaybeUninit::<CharacterName>::uninit();
//     let mut weapon_name = MaybeUninit::<WeaponName>::uninit();
//     let mut artifact_name = MaybeUninit::<ArtifactName>::uninit();
//     let mut members: Vec<CharacterData> = Vec::new();
//     let mut abilities: Vec<FieldAbility> = Vec::new();
//     for i in 0..3 {
//         match i {
//             0 => unsafe { character_name.as_mut_ptr().write(CharacterName::from(debug_args[0].as_str())) },
//             1 => unsafe { weapon_name.as_mut_ptr().write(WeaponName::from(debug_args[1].as_str())) },
//             2 => unsafe { artifact_name.as_mut_ptr().write(ArtifactName::from(debug_args[2].as_str())) },
//             _ => (),
//         }
//     };
//     let (cr1, ca1) = characters.find(unsafe { &character_name.assume_init() }, &mut builder);
//     let (wr1, wa1) = weapons.find(unsafe { &weapon_name.assume_init() }, &mut builder);
//     let (ar1, aa1) = artifacts.find(unsafe { &artifact_name.assume_init() }, &mut builder);
//     let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];
//     if physical_infusion.contains(&cr1.name) {
//         ar1.infuse_goblet(&Vision::Physical);
//     } else {
//         ar1.infuse_goblet(&cr1.vision);
//     }
//     members.push(CharacterData::new(idx, &cr1, &wr1, &ar1));
//     abilities.push(builder.build(&mut timers));
//     println!("debugging: {:?} {:?} {:?}", cr1.name, wr1.name, ar1.name);
//     main_loop(&mut members, &mut abilities, args);
// }

fn start_and_wait() -> Result<(), Box<dyn Error + 'static>> {
    let mut debug_args: Vec<String> = Vec::new();
    let args = Args::parse(&mut env::args(), &mut debug_args)?;
    if debug_args.len() > 0 {
        return Ok(());
        // return Ok(debugging(&args, debug_args));
    }
    let num_cpu = 4;
    let character_size = characters::N_CHARACTERS;
    let chunk_size = character_size / num_cpu + 1;
    let (tx, rx) = mpsc::channel();
    if num_cpu == 1 {
        permu3(tx, 0, character_size, &args);
    } else {
        for i in 0..num_cpu {
            let start = i * chunk_size;
            let mut end = (i + 1) * chunk_size;
            if start >= character_size {
                break;
            } else if end > character_size {
                end = character_size;
            }
            let txn = tx.clone();
            match args.n_members {
                1 => thread::spawn(move || permu3(txn, start, end, &args) ),
                2 => thread::spawn(move || permu6(txn, start, end, &args) ),
                _ => unimplemented!(),
            };
        }
        drop(tx);
    }
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut count = 0;
    for mut received in rx {
        if args.truncate && args.n_members > 1 {
            received.sort();
            received.drain(..received.len() / 2);
        }
        for rc in received.iter_mut() {
            wtr.write_record(&rc.make_row())?;
        }
        wtr.flush()?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn main2() {
    // thread::sleep(Duration::from_secs(10));
    if let Err(err) = start_and_wait() {
        println!("{:?}", err);
        process::exit(1);
    };
}
