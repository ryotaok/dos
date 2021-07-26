// #![feature(destructuring_assignment)]
#![feature(unsized_tuple_coercion)]

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

use crate::state::State;
use crate::characters::{AllCharacters, CharacterName};
use crate::weapons::{AllWeapons, WeaponName};
use crate::artifact::{Artifact, AllArtifacts, ArtifactName};
use crate::action::{CharacterTimersBuilder};
use crate::fc::{FieldCharacterIndex, CharacterRecord, WeaponRecord, CharacterData, Enemy, FieldCharacterData};
use crate::types::{Vision, Preference};
use crate::permutools::Permutation3;
use crate::cli::Args;

#[derive(Debug, Eq)]
struct Recorder {
    head: Vec<&'static str>,
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
            head: vec![item.cr.name, item.wr.name, item.ar.name],
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
            self.data.push(value.floor() as i32);
        }
    }

    fn make_row(&mut self) -> Vec<String> {
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

fn main_loop(members: &mut [FieldCharacterData], time: f32, simulation_time: f32) -> Recorder {
    let mut head: Vec<&str> = Vec::new();
    for m in members.iter() {
        head.push(m.fc.data.cr.name);
        head.push(m.fc.data.wr.name);
        head.push(m.fc.data.ar.name);
    }
    let mut rc = Recorder::new(head);
    let mut enemy = Enemy::hilichurl();
    // setup for Emulator
    let mut current_time = 0.0;
    let mut total_dmg = 0.0;
    // accumulate passives of weapons and artifacts at the 0 second.
    {
        let mut modifiable_state: Vec<State> = Vec::with_capacity(members.len());
        for _ in 0..members.len() {
            modifiable_state.push(State::new());
        }
        for FieldCharacterData { fc, .. } in members.iter_mut() {
            fc.modify(&mut modifiable_state, &mut enemy);
        }
        for (state, FieldCharacterData { fc, .. }) in modifiable_state.into_iter().zip(members.iter_mut()) {
            fc.data.state.merge(&state);
        }
    }
    while current_time < simulation_time {
        if current_time == 0.0 {
            for m in members.iter_mut() {
                m.fc.data.state.energy.0 = m.fc.data.state.energy_cost;
            }
        }
        if current_time == time * 5.0 {
            for m in members.iter_mut() {
                m.fc.data.state.energy.0 += 15.0 * m.fc.data.state.ER();
            }
        }
        total_dmg += simulate::simulate(members, &mut enemy, time);
        current_time += time;
        rc.record(current_time, total_dmg);
    }
    rc
}

// type Permu3Item = &(
//     (CharacterRecord, dyn SpecialAbility),
//     (WeaponRecord, dyn SpecialAbility),
//     (Artifact, dyn SpecialAbility),
// );

// fn combination_filter_attacker(((cr, _ca), (wr, _wa), (ar, _aa)): Permu3Item, args: &Args) -> bool {
fn combination_filter_attacker(cr: &CharacterRecord, wr: &WeaponRecord, ar: &Artifact, args: &Args) -> bool {
    // TODO
    let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];

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
    let physical_attack = physical_infusion.contains(&cr.name);
    for p in &ar.preference {
        if *p != cr.vision
        && (physical_attack && *p != Preference::Physical)
        && *p != cr.weapon
        && *p != Preference::Attacker {
            return false;
        }
    }

    true
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
    for p in &ar.preference {
        if *p != cr.vision
        && *p != cr.weapon
        && *p != Preference::Supporter {
            return false;
        }
    }

    true
}

fn permu6(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let idx0 = FieldCharacterIndex(0);
    let idx1 = FieldCharacterIndex(1);
    let mut timers0 = Box::new(CharacterTimersBuilder::new().build());
    let mut timers1 = Box::new(CharacterTimersBuilder::new().build());
    let mut characters1 = AllCharacters::new(idx0);
    let mut characters2 = AllCharacters::new(idx1);
    let mut weapons1 = AllWeapons::new(idx0);
    let mut weapons2 = AllWeapons::new(idx1);
    let mut artifacts1 = AllArtifacts::new();
    let mut artifacts2 = AllArtifacts::new();
    let input_characters: Vec<CharacterName> = CharacterName::vec().drain(start..end).collect();
    // TODO
    let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];

    let mut member1 = Permutation3::new(
        input_characters,
        WeaponName::vec(),
        ArtifactName::vec(),
    );
    let mut member2 = Permutation3::new(
        CharacterName::vec(),
        WeaponName::vec(),
        ArtifactName::vec(),
    );
    for (character_name1, weapon_name1, artifact_name1) in member1.iter() {
        let (cr1, ca1) = characters1.find(&character_name1);
        let (wr1, wa1) = weapons1.find(&weapon_name1);
        let (ar1, aa1) = artifacts1.find(&artifact_name1);
        if !combination_filter_attacker(&cr1, &wr1, &ar1, args) {
            member1.back((character_name1, weapon_name1, artifact_name1));
            continue;
        }
        if physical_infusion.contains(&cr1.name) {
            ar1.infuse_goblet(&Vision::Physical);
        } else {
            ar1.infuse_goblet(&cr1.vision);
        }

        let mut items: Vec<Recorder> = Vec::new();
        for (character_name2, weapon_name2, artifact_name2) in member2.iter() {
            let (cr2, ca2) = characters2.find(&character_name2);
            let (wr2, wa2) = weapons2.find(&weapon_name2);
            let (ar2, aa2) = artifacts2.find(&artifact_name2);
            // identical characters cannot be on the same field.
            if cr1.name == cr2.name ||
               !combination_filter_supporter(&cr2, &wr2, &ar2, args) {
                member2.back((character_name2, weapon_name2, artifact_name2));
                continue;
            }
            if physical_infusion.contains(&cr2.name) {
                ar2.infuse_goblet(&Vision::Physical);
            } else {
                ar2.infuse_goblet(&cr2.vision);
            }
            let mut members = vec![
                FieldCharacterData::new(&mut timers0, ca1, wa1, aa1, CharacterData::new(idx0, &cr1, &wr1, &ar1)),
                FieldCharacterData::new(&mut timers1, ca2, wa2, aa2, CharacterData::new(idx1, &cr2, &wr2, &ar2)),
            ];
            let rc = main_loop(&mut members, args.unit_time, args.simulation_time);
            items.push(rc);

            // destruct objects
            for m in members.iter_mut() {
                m.fc.reset();
            }
            ar1.dry_goblet();
            ar2.dry_goblet();
            member2.back((character_name2, weapon_name2, artifact_name2));
        }
        member2.reset();
        member1.back((character_name1, weapon_name1, artifact_name1));
        tx.send(items).unwrap();
    }
}

