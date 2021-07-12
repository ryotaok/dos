#![feature(destructuring_assignment)]

use std::error::Error;
use std::io;
use std::process;
use std::cmp::Ordering;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration};

mod action;
mod state;
mod types;
mod fc;
mod simulate;
mod characters;
mod weapons;
mod artifact;
mod permutools;
mod testutil;
mod cli;

use crate::artifact::Artifact;
use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, Enemy, FieldCharacter, FieldCharacterData, FieldAbility, SpecialAbility};
use crate::types::{Vision, Preference};
// use crate::testutil::{Srdst};
use crate::permutools::Permutation3;
use crate::cli::Args;

#[derive(Debug, Eq)]
struct Recorder {
    head: Vec<String>,
    data: Vec<i32>,
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

impl NewRecorder<Vec<String>> for Recorder {
    fn new(item: Vec<String>) -> Self {
        Self {
            head: item,
            data: Vec::new()
        }
    }
}

impl NewRecorder<&FieldCharacter> for Recorder {
    fn new(item: &FieldCharacter) -> Self {
        Self {
            head: vec![item.cr.name.clone(), item.wr.name.clone(), item.ar.name.clone()],
            data: Vec::new()
        }
    }
}

impl NewRecorder<&(&CharacterRecord, &WeaponRecord, &Artifact)> for Recorder {
    fn new(item: &(&CharacterRecord, &WeaponRecord, &Artifact)) -> Self {
        let (cr, wr, ar) = item;
        Self {
            head: vec![cr.name.clone(), wr.name.clone(), ar.name.clone()],
            data: Vec::new()
        }
    }
}

impl Recorder {
    fn record(&mut self, time: f32, value: f32) -> () {
        if self.data.len() == time.floor() as usize {
            self.data.push(value.floor() as i32);
        }
    }

