use bevy::prelude::{Component, Resource};

use crate::Enumerated;

// A Setting is any enum which (1) has a variants() method, (2) can be Displayed, and (3) is a Component
pub trait Setting: std::fmt::Display + Component + Clone + Copy {}

#[derive(Resource, Component, Enumerated, Clone, Copy, Default, PartialEq, Eq)]
pub enum HumanMark {
    #[default]
    HumanX,
    HumanO
}

impl std::fmt::Display for HumanMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            HumanMark::HumanX => "Human X",
            HumanMark::HumanO => "Human O",
        })
    }
}

impl Setting for HumanMark {}

#[derive(Resource, Component, Enumerated, Clone, Copy, Default, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    #[default]
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        })
    }
}

impl Setting for Difficulty {}

#[derive(Resource, Component, Enumerated, Clone, Copy, Default, PartialEq, Eq)]
pub enum GameMode {
    OnePlayer,
    #[default]
    TwoPlayers,
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GameMode::OnePlayer => "One Player",
            GameMode::TwoPlayers => "Two Players",
        })
    }
}

impl Setting for GameMode {}