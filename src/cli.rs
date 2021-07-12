use std::error::Error;
use std::env;
use std::fmt;
use std::process;

#[derive(Debug)]
pub struct MyError {
    details: String
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError { details: msg.to_string() }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
enum ArgToken {
    Help,
    NMembers,
    CharacterVersion,
    WeaponVersion,
    ArtifactVersion,
    UnitTime,
    EmulationTime,
    Truncate,
    Value(String),
}

#[derive(Debug, Copy, Clone)]
pub struct Args {
    pub n_members: usize,
    pub character_version: f32,
    pub weapon_version: f32,
    pub artifact_version: f32,
    pub unit_time: f32,
    pub emulation_time: f32,
    pub truncate: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            n_members: 1,
            character_version: 1.6,
            weapon_version: 1.6,
            artifact_version: 1.6,
            unit_time: 0.2,
            emulation_time: 20.0,
            truncate: false,
        }
    }
}

impl Args {
    pub fn parse() -> Result<Args, Box<dyn Error>> {
        use ArgToken::*;
        let mut kv: Vec<(ArgToken, ArgToken)> = Vec::new();
        for a in env::args() {
            match a.as_str() {
                "-h" | "--help" => kv.push((Help, Help)),
                "--n_members" => kv.push((NMembers, Help)),
                "--character_version" => kv.push((CharacterVersion, Help)),
                "--weapon_version" => kv.push((WeaponVersion, Help)),
                "--artifact_version" => kv.push((ArtifactVersion, Help)),
                "--unit_time" => kv.push((UnitTime, Help)),
                "--emulation_time" => kv.push((EmulationTime, Help)),
                "--truncate" => kv.push((Truncate, Help)),
                _ => if let Some((_k, v)) = kv.last_mut() {
                    *v = Value(a);
                },
            };
        }
        let mut args = Args::default();
        for item in kv.drain(..) {
            match item {
                (Help, _) => {
                    println!("Genshin is a party damage simulator.

Usage:
    genshin [--n_members N] [--character_version N] [--weapon_version N] [--artifact_version N]
            [--unit_time N] [--emulation_time N] [--truncate]

Options:
    --n_members N         : Number of field members [default: 1]
    --character_version N : characters up to the version will be simulated [default: 1.6]
    --weapon_version N    : weapons up to the version will be simulated [default: 1.6]
    --artifact_version N  : artifacts up to the version will be simulated [default: 1.6]
    --unit_time N         : frequency of character actions [default: 0.2]
    --emulation_time N    : end the simulation at N seconds [default: 20.0]
    --truncate            : remove some results from outputs when field members are greater than 2 [default: false]");
                    process::exit(0);
                },
                (NMembers, Value(v)) => args.n_members = v.parse()?,
                (CharacterVersion, Value(v)) => args.character_version = v.parse()?,
                (WeaponVersion, Value(v)) => args.weapon_version = v.parse()?,
                (ArtifactVersion, Value(v)) => args.artifact_version = v.parse()?,
                (UnitTime, Value(v)) => args.unit_time = v.parse()?,
                (EmulationTime, Value(v)) => args.emulation_time = v.parse()?,
                (Truncate, _) => args.truncate = true,
                _ => return Err(Box::new(MyError::new("arguments were not recognized."))),
            }
        }
        args.validate()?;
        Ok(args)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.n_members < 1 || 2 < self.n_members {
            Err(Box::new(MyError::new("given n_members is not supported now.")))
        } else if self.character_version < 0.0 || self.weapon_version < 0.0 || self.artifact_version < 0.0 {
            Err(Box::new(MyError::new("versions should not be negative.")))
        } else if self.unit_time < 0.0 || self.emulation_time < 0.0 {
            Err(Box::new(MyError::new("times should not be negative.")))
        } else {
            Ok(())
        }
    }
}
