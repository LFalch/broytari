use std::collections::{HashSet, HashMap};

pub mod file;
pub mod run;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phone {
    inner: Box<str>,
}

impl Phone {
    pub fn new<S: Into<Box<str>>>(s: S) -> Self {
        Phone { inner: s.into() }
    }
}

impl<S: Into<Box<str>>> From<S> for Phone {
    fn from(s: S) -> Self {
        Phone::new(s)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Category {
    pub inner: HashSet<Phone>,
}

impl Category {
    pub fn add<P: Into<Phone>>(&mut self, phone: P) -> &mut Self {
        self.inner.insert(phone.into());
        self
    }
    pub fn remove(&mut self, phone: &Phone) {
        self.inner.remove(&phone);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Feature {
    pub plus_set: HashSet<Phone>,
    pub minus_set: HashSet<Phone>,
}

impl Feature {
    pub fn plus<P: Into<Phone>>(&mut self, phone: P) -> &mut Self {
        let phone = phone.into();
        self.minus_set.remove(&phone);
        self.plus_set.insert(phone);
        self
    }
    pub fn minus<P: Into<Phone>>(&mut self, phone: P) -> &mut Self {
        let phone = phone.into();
        self.plus_set.remove(&phone);
        self.minus_set.insert(phone);
        self
    }
    pub fn zero(&mut self, phone: &Phone) {
        self.plus_set.remove(phone);
        self.minus_set.remove(phone);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Phonology {
    pub categories: HashMap<String, Category>,
    pub features: HashMap<String, Feature>,
}

impl Phonology {
    pub fn add_category<S: ToString>(&mut self, s: S) -> &mut Category {
        let s = s.to_string();
        self.categories.remove(&s);
        self.categories.entry(s).or_insert(Category::default())
    }
    pub fn add_feature<S: ToString>(&mut self, s: S) -> &mut Feature {
        let s = s.to_string();
        self.features.remove(&s);
        self.features.entry(s).or_insert(Feature::default())
    }
    pub fn clear_phone(&mut self, p: &Phone) {
        for cat in self.categories.values_mut() {
            cat.remove(p);
        }
        for feature in self.features.values_mut() {
            feature.zero(p);
        }
    }
    pub fn print(&self) {
        let phones: HashSet<_> = self.categories
            .values()
            .flat_map(|c| c.inner.iter())
            .chain(
                self.features
                    .values()
                    .flat_map(|f| f.plus_set.iter().chain(f.minus_set.iter()))
            )
            .collect();

        for phone in phones {
            print!("{}:", phone.inner);
            for (name, category) in &self.categories {
                if category.inner.contains(phone) {
                    print!(" [{}]", name);
                }
            }
            for (name, feature) in &self.features {
                if feature.minus_set.contains(phone) {
                    print!(" [-{}]", name);
                } else if feature.plus_set.contains(phone) {
                    print!(" [+{}]", name);
                }
            }
            println!();
        }
    }
}
