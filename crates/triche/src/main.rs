use clap::{builder::ValueRange, Arg, ArgAction, Command};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX: usize = 80;
const MOTS_PAR_LIGNE: usize = 8;

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
        .version("0.2.1")
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
            Arg::new("Jaune")
                .help("position de 2 lettres jaunes identiques sur la même rangée.  Ex: e1 e3")
                .short('J')
                .long("Jaune")
                .num_args(ValueRange::new(2..=2))
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
        None => Vec::new(),
    };
    let mut jaunes = match matches.try_get_many::<(char, usize)>("jaune")? {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    let mut jaunes2 = match matches.try_get_many::<(char, usize)>("Jaune")? {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    let mut noires = match matches.try_get_many::<char>("noire")? {
        Some(values) => values.collect(),
        None => Vec::new(),
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

    // Valider que les 2 lettres sont identiques
    if !jaunes2.is_empty() {
        if jaunes2[0].0 == jaunes2[1].0 {
            jaunes.extend_from_slice(&jaunes2);
        } else {
            jaunes2 = Vec::new();
        }
    }

    // Éliminer les noires qui sont aussi des jaunes ou des vertes
    noires = noires
        .iter()
        .filter_map(|&n| match jaunes.iter().find(|j| j.0 == *n) {
            Some(_) => None,
            None => Some(n),
        })
        .filter_map(|n| match vertes.iter().find(|v| v.0 == *n) {
            Some(_) => None,
            None => Some(n),
        })
        .collect();

    let mut filtres: Vec<Box<dyn Fn(&[char; 5]) -> bool>> = Vec::new();

    // Conserver les mots ayant les lettres vertes à la position indiquée
    for v in vertes {
        let filtre = |mot: &[char; 5]| mot[v.1] == v.0;
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant les lettres jaunes à une position autre que la position indiquée
    for j in jaunes {
        let filtre = |mot: &[char; 5]| {
            if mot[j.1] != j.0 {
                match (0..j.1).chain(j.1 + 1..5).find(|&i| mot[i] == j.0) {
                    Some(_) => true,
                    None => false,
                }
            } else {
                false
            }
        };
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant 2 lettres jaunes identiques (sur une même rangée) à une position autre que la position indiquée
    if !jaunes2.is_empty() {
        let filtre = |mot: &[char; 5]| {
            let mut mot = mot.clone();
            let mut trouvées = 0;
            for j in &jaunes2 {
                if mot[j.1] != j.0 {
                    if let None = (0..j.1).chain(j.1 + 1..5).find(|&i| {
                        if mot[i] == j.0 {
                            mot[i] = ' ';
                            trouvées += 1;
                            true
                        } else {
                            false
                        }
                    }) {
                        break;
                    }
                } else {
                    break;
                }
            }
            trouvées == 2
        };
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ne contenant pas une lettre noire
    for n in noires {
        let filtre = |mot: &[char; 5]| match mot.iter().find(|&&l| l == *n) {
            Some(_) => false,
            None => true,
        };
        filtres.push(Box::new(filtre));
    }

    let mut fichier = env::current_exe()?;
    fichier.set_file_name("words_alpha.txt");
    let fichier = File::open(fichier)?;
    let fichier = BufReader::new(fichier);

    let passe = Box::new(|_: &[char; 5]| true) as Box<dyn Fn(&[char; 5]) -> bool>;
    let mut filtres = filtres.into_iter();
    let filtre = filtres.next().unwrap_or(passe);

    let mut mots: Vec<[char; 5]> = Vec::new();

    for mot in fichier.lines() {
        match mot {
            Ok(mot) if mot.len() == 5 => {
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

    for filtre in filtres {
        let mut filtrés: Vec<[char; 5]> = Vec::new();
        for mot in mots {
            if filtre(&mot) {
                filtrés.push(mot)
            }
        }
        mots = filtrés;
    }

    let mut mpl: usize = 0;
    let mut newline = false;
    for mot in mots.iter().take(MAX) {
        let mot = String::from_iter(mot);
        if newline {
            print!("\n");
            newline = false;
        }
        print!("{}  ", mot);
        mpl += 1;
        if mpl == MOTS_PAR_LIGNE {
            newline = true;
            mpl = 0;
        }
    }

    if mots.len() > MAX {
        print!("\nLes {} premiers mots de la sélection ({}) sont affichés", MAX, mots.len())
    }

    Ok(())
}