    fn make_row(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.append(&mut self.head);
        for x in &self.data {
            result.push(x.to_string());
        }
        result
    }
}

fn main_loop(members: &mut [FieldCharacterData], time: f32, emulation_time: f32) -> Recorder {
    let mut head: Vec<String> = Vec::new();
    for m in members.iter() {
        head.push(m.0.cr.name.clone());
        head.push(m.0.wr.name.clone());
        head.push(m.0.ar.name.clone());
    }
    let mut rc = Recorder::new(head);
    let mut enemy = Enemy::hilichurl();
    // setup for Emulator
    let mut current_time = 0.0;
    let mut total_dmg = 0.0;
    while current_time < emulation_time {
        if current_time == 0.0 {
            for m in members.iter_mut() {
                m.0.state.energy.0 = m.0.state.energy_cost;
            }
        }
        if current_time == time * 5.0 {
            for m in members.iter_mut() {
                m.0.state.energy.0 += 15.0 * m.0.state.ER();
            }
        }
        total_dmg += simulate::simulate(members, &mut enemy, time);
        current_time += time;
        rc.record(current_time, total_dmg);
    }
    rc
}

type Permu3Item = (
    (CharacterRecord, Vision, Box<dyn SpecialAbility>),
    (WeaponRecord, Box<dyn SpecialAbility>),
    (Artifact, Box<dyn SpecialAbility>),
);

fn combination_filter_attacker(((cr, vision, _ca), (wr, _wa), (ar, _aa)): &Permu3Item) -> bool {
    // TODO
    let physical_infusion: Vec<String> = vec![String::from("Razor"), String::from("Eula"), ];

    // check weapon
    if cr.weapon != wr.type_ {
        return false;
    }

    // check artifact
    let physical_attack = physical_infusion.contains(&cr.name);
    for p in &ar.preference {
        if p != vision
        && (physical_attack && *p != Preference::Physical)
        && *p != cr.weapon
        && *p != Preference::Attacker {
            return false;
        }
    }

    true
}

fn combination_filter_supporter(((cr, vision, _ca), (wr, _wa), (ar, _aa)): &Permu3Item) -> bool {
    // check weapon
    if cr.weapon != wr.type_ {
        return false;
    }

    // check artifact
    for p in &ar.preference {
        if p != vision
        && *p != cr.weapon
        && *p != Preference::Supporter {
            return false;
        }
    }

    true
}

fn permu6(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let input_characters: Vec<(CharacterRecord, Vision, Box<dyn SpecialAbility>)> = characters::setup(args).drain(start..end).collect();
    // TODO
    let physical_infusion: Vec<String> = vec![String::from("Razor"), String::from("Eula"), ];
    let mut member1 = Permutation3::new(
        input_characters,
        weapons::setup(args),
        Artifact::setup(args.artifact_version),
    );
    let mut member2 = Permutation3::new(
        characters::setup(args),
        weapons::setup(args),
        Artifact::setup(args.artifact_version),
    );
    for item1 in member1.iter() {
        if !combination_filter_attacker(&item1) {
            member1.back(item1);
            continue;
        }
        let ((mut cr1, mut vision1, mut ca1), (mut wr1, mut wa1), (mut ar1, mut aa1)) = item1;
        let mut items: Vec<Recorder> = Vec::with_capacity(member2.size());
        for item2 in member2.iter() {
            if !combination_filter_supporter(&item2) {
                member2.back(item2);
                continue;
            }
            // identical characters cannot be on field.
            if cr1.name == item2.0.0.name {
                member2.back(item2);
                continue;
            }
            let ((mut cr2, mut vision2, mut ca2), (mut wr2, mut wa2), (mut ar2, mut aa2)) = item2;
            if physical_infusion.contains(&cr1.name) {
                ar1.infuse_goblet(&Vision::Physical);
            } else {
                ar1.infuse_goblet(&vision1);
            }
            ar2.infuse_goblet(&vision2);
            let mut members = vec![
                FieldCharacter::new(FieldCharacterIndex(0), cr1, vision1, wr1, ar1).to_data(FieldAbility {
                    character: ca1,
                    weapon: wa1,
                    artifact: aa1,
                }),
                FieldCharacter::new(FieldCharacterIndex(1), cr2, vision2, wr2, ar2).noop(FieldAbility {
                    character: ca2,
                    weapon: wa2,
                    artifact: aa2,
                }),
                ];
            let rc = main_loop(&mut members, args.unit_time, args.emulation_time);
            items.push(rc);

            // destruct objects
            let (fc, field_ability, _, _) = members.remove(0);
            FieldCharacter { cr: cr1, wr: wr1, ar: ar1, vision: vision1, .. } = fc;
            FieldAbility { character: ca1, weapon: wa1, artifact: aa1 } = field_ability;
            ca1.reset();
            wa1.reset();
            aa1.reset();
            let (fc, field_ability, _, _) = members.remove(0);
            FieldCharacter { cr: cr2, wr: wr2, ar: ar2, vision: vision2, .. } = fc;
            FieldAbility { character: ca2, weapon: wa2, artifact: aa2 } = field_ability;
            ca2.reset();
            wa2.reset();
            aa2.reset();
            member2.back(((cr2, vision2, ca2), (wr2, wa2), (ar2, aa2)));
        }
        member2.reset();
        member1.back(((cr1, vision1, ca1), (wr1, wa1), (ar1, aa1)));
        tx.send(items).unwrap();
    }
}

fn permu3(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let input_characters: Vec<(CharacterRecord, Vision, Box<dyn SpecialAbility>)> = characters::setup(args).drain(start..end).collect();
    // TODO
    let physical_infusion: Vec<String> = vec![String::from("Razor"), String::from("Eula"), ];
    let mut member1 = Permutation3::new(
        input_characters,
        weapons::setup(args),
        Artifact::setup(args.artifact_version),
    );
    let mut items: Vec<Recorder> = Vec::with_capacity(member1.size());
    for item1 in member1.iter() {
        if !combination_filter_attacker(&item1) {
            member1.back(item1);
            continue;
        }
        let ((mut cr1, mut vision1, mut ca1), (mut wr1, mut wa1), (mut ar1, mut aa1)) = item1;
        if physical_infusion.contains(&cr1.name) {
            ar1.infuse_goblet(&Vision::Physical);
        } else {
            ar1.infuse_goblet(&vision1);
        }
        let mut members = vec![
                FieldCharacter::new(FieldCharacterIndex(0), cr1, vision1, wr1, ar1).to_data(FieldAbility {
                    character: ca1,
                    weapon: wa1,
                    artifact: aa1,
                }),
            ];
        let rc = main_loop(&mut members, args.unit_time, args.emulation_time);
        items.push(rc);

        // destruct objects
        let (fc, field_ability, _, _) = members.remove(0);
        FieldCharacter { cr: cr1, wr: wr1, ar: ar1, vision: vision1, .. } = fc;
        FieldAbility { character: ca1, weapon: wa1, artifact: aa1 } = field_ability;
        ca1.reset();
        wa1.reset();
        aa1.reset();
        member1.back(((cr1, vision1, ca1), (wr1, wa1), (ar1, aa1)));
    }
    tx.send(items).unwrap();
}

fn start_and_wait() -> Result<(), Box<dyn Error + 'static>> {
    let args = Args::parse()?;
    let num_cpu = 3;
    let character_size = characters::setup(&args).len();
    // let character_size = 6;
    let chunk_size = character_size / num_cpu;
    let (tx, rx) = mpsc::channel();
    for i in 0..(num_cpu+1) {
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
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for mut received in rx {
        if args.truncate && args.n_members > 1 {
            received.sort();
            received.drain(..received.len() / 2);
        }
        for rc in &mut received {
            wtr.write_record(&rc.make_row())?;
        }
        wtr.flush()?;
    }
    Ok(())
}

fn main() {
    // thread::sleep(Duration::from_secs(10));
    // let character_file = "/Users/ryota/Documents/Genshin DPS rust 2/Damage & DPS-Characters.csv";
    // let weapon_file = "/Users/ryota/Documents/Genshin DPS rust 2/Damage & DPS-Weapons.csv";
    if let Err(err) = start_and_wait() {
        println!("error running example: {}", err);
        process::exit(1);
    };
}
