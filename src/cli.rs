use std::error::Error;
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
    StartEnergy,
    Truncate,
    Value(String),
}

#[derive(Debug, PartialEq)]
enum Mode {
    Simulate,
    Debugging,
}

#[derive(Debug, Copy, Clone)]
pub struct Args {
    pub n_members: usize,
    pub character_version: f32,
    pub weapon_version: f32,
    pub artifact_version: f32,
    pub unit_time: f32,
    pub simulation_time: f32,
    pub start_energy: i32,
    pub truncate: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            n_members: 1,
            character_version: 2.0,
            weapon_version: 2.0,
            artifact_version: 2.0,
            unit_time: 0.2,
            simulation_time: 20.0,
            start_energy: -1,
            truncate: false,
        }
    }
}

impl Args {
    pub fn parse<I: Iterator<Item = String>>(env_args: &mut I, debug_args: &mut Vec<String>) -> Result<Args, Box<dyn Error>> {
        use ArgToken::*;
        let mut kv: Vec<(ArgToken, ArgToken)> = Vec::new();
        // ignore the first
        env_args.next();
        let mode = if let Some(string) = env_args.next() {
            match string.as_str() {
                "simulate" => Mode::Simulate,
                "debug" => Mode::Debugging,
                _ => {
                    kv.push((Help, Help));
                    Mode::Simulate
                }
            }
        } else {
            kv.push((Help, Help));
            Mode::Simulate
        };
        if mode == Mode::Debugging {
            while let Some(ref x) = env_args.next() {
                debug_args.push(x.to_string());
            }
            return Ok(Args::default());
        }
        while let Some(a) = env_args.next() {
            match a.as_str() {
                "-h" | "--help" => kv.push((Help, Help)),
                "--n_members" => kv.push((NMembers, Help)),
                "--character_version" => kv.push((CharacterVersion, Help)),
                "--weapon_version" => kv.push((WeaponVersion, Help)),
                "--artifact_version" => kv.push((ArtifactVersion, Help)),
                "--unit_time" => kv.push((UnitTime, Help)),
                "--simulation_time" => kv.push((EmulationTime, Help)),
                "--start_energy" => kv.push((StartEnergy, Help)),
                "--truncate" => kv.push((Truncate, Help)),
                _ => if let Some((_k, v)) = kv.last_mut() {
                    *v = Value(a);
                },
            };
        };
        let mut args = Args::default();
        for item in kv.drain(..) {
            match item {
                (Help, _) => {
                    println!("dos is a party damage output simulator.

Usage:
    dos simulate [--n_members N] [--character_version N] [--weapon_version N] [--artifact_version N]

Options:
    --n_members N         : Number of field members [default: 1]
    --character_version N : characters up to the version will be simulated [default: 1.6]
    --weapon_version N    : weapons up to the version will be simulated [default: 1.6]
    --artifact_version N  : artifacts up to the version will be simulated [default: 1.6]
    --unit_time N         : frequency of character actions [default: 0.2]
    --simulation_time N   : end the simulation at N seconds [default: 20.0]
    --start_energy N      : amount of energy given to characters at the beginning of the simulation. Negative values mean full energy [default: -1]
    --truncate            : remove some results from outputs when field members are greater than 2 [default: false]");
                    process::exit(0);
                },
                (NMembers, Value(v)) => args.n_members = v.parse()?,
                (CharacterVersion, Value(v)) => args.character_version = v.parse()?,
                (WeaponVersion, Value(v)) => args.weapon_version = v.parse()?,
                (ArtifactVersion, Value(v)) => args.artifact_version = v.parse()?,
                (UnitTime, Value(v)) => args.unit_time = v.parse()?,
                (EmulationTime, Value(v)) => args.simulation_time = v.parse()?,
                (StartEnergy, Value(v)) => args.start_energy = v.parse()?,
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
        } else if self.unit_time < 0.0 || self.simulation_time < 0.0 {
            Err(Box::new(MyError::new("times should not be negative.")))
        } else {
            Ok(())
        }
    }
}
