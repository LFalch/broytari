use std::{fs::File, path::Path, io::{self, BufRead, BufReader}, fmt::{Display, self}};

use super::Phone;

pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<Line>> {
    let file = BufReader::new(File::open(path)?);

    let mut lines = Vec::new();

    for (line, line_no) in file.lines().zip(1..) {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        } else if line.starts_with("%") {
            lines.push(Line::Directive(line_no, parse_directive(&line[1..]).unwrap()));
        } else if line.starts_with("=#") {
            lines.push(Line::Stage(line_no, line[2..].trim_start().to_owned()));
        } else if line.starts_with("=") {
            let (phone, qualifiers) = parse_new_phone(line[1..].trim_start()).unwrap();
            lines.push(Line::Phone(line_no, phone, qualifiers));
        } else if line.starts_with("//") {
            // comment
            continue;
        } else {
            lines.push(Line::Change(line_no, parse_sound_change(line).unwrap()));
        }
    }

    Ok(lines)
}

fn parse_directive(s: &str) -> Option<Directive> {
    let mut terms = s.split_whitespace();
    match terms.next()? {
        "cat" | "category" => {
            let name = terms.next()?.to_owned();
            let mut phones = Vec::new();
            let colon = terms.next();
            if colon == Some(":") {
                for term in terms {
                    phones.push(Phone::new(term));
                }
            } else if colon.is_some() { return None } 

            Some(Directive::Category(name, phones))
        }
        "feat" | "feature" => {
            let name = terms.next()?.to_owned();
            let mut plus_set = Vec::new();
            let mut minus_set = Vec::new();
            let colon = terms.next();
            if colon == Some(":") {
                for term in terms {
                    let (sign, sound) = term.split_at(1);
                    let phone = Phone::new(sound);
                    match sign {
                        "-" => minus_set.push(phone),
                        "+" => plus_set.push(phone),
                        _ => return None,
                    }
                }
            } else if colon.is_some() { return None } 

            Some(Directive::Feature(name, plus_set, minus_set))
        }
        "sym" | "symbol" => {
            let mut cs = terms.next()?.chars();
            let c = cs.next()?;
            if cs.next().is_some() {
                return None;
            }
            let mut phones = Vec::new();
            let mut qualifiers = Vec::new();
            let colon = terms.next();
            if colon == Some(":") {
                for term in terms {
                    if term.starts_with("+") {
                        qualifiers.push(PhoneQualifier::Plus(term[1..].to_owned()));
                    } else if term.starts_with("-") {
                        qualifiers.push(PhoneQualifier::Minus(term[1..].to_owned()));
                    } else if term.starts_with("0") {
                        qualifiers.push(PhoneQualifier::Zero(term[1..].to_owned()));
                    } else if term.starts_with("'") {
                        phones.push(Phone::new(&term[1..term.len()-1]));
                    } else {
                        qualifiers.push(PhoneQualifier::Cat(term.to_owned()));
                    }
                }
            } else if colon.is_some() { return None } 

            Some(Directive::Symbol(c, phones, qualifiers))
        }
        "print" => {
            println!("{:?}", terms);
            None
        }
        _ => None,
    }
}

#[derive(Debug)]
pub enum Line {
    Directive(u32, Directive),
    Stage(u32, String),
    Phone(u32, Phone, Vec<PhoneQualifier>),
    Change(u32, SoundChange),
}

