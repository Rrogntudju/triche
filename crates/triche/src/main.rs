use clap::{builder::ValueRange, Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn valide_position(arg: &str) -> Result<String, String> {
    if arg.len() == 2 {
        match valide_lettre(arg) {
            Ok(c) => {
                let n = arg.chars().nth(1).unwrap();
                if "12345".contains(n) {
                    Ok(format!("{}{}", c, n))
                } else {
                    Err("la position n'est pas 1-5".to_owned())
                }
            }
            Err(e) => Err(e),
        }
    } else {
        Err("la longueur != 2".to_owned())
    }
}

fn valide_lettre(arg: &str) -> Result<char, String> {
    let c = arg.chars().nth(0).unwrap();
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err("la lettre n'est pas a-z".to_owned())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("triche")
        .version("0.1.0")
        .arg(
            Arg::new("verte")
                .help("position des lettres vertes. Ex: a1 r2 o3 s4 e5")
                .short('v')
                .long("verte")
                .num_args(ValueRange::new(1..=5))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .arg(
            Arg::new("jaune")
                .help("position des lettres jaunes.  Ex: e1 a3")
                .short('j')
                .long("jaune")
                .num_args(ValueRange::new(1..=5))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .arg(
            Arg::new("noire")
                .help("lettres noires.  Ex: r s")
                .short('n')
                .long("noire")
                .num_args(ValueRange::new(1..=5))
                .action(ArgAction::Append)
                .value_parser(valide_lettre),
        )
        .get_matches();

    let fichier = File::open("english-words.txt")?;
    let fichier = BufReader::new(fichier);
    let mut mots: Vec<String> = Vec::new();
    for mot in fichier.lines() {
        match mot {
            Ok(mot) if mot.len() == 5 => mots.push(mot),
            Ok(_) => continue,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
