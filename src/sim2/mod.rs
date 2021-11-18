use std::error::Error;
use std::env;
use std::io;
use std::process;
use std::cmp::Ordering;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration};
use std::collections::HashMap;

pub mod artifact;
pub mod attack;
pub mod cli;
pub mod element;
pub mod record;
pub mod simulate;
pub mod state;
pub mod testutil;
pub mod timeline;
pub mod types;
pub mod training;
pub mod characters;
pub mod weapons;

use crate::sim1::permutools::Permutation3;

use crate::sim2::cli::Args;
use crate::sim2::state::State;
use crate::sim2::timeline::ActionState;
use crate::sim2::attack::{Attack, DamageResult};
use crate::sim2::simulate::History;
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, Preference, combination_filter};
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
    fn new(end_time: f32, item: T) -> Recorder;
}

impl NewRecorder<Vec<&'static str>> for Recorder {
    fn new(end_time: f32,item: Vec<&'static str>) -> Self {
        Self {
            head: item,
            data: vec![0; end_time as usize]
        }
    }
}

impl NewRecorder<&CharacterData<'_>> for Recorder {
    fn new(end_time: f32,item: &CharacterData) -> Self {
        Self {
            head: vec![item.character.name, item.weapon.name, item.artifact.name],
            data: vec![0; end_time as usize]
        }
    }
}

impl NewRecorder<&(&CharacterRecord, &WeaponRecord, &Artifact)> for Recorder {
    fn new(end_time: f32,item: &(&CharacterRecord, &WeaponRecord, &Artifact)) -> Self {
        let (cr, wr, ar) = item;
        Self {
            head: vec![cr.name, wr.name, ar.name],
            data: vec![0; end_time as usize]
        }
    }
}

