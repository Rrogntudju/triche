use clap::{builder::ValueRange, Arg, ArgAction, Command};
use std::error::Error;
use std::fmt;
use std::io::{BufRead, Cursor};

const ALPHA5: &str = include_str!("./words_alpha5.txt");
const MAX: usize = 80;
const MOTS_PAR_LIGNE: usize = 8;

#[derive(Clone, Default, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Lettre(char, usize);

impl fmt::Display for Lettre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1 + 1)
    }
}

fn valide_position(arg: &str) -> Result<Lettre, String> {
    if arg.len() == 2 {
        match valide_lettre(arg) {
            Ok(c) => {
                let n = arg.chars().nth(1).unwrap();
                if "12345".contains(n) {
                    Ok(Lettre(c, n.to_string().parse::<usize>().unwrap() - 1))
                } else {
                    Err("la position de la lettre n'est pas 1-5".to_owned())
                }
            }
            Err(e) => Err(e),
        }
    } else {
        Err("position invalide".to_owned())
    }
}

fn valide_lettre(arg: &str) -> Result<char, String> {
    let c = arg.chars().next().unwrap();
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err("la lettre n'est pas a-z".to_owned())
    }
}

fn filtre_doublons<T>(liste: &mut Vec<&T>)
where
    T: Ord + PartialEq + Default + Copy,
{
    liste.sort();
    let mut prec = T::default();
    liste.retain(|&e| {
        if e == &prec {
            false
        } else {
            prec = *e;
            true
        }
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("triche")
        .version("1.3.1")
        .arg(
            Arg::new("verte")
                .help("position des lettres correctes. Ex: l1 i2 l3 a4 c5")
                .short('v')
                .long("verte")
                .num_args(ValueRange::new(1..=30))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .arg(
            Arg::new("Verte")
                .help("position d'une lettre correcte et d'une lettre présente identique sur la même rangée. Ex: n3 n2")
                .short('V')
                .long("Verte")
                .num_args(ValueRange::new(2..=2))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .arg(
            Arg::new("jaune")
                .help("position des lettres présentes.  Ex: i1 a3")
                .short('j')
                .long("jaune")
                .num_args(ValueRange::new(1..=30))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .arg(
            Arg::new("Jaune")
                .help("position de 2 lettres présentes identiques sur la même rangée.  Ex: l2 l5")
                .short('J')
                .long("Jaune")
                .num_args(ValueRange::new(2..=2))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .arg(
            Arg::new("noire")
                .help("lettres absentes à toutes les positions.  Ex: w t f")
                .short('n')
                .long("noire")
                .num_args(ValueRange::new(1..=30))
                .action(ArgAction::Append)
                .value_parser(valide_lettre),
        )
        .arg(
            Arg::new("Noire")
                .help("position des lettres absentes.  Ex: w3 t4 f5")
                .short('N')
                .long("Noire")
                .num_args(ValueRange::new(1..=30))
                .action(ArgAction::Append)
                .value_parser(valide_position),
        )
        .get_matches();

    let mut vertes = match matches.try_get_many::<Lettre>("verte")? {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    let vertes2 = match matches.try_get_many::<Lettre>("Verte")? {
        Some(values) => {
            let vertes2: Vec<&Lettre> = values.collect();
            // Valider que les 2 lettres sont identiques
            if vertes2[0].0 == vertes2[1].0 {
                vertes2
            } else {
                return Err(format!("Les lettres dans {} et {} doivent être identiques", vertes2[0], vertes2[1]).into());
            }
        }
        None => Vec::new(),
    };

    let mut jaunes = match matches.try_get_many::<Lettre>("jaune")? {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    let jaunes2 = match matches.try_get_many::<Lettre>("Jaune")? {
        Some(values) => {
            let jaunes2: Vec<&Lettre> = values.collect();
            // Valider que les 2 lettres sont identiques
            if jaunes2[0].0 == jaunes2[1].0 {
                jaunes2
            } else {
                return Err(format!("Les lettres dans {} et {} doivent être identiques", jaunes2[0], jaunes2[1]).into());
            }
        }
        None => Vec::new(),
    };

    let mut noires = match matches.try_get_many::<char>("noire")? {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    let mut noires2 = match matches.try_get_many::<Lettre>("Noire")? {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    // Éliminer les doublons
    filtre_doublons(&mut vertes);
    filtre_doublons(&mut jaunes);
    filtre_doublons(&mut noires);
    filtre_doublons(&mut noires2);

    // Éliminer les lettres noires qui sont aussi des jaunes, des vertes ou des noires positionnées
    noires = noires
        .into_iter()
        .filter(|&n| jaunes.iter().all(|j| j.0 != *n))
        .filter(|&n| jaunes2.iter().all(|j| j.0 != *n))
        .filter(|&n| vertes.iter().all(|v| v.0 != *n))
        .filter(|&n| vertes2.iter().all(|v| v.0 != *n))
        .filter(|&n| noires2.iter().all(|v| v.0 != *n))
        .collect();

    // Éliminer les lettres noires positionnées qui sont aussi des vertes
    noires2.retain(|&n| vertes.iter().all(|&v| v != n));

    let mut filtres: Vec<Box<dyn Fn(&[char; 5]) -> bool>> = Vec::new();

    // Conserver les mots ayant les lettres vertes à la position indiquée
    for v in vertes {
        let filtre = |mot: &[char; 5]| mot[v.1] == v.0;
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ne contenant pas une lettre noire
    for n in noires {
        let filtre = |mot: &[char; 5]| mot.iter().all(|&l| l != *n);
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots n'ayant pas une lettre noire à la position indiquée
    for n in noires2 {
        let filtre = |mot: &[char; 5]| mot[n.1] != n.0;
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant les lettres jaunes à une position autre que la position indiquée
    for j in jaunes {
        let filtre = |mot: &[char; 5]| {
            if mot[j.1] != j.0 {
                (0..j.1).chain(j.1 + 1..5).any(|i| mot[i] == j.0)
            } else {
                false
            }
        };
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant 2 lettres jaunes identiques à une position autre que la position indiquée
    if !jaunes2.is_empty() {
        let filtre = |mot: &[char; 5]| {
            let mut mot = *mot;
            let mut trouvées = 0;
            for j in &jaunes2 {
                if mot[j.1] != j.0 {
                    (0..j.1).chain(j.1 + 1..5).any(|i| {
                        if mot[i] == j.0 {
                            mot[i] = ' ';
                            trouvées += 1;
                            true
                        } else {
                            false
                        }
                    });
                } else {
                    break;
                }
            }
            trouvées == 2
        };
        filtres.push(Box::new(filtre));
    }

    // Conserver les mots ayant une lettre jaune à une position autre que la position d'une lettre verte identique
    if !vertes2.is_empty() {
        let filtre = |mot: &[char; 5]| {
            let v = &vertes2[0];
            let j = &vertes2[1];
            if mot[j.1] != j.0 {
                (0..j.1).chain(j.1 + 1..5).any(|i| mot[i] == j.0 && i != v.1)
            } else {
                false
            }
        };
        filtres.push(Box::new(filtre));
    }

    let mut mots: Vec<[char; 5]> = Cursor::new(ALPHA5)
        .lines()
        .map_while(Result::ok)
        .map(|m| {
            let mut mot = [' '; 5];
            m.char_indices().for_each(|(i, c)| mot[i] = c);
            mot
        })
        .collect();

    for filtre in filtres {
        let filtrés: Vec<[char; 5]> = mots.into_iter().filter(|m| filtre(m)).collect();
        mots = filtrés;
    }

    let mut mpl: usize = 0;
    let mut newline = false;
    for mot in mots.iter().take(MAX) {
        let mot = String::from_iter(mot);
        if newline {
            println!();
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
    println!("");

    Ok(())
}