impl Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Line::*;
        match self {
            Directive(_, dir) => {
                use self::Directive::*;
                match dir {
                    Category(cat, phones) => {
                        write!(f, "%category {cat} :")?;
                        for phone in phones {
                            write!(f, " {}", phone.inner)?;
                        }
                        Ok(())
                    }
                    Feature(name, plus, minus) => {
                        write!(f, "%feature {name} :")?;
                        for phone in plus {
                            write!(f, " +{}", phone.inner)?;
                        }
                        for phone in minus {
                            write!(f, " -{}", phone.inner)?;
                        }
                        Ok(())
                    }
                    Symbol(c, phones, qualifiers) => {
                        write!(f, "%symbol {c} :")?;
                        for phone in phones {
                            write!(f, " '{}'", phone.inner)?;
                        }
                        for qualifier in qualifiers {
                            match qualifier {
                                PhoneQualifier::Cat(s) => write!(f, " {s}")?,
                                PhoneQualifier::Minus(s) => write!(f, " -{s}")?,
                                PhoneQualifier::Plus(s) => write!(f, " +{s}")?,
                                PhoneQualifier::Zero(s) => write!(f, " 0{s}")?,
                            }
                        }
                        Ok(())
                    }
                }
            }
            Stage(_, name) => write!(f, "#= {name}"),
            Phone(_, phone, qualifiers) =>  {
                write!(f, "= {} :", phone.inner)?;
                for qualifier in qualifiers {
                    match qualifier {
                        PhoneQualifier::Cat(s) => write!(f, " {s}")?,
                        PhoneQualifier::Minus(s) => write!(f, " -{s}")?,
                        PhoneQualifier::Plus(s) => write!(f, " +{s}")?,
                        PhoneQualifier::Zero(s) => write!(f, " 0{s}")?,
                    }
                }
                Ok(())
            },
            Change(_, sc) => {
                write!(f, "{} -> {}", sc.from, sc.to)?;
                for senv in &sc.special_environments {
                    write!(f, " / {}", senv)?;
                }
                for env in &sc.environments {
                    let exc = if env.exception { "!" } else { "" };
                    let start = if env.from_start { "#" } else { "" };
                    let end = if env.to_end { "#" } else { "" };
                    write!(f, " /{} {}{}_{}{} ", exc, start, env.after, env.before, end)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum Directive {
    Category(String, Vec<Phone>),
    Feature(String, Vec<Phone>, Vec<Phone>),
    Symbol(char, Vec<Phone>, Vec<PhoneQualifier>)
}

fn parse_new_phone(s: &str) -> Option<(Phone, Vec<PhoneQualifier>)> {
    let mut terms = s.split_whitespace();
    let phone = Phone::new(terms.next()?);
    let mut qualifiers = Vec::new();
    let colon = terms.next();
    if colon == Some(":") {
        for term in terms {
            if term.starts_with("+") {
                qualifiers.push(PhoneQualifier::Plus(term[1..].to_owned()));
            } else if term.starts_with("-") {
                qualifiers.push(PhoneQualifier::Minus(term[1..].to_owned()));
            } else if term.starts_with("0") {
                qualifiers.push(PhoneQualifier::Zero(term[1..].to_owned()));
            } else {
                qualifiers.push(PhoneQualifier::Cat(term.to_owned()));
            }
        }
    } else if colon.is_some() { return None } 

    Some((phone, qualifiers))
}

#[derive(Debug)]
pub enum PhoneQualifier {
    Plus(String),
    Minus(String),
    Zero(String),
    Cat(String),
}

fn parse_sound_change(s: &str) -> Option<SoundChange> {
    let arrow_index = s.find('>')?;
    let before_s = s[..arrow_index].trim();
    let after_s = &s[arrow_index+1..];
    let environment_index = after_s.find('/').unwrap_or(after_s.len());
    let (after_s, environment_s) = after_s.split_at(environment_index);
    let after_s = after_s.trim();

    let (environments, special_environments) = parse_environments(environment_s);

    Some(SoundChange {
        from: before_s.to_owned(),
        to: after_s.to_owned(),
        environments,
        special_environments,
    })
}

#[derive(Debug, Default)]
pub struct SoundChange {
    from: Sounds,
    to: Sounds,
    environments: Vec<Environment>,
    special_environments: Vec<String>,
}

pub type Sounds = String;

fn parse_environments(env_s: &str) -> (Vec<Environment>, Vec<String>) {
    let mut envs = Vec::new();
    let mut specials = Vec::new();

    for env in env_s.split('/').skip(1).map(str::trim) {
        if let Some((mut before, mut after)) = env.split_once("_") {
            let exception = before.starts_with("!");
            if exception {
                before = before[1..].trim_start();
            }
            let from_start = before.starts_with("#");
            if from_start {
                before = &before[1..];
            }
            let to_end = after.ends_with("#");
            if to_end {
                after = &after[1..];
            }

            envs.push(Environment { exception, from_start, to_end,
                before: before.to_owned(),
                after: after.to_owned(),
            });
        } else {
            specials.push(env.to_owned());
        }
    }

    (envs, specials)
}

#[derive(Debug, Default)]
pub struct Environment {
    exception: bool,
    from_start: bool,
    before: Sounds,
    after: Sounds,
    to_end: bool,
}