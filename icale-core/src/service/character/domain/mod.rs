use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Gender {
    Unknown,
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub id: String,
    pub gender: Gender,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Skin {
    Unknown,
    White,
    Black,
    Yellow,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Appearance {
    pub skin: Skin,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    id: String,
    pub profile: Profile,
    pub appearance: Appearance,
}

impl Character {
    pub fn new(profile: Profile, appearance: Appearance) -> Self {
        Self::new_with_id(profile, appearance, Uuid::new_v4().to_hyphenated().to_string())
    }
    pub fn new_with_id(profile: Profile, appearance: Appearance, id: String) -> Self {
        Self {
            id: id,
            appearance: appearance,
            profile: profile,
        }
    }
    pub fn get_id<'a>(&'a self) -> &'a String {
        &self.id
    }

    pub fn get_profile<'a>(&'a self) -> &'a Profile {
        &self.profile
    }

    pub fn change_profile(mut self, to: Profile) -> Self {
        self.profile = to;
        self
    }

    pub fn get_appearance<'a>(&'a self) -> &'a Appearance {
        &self.appearance
    }
    pub fn change_appearance(mut self, to: Appearance) -> Self {
        self.appearance = to;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let profile = Profile {
            id: String::from("euiko"),
            gender: Gender::Male,
            name: String::from("Candra Kharista"),
        };

        let appearance = Appearance { skin: Skin::Yellow };

        let euiko = Character::new(profile, appearance);

        let p = euiko.get_profile();
        let a = euiko.get_appearance();

        assert_eq!(p.id, "euiko");
        assert_eq!(a.skin, Skin::Yellow);
    }
}