fn permu3(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
    let idx = FieldCharacterIndex(0);
    let mut timers = Box::new(CharacterTimersBuilder::new().build());
    let mut characters = AllCharacters::new(idx);
    let mut weapons = AllWeapons::new(idx);
    let mut artifacts = AllArtifacts::new();
    let input_characters: Vec<CharacterName> = CharacterName::vec().drain(start..end).collect();
    // TODO
    let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];
    let mut member1 = Permutation3::new(
        input_characters,
        WeaponName::vec(),
        ArtifactName::vec(),
    );
    // let mut items: Vec<Recorder> = Vec::with_capacity(member1.size());
    let mut items: Vec<Recorder> = Vec::new();
    for (character_name, weapon_name, artifact_name) in member1.iter() {
        let (cr1, ca1) = characters.find(&character_name);
        let (wr1, wa1) = weapons.find(&weapon_name);
        let (ar1, aa1) = artifacts.find(&artifact_name);
        if !combination_filter_attacker(&cr1, &wr1, &ar1, args) {
            member1.back((character_name, weapon_name, artifact_name));
            continue;
        }
        if physical_infusion.contains(&cr1.name) {
            ar1.infuse_goblet(&Vision::Physical);
        } else {
            ar1.infuse_goblet(&cr1.vision);
        }
        let mut members = vec![
            FieldCharacterData::new(&mut timers, ca1, wa1, aa1, CharacterData::new(idx, &cr1, &wr1, &ar1))
        ];
        let rc = main_loop(&mut members, args.unit_time, args.simulation_time);
        items.push(rc);

        // destruct objects
        for m in members.iter_mut() {
            m.fc.reset();
        }
        ar1.dry_goblet();
        member1.back((character_name, weapon_name, artifact_name));
    }
    tx.send(items).unwrap();
}

fn debugging(args: &Args, debug_args: Vec<String>) -> () {
    let idx = FieldCharacterIndex(0);
    let mut timers = Box::new(CharacterTimersBuilder::new().build());
    let mut characters = AllCharacters::new(idx);
    let mut weapons = AllWeapons::new(idx);
    let mut artifacts = AllArtifacts::new();
    let mut character_name = MaybeUninit::<CharacterName>::uninit();
    let mut weapon_name = MaybeUninit::<WeaponName>::uninit();
    let mut artifact_name = MaybeUninit::<ArtifactName>::uninit();
    for i in 0..3 {
        match i {
            0 => unsafe { character_name.as_mut_ptr().write(CharacterName::from(debug_args[0].as_str())) },
            1 => unsafe { weapon_name.as_mut_ptr().write(WeaponName::from(debug_args[1].as_str())) },
            2 => unsafe { artifact_name.as_mut_ptr().write(ArtifactName::from(debug_args[2].as_str())) },
            _ => (),
        }
    };
    let (cr1, ca1) = characters.find(unsafe { &character_name.assume_init() });
    let (wr1, wa1) = weapons.find(unsafe { &weapon_name.assume_init() });
    let (ar1, aa1) = artifacts.find(unsafe { &artifact_name.assume_init() });
    let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];
    if physical_infusion.contains(&cr1.name) {
        ar1.infuse_goblet(&Vision::Physical);
    } else {
        ar1.infuse_goblet(&cr1.vision);
    }
    let mut members = vec![
        FieldCharacterData::new(&mut timers, ca1, wa1, aa1, CharacterData::new(idx, &cr1, &wr1, &ar1))
    ];
    println!("debugging: {:?} {:?} {:?}", cr1.name, wr1.name, ar1.name);
    main_loop(&mut members, args.unit_time, args.simulation_time);
}

fn start_and_wait() -> Result<(), Box<dyn Error + 'static>> {
    let mut debug_args: Vec<String> = Vec::new();
    let args = Args::parse(&mut env::args(), &mut debug_args)?;
    if debug_args.len() > 0 {
        return Ok(debugging(&args, debug_args));
    }
    let num_cpu = 3;
    let character_size = CharacterName::vec().len();
    let chunk_size = character_size / num_cpu;
    let (tx, rx) = mpsc::channel();
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
    if let Err(err) = start_and_wait() {
        println!("{:?}", err);
        process::exit(1);
    };
}
