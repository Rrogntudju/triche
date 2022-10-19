use clap::{builder::ValueRange, Arg, ArgAction, Command};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX:usize = 70;

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
                .num_args(ValueRange::new(1..=26))
                .action(ArgAction::Append)
                .value_parser(valide_lettre),
        )
        .get_matches();

    let vertes = match matches.try_get_many::<(char, usize)>("verte")? {
        Some(values) => values.collect(),
        None => Vec::new()
    };
    let jaunes =  match matches.try_get_many::<(char, usize)>("jaune")? {
        Some(values) => values.collect(),
        None => Vec::new()
    };
    let mut noires =  match matches.try_get_many::<char>("noire")? {
        Some(values) => values.collect(),
        None => Vec::new()
    };

    // Éliminer les doublons
    noires.sort();
    let mut prec = ' ';
    noires = noires
        .iter()
        .filter_map(|&l| {
            if l == &prec {
                None
            } else {
                prec = *l;
                Some(l)
            }
        })
        .collect();

    let mut filtres: Vec<Box<dyn FnMut(&[char; 5]) -> bool>> = Vec::new();

    // Éliminer les noires qui sont aussi des jaunes
    if !noires.is_empty() && !jaunes.is_empty() {
        noires = noires
            .iter()
            .filter_map(|&n| match jaunes.iter().find(|j| j.0 == *n) {
                Some(_) => None,
                None => Some(n),
            })
            .collect();
    }

    // Éliminer les noires qui sont aussi des vertes
    if !noires.is_empty() && !vertes.is_empty() {
        noires = noires
            .iter()
            .filter_map(|&n| match vertes.iter().find(|v| v.0 == *n) {
                Some(_) => None,
                None => Some(n),
            })
            .collect();
    }

    // Éliminer les mots contenant une lettre noire
    if !noires.is_empty() {
        let filtre = |mot: &[char; 5]| match mot.iter().find(|l| match noires.iter().find(|&n| n == l) {
            Some(_) => true,
            None => false,
        }) {
            Some(_) => false,
            None => true,
        };
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant une lettre jaune à une position autre que la position indiquée
    if !jaunes.is_empty() {
        let filtre = |mot: &[char; 5]| {
            let trouvées = jaunes.iter().fold(0, |mut acc, &&j| {
                if mot[j.1] != j.0 {
                    if let Some(_) = (0..j.1).find(|&i| mot[i] == j.0) {
                        acc += 1
                    } else if let Some(_) = (j.1 + 1..5).find(|&i| mot[i] == j.0) {
                        acc += 1
                    }
                }
                acc
            });
            trouvées == jaunes.len()
        };
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant une lettre verte à la position indiquée
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

    let mut fichier = env::current_exe()?;
    fichier.set_file_name("english-words.txt");
    let fichier = File::open(fichier)?;
    let fichier = BufReader::new(fichier);
    let mut mots: Vec<[char; 5]> = Vec::new();

    for mot in fichier.lines() {
        match mot {
            Ok(mot) if mot.len() == 5 => {
                if let None = mot.find(|l: char| !l.is_ascii_alphabetic()) {
                    let mot = mot.to_ascii_lowercase();
                    let mut m = [' '; 5];
                    mot.char_indices().for_each(|(i, c)| m[i] = c);
                    if filtre(&m) {
                        mots.push(m);
                    }
                }
            }
            Ok(_) => continue,
            Err(e) => return Err(e.into()),
        }
    }

    for mut filtre in filtres {
        let mut filtrés: Vec<[char; 5]> = Vec::new();
        for mot in mots {
            if filtre(&mot) {
                filtrés.push(mot)
            }
        }
        mots = filtrés;
    }

    let mut nb: usize = 0;
    let mut newline = false;
    for mot in mots.iter().take(MAX) {
        let mot = String::from_iter(mot);
        if nb == 0 {
            if newline {
                print!("\n");
                newline = false;
            }
            print!("{}", mot);
        } else {
            print!("  {}", mot);
        }
        nb += 1;
        if nb == 7 {
            newline = true;
            nb = 0;
        }
    }

    if mots.len() > MAX {
        print!("\nSeulement les {} premiers mots de la sélection ({}) sont affichés", MAX, mots.len())
    }

    Ok(())
}
