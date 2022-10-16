use clap::{builder::ValueRange, Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn valide_position(arg: &str) -> Result<(char, usize), String> {
    if arg.len() == 2 {
        match valide_lettre(arg) {
            Ok(c) => {
                let n = arg.chars().nth(1).unwrap();
                if "12345".contains(n) {
                    Ok((c, n.to_string().parse::<usize>().unwrap() - 1))
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

    let vertes = matches.get_many("verte").unwrap_or_default().collect::<Vec<&(char, usize)>>();
    let jaunes = matches.get_many("jaune").unwrap_or_default().collect::<Vec<&(char, usize)>>();
    let noires = matches.get_many("noire").unwrap_or_default().collect::<Vec<&char>>();
    let mut filtres: Vec<Box<dyn FnMut(&[char; 5]) -> bool>> = Vec::new();

    if !noires.is_empty() {
        let noires_et_jaunes: Vec<(char,usize)> = noires.iter().filter_map(|&&n| {
            match jaunes.iter().find(|&&&j| j.0 == n) {
                Some(&&j) => Some((n, j.1)),
                None => None
            }
        }).collect();
    }

    if !jaunes.is_empty() {
        let filtre = |mot: &[char; 5]| {
            let trouvées = jaunes.iter().fold(0, |mut acc, &&j| {
                if mot[j.1] != j.0 {
                    if let Some(_) = (0..j.1).find(|&i| mot[i] == j.0) {
                        acc+=1
                    } else if let Some(_) = (j.1 + 1..=5).find(|&i| mot[i] == j.0) {
                        acc+=1
                    }
                }
                acc
            });
            trouvées == jaunes.len()
        };
        filtres.push(Box::new(filtre));
    }

    if !vertes.is_empty() {
        let filtre = |mot: &[char; 5]| {
            let trouvées = vertes.iter().fold(0, |mut acc, &&v| {
                if mot[v.1] == v.0 {
                    acc += 1;
                }
                acc
            });
            trouvées == vertes.len()
        };
        filtres.push(Box::new(filtre));
    }

    let mut filtre = match filtres.pop() {
        Some(filtre) => filtre,
        _ => return Err("Pas de filtre".into()),
    };
    let fichier = File::open("english-words.txt")?;
    let fichier = BufReader::new(fichier);
    let mut mots: Vec<[char; 5]> = Vec::new();

    for mot in fichier.lines() {
        match mot {
            Ok(mot) if mot.len() == 5 => {
                let mot = mot.to_ascii_lowercase();
                let mut m = [' '; 5];
                mot.char_indices().for_each(|(i, c)| m[i] = c);
                if filtre(&m) {
                    mots.push(m);
                }
            }
            Ok(_) => continue,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
