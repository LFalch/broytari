use std::collections::HashMap;

use crate::{file::{Line, Directive, PhoneQualifier, SoundChange}, Phonology, Phone};


pub fn run(lines: &[Line], words: &mut [String], stage: Option<&str>) -> Phonology {
    let mut phonology = Phonology::default();
    let mut symbols = HashMap::new();

    let mut stage_found = stage.is_none();

    for line in lines {
        match line {
            Line::Stage(_, s) => {
                if Some(&**s) == stage {
                    stage_found = true;
                    break;
                }
            }
            Line::Directive(_, dir) => match dir {
                Directive::Category(cat, phones) => {
                    let cat = phonology.add_category(cat);
                    for phone in phones {
                        cat.add(phone.clone());
                    }
                }
                Directive::Feature(feat, p, m) => {
                    let feat = phonology.add_feature(feat);
                    for p in p {
                        feat.plus(p.clone());
                    }
                    for m in m {
                        feat.minus(m.clone());
                    }
                }
                Directive::Symbol(c, phones, qualifiers) => {
                    symbols.insert(*c, (&**phones, &**qualifiers));
                },
            }
            Line::Change(_, sc) => apply_sound_change(sc, &symbols, words),
            Line::Phone(_, phone, qualifiers) => {
                phonology.clear_phone(phone);
                for qualifier in qualifiers {
                    match qualifier {
                        PhoneQualifier::Cat(c) => {phonology.categories.get_mut(c).unwrap().add(phone.clone());}
                        PhoneQualifier::Minus(f) => {phonology.features.get_mut(f).unwrap().minus(phone.clone());}
                        PhoneQualifier::Plus(f) => {phonology.features.get_mut(f).unwrap().plus(phone.clone());}
                        PhoneQualifier::Zero(f) => {phonology.features.get_mut(f).unwrap().zero(phone);}
                    }
                }
            },
        }
    }

    if !stage_found {
        println!("WARNING: did not find stage");
    }

    phonology
}

fn apply_sound_change(sound_change: &SoundChange, symbols: &HashMap<char, (&[Phone], &[PhoneQualifier])>, words: &mut [String]) {
    eprintln!("{:?}", sound_change);
}
