use std::fmt;

#[derive(Debug, Clone)]
pub enum Expansion {
    Dominion,
    Intrigue,
    Seaside,
    Alchemy,
    Prosperity,
    CornucopiaAndGuilds,
    Hinterlands,
    DarkAges,
    Adventures,
    Empires,
    Nocturne,
    Renaissance,
    Menagerie,
    Allies,
    Plunder,
    RisingSun,
    Promos,
    Custom,
}

impl fmt::Display for Expansion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dominion => write!(f, "Dominion"),
            Self::Intrigue => write!(f, "Intrigue"),
            Self::Seaside => write!(f, "Seaside"),
            Self::Alchemy => write!(f, "Alchemy"),
            Self::Prosperity => write!(f, "Prosperity"),
            Self::CornucopiaAndGuilds => write!(f, "CornucopiaAndGuilds"),
            Self::Hinterlands => write!(f, "Hinterlands"),
            Self::DarkAges => write!(f, "DarkAges"),
            Self::Adventures => write!(f, "Adventures"),
            Self::Empires => write!(f, "Empires"),
            Self::Nocturne => write!(f, "Nocturne"),
            Self::Renaissance => write!(f, "Renaissance"),
            Self::Menagerie => write!(f, "Menagerie"),
            Self::Allies => write!(f, "Allies"),
            Self::Plunder => write!(f, "Plunder"),
            Self::RisingSun => write!(f, "RisingSun"),
            Self::Promos => write!(f, "Promos"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}
