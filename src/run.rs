use crate::{file::{Line, Directive, PhoneQualifier}, Phonology};


pub fn run(lines: &[Line], words: &mut [String], stage: Option<&str>) -> Phonology {
    let mut phonology = Phonology::default();

    let mut stage_found = stage.is_none();

    for line in lines {
        match line {
            Line::Stage(s) => {
                if Some(&**s) == stage {
                    stage_found = true;
                    break;
                }
            }
            Line::Directive(dir) => match dir {
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
                    eprintln!("new symbol {c}");
                },
            }
            Line::Change(sc) => eprintln!("{:?}", sc),
            Line::Phone(phone, qualifiers) => {
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