impl Recorder {
    fn record(&mut self, time: f32, value: f32) -> () {
        // if self.data.len() == time.floor() as usize {
        //     self.data.push(value.floor() as usize);
        // }
        let mut idx = time.floor() as usize;
        if idx == self.data.len() {
            idx -= 1;
        }
        self.data[idx] += value.floor() as usize;
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

fn permu3(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let input_characters: Vec<(CharacterRecord, characters::CharacterUnion)> = characters::all().drain(start..end).collect();
    let mut member1 = Permutation3::new(
        input_characters,
        weapons::all(),
        artifact::all(),
    );

    let mut items: Vec<Recorder> = Vec::new();
    for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
        if !combination_filter(&cr1, &wr1, &ar1, args) {
            member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
            continue;
        }

        let mut enemy = Enemy::hilichurl();
        let mut recorder = Recorder::new(args.simulation_time, &(&cr1, &wr1, &ar1));
        let mut history = History::<1>::new(args.simulation_time, args.unit_time);
        let dmg: Vec<DamageResult>;

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
        for r in dmg.iter() {
            recorder.record(r.time, r.total_damage());
        }
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
    // let mut member1 = Permutation3::new(
    //     input_characters,
    //     training::weapons(),
    //     training::artifacts(),
    // );
    // let mut member2 = Permutation3::new(
    //     characters::all(),
    //     training::weapons(),
    //     training::artifacts(),
    // );

    let mut cache: HashMap<(&'static str,&'static str), History<2>> = HashMap::new();
    for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
        if !combination_filter(&cr1, &wr1, &ar1, args) {
            member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
            continue;
        }
        ar1.flat_atk = 311.;
        ar1.infuse_goblet(&cr1.vision, &cr1.name);
        let mut items: Vec<Recorder> = Vec::new();
        for ((cr2, mut ca2), (wr2, mut wa2), (mut ar2, mut aa2)) in member2.iter() {
            if cr1.name == cr2.name || !combination_filter(&cr2, &wr2, &ar2, args) {
                member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
                continue;
            }

            let mut enemy = Enemy::hilichurl();
            let (init_history, mut history) = if let Some(h) = cache.remove(&(cr1.name, cr2.name)) {
                (false, h)
            } else {
                (true, History::<2>::new(args.simulation_time, args.unit_time))
            };
            let dmg: Vec<DamageResult>;

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
            let mut recorder = Recorder::new(args.simulation_time, head);
            if init_history {
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
            cache.insert((cr1.name, cr2.name), history);
            ar2.dry_goblet();
            member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
            for r in dmg.iter() {
                recorder.record(r.time, r.total_damage());
            }
            items.push(recorder);
        }
        ar1.dry_goblet();
        member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
        tx.send(items).unwrap();
    }
}

fn permu9(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let input_characters: Vec<(CharacterRecord, characters::CharacterUnion)> = characters::all().drain(start..end).collect();
    let mut member1 = Permutation3::new(
        input_characters,
        training::weapons(),
        training::artifacts(),
    );
    let mut member2 = Permutation3::new(
        characters::all(),
        training::weapons(),
        training::artifacts(),
    );
    let mut member3 = Permutation3::new(
        characters::all(),
        training::weapons(),
        training::artifacts(),
    );

    for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
        if !combination_filter(&cr1, &wr1, &ar1, args) {
            member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
            continue;
        }
        ar1.flat_atk = 311.;
        ar1.infuse_goblet(&cr1.vision, &cr1.name);
        let mut no_dup: Vec<&'static str> = Vec::new();
        for ((cr2, mut ca2), (wr2, mut wa2), (mut ar2, mut aa2)) in member2.iter() {
            if cr1.name == cr2.name || !combination_filter(&cr2, &wr2, &ar2, args) {
                member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
                continue;
            }

            // supporter role
            ar2.atk_spd = -80.;
            ar2.flat_atk = 311.;
            ar2.infuse_goblet(&cr2.vision, &cr2.name);

            let mut items: Vec<Recorder> = Vec::new();
            for ((cr3, mut ca3), (wr3, mut wa3), (mut ar3, mut aa3)) in member3.iter() {
                if no_dup.contains(&cr3.name) || cr1.name == cr3.name || cr2.name == cr3.name || !combination_filter(&cr3, &wr3, &ar3, args) {
                    member3.back(((cr3, ca3), (wr3, wa3), (ar3, aa3)));
                    continue;
                }

                // supporter role
                ar3.atk_spd = -80.;
                ar3.flat_atk = 311.;
                ar3.infuse_goblet(&cr3.vision, &cr3.name);

                let mut enemy = Enemy::hilichurl();
                let mut history = History::<3>::new(args.simulation_time, args.unit_time);
                let dmg: Vec<DamageResult>;

                let mut data = [CharacterData::new(0, &cr1, &wr1, &ar1),CharacterData::new(1, &cr2, &wr2, &ar2),CharacterData::new(2, &cr3, &wr3, &ar3),];
                let mut head: Vec<&'static str> = Vec::new();
                for i in data.iter() {
                    head.push(i.character.name);
                    head.push(i.weapon.name);
                    head.push(i.artifact.name);
                }
                let mut recorder = Recorder::new(args.simulation_time, head);
                {
                    let mut members = [TimelineMember {
                        character: ca1.timeline(),
                        weapon: wa1.timeline(),
                        artifact: aa1.timeline(),
                    }, TimelineMember {
                        character: ca2.timeline(),
                        weapon: wa2.timeline(),
                        artifact: aa2.timeline(),
                    }, TimelineMember {
                        character: ca3.timeline(),
                        weapon: wa3.timeline(),
                        artifact: aa3.timeline(),
                    }, ];
                    let mut states = [ActionState::new(); 3];
                    if args.start_energy < 0 {
                        states[0].energy = cr1.energy_cost;
                        states[1].energy = cr2.energy_cost;
                        states[2].energy = cr3.energy_cost;
                    } else {
                        let energy_cost = args.start_energy as f32;
                        states[0].energy = energy_cost;
                        states[1].energy = energy_cost;
                        states[2].energy = energy_cost;
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
                    }, FieldMember {
                        character: ca3.field(),
                        weapon: wa3.field(),
                        artifact: aa3.field(),
                    }, ];
                    dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
                    for m in members.iter_mut() {
                        m.character.reset_modify();
                        m.weapon.reset_modify();
                        m.artifact.reset_modify();
                    }
                }

                // destruct objects
                ar3.dry_goblet();
                member3.back(((cr3, ca3), (wr3, wa3), (ar3, aa3)));
                for r in dmg.iter() {
                    recorder.record(r.time, r.total_damage());
                }
                items.push(recorder);
            }
            if !no_dup.contains(&cr2.name) {
                no_dup.push(cr2.name);
            }
            ar2.dry_goblet();
            member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
            tx.send(items).unwrap();
        }
        ar1.dry_goblet();
        member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
    }
}

fn start_and_wait() -> Result<(), Box<dyn Error + 'static>> {
    let mut debug_args: Vec<String> = Vec::new();
    let args = Args::parse(&mut env::args(), &mut debug_args)?;
    if debug_args.len() > 0 {
        return Ok(());
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
                3 => thread::spawn(move || permu9(txn, start, end, &args) ),
                _ => unimplemented!(),
            };
        }
        drop(tx);
    }
    let mut wtr = csv::Writer::from_writer(io::stdout());
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
