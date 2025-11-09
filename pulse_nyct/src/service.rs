use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Service {
    A,
    C,
    E,
    SR, // H?

    B,
    D,
    F,
    M,
    SF,

    G,

    J,
    Z,

    N,
    Q,
    R,
    W,

    L,

    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    S,

    SIR,
}

impl Service {
    fn url(&self) -> &str {
        match self {
            Service::A | Service::C | Service::E | Service::SR => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace"
            }
            Service::B | Service::D | Service::F | Service::M | Service::SF => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm"
            }
            Service::G => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-g",
            Service::J | Service::Z => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-jz"
            }
            Service::N | Service::Q | Service::R | Service::W => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-nqrw"
            }
            Service::L => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-l",
            Service::One
            | Service::Two
            | Service::Three
            | Service::Four
            | Service::Five
            | Service::Six
            | Service::Seven
            | Service::S => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs",
            Service::SIR => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-si",
        }
    }
}

impl From<&str> for Service {
    fn from(id: &str) -> Service {
        match id {
            "A" => Service::A,
            "C" => Service::C,
            "E" => Service::E,
            "H" => Service::SR,
            "B" => Service::B,
            "D" => Service::D,
            "F" => Service::F,
            "M" => Service::M,
            "FS" => Service::SF,
            "G" => Service::G,
            "J" => Service::J,
            "Z" => Service::Z,
            "N" => Service::N,
            "Q" => Service::Q,
            "R" => Service::R,
            "W" => Service::W,
            "L" => Service::L,
            "1" => Service::One,
            "2" => Service::Two,
            "3" => Service::Three,
            "4" => Service::Four,
            "5" => Service::Five,
            "6" => Service::Six,
            "7" => Service::Seven,
            "GS" => Service::S,
            "SI" => Service::SIR,
            _ => panic!(""),
        }
    }
}

pub struct Services(HashSet<Service>);

impl<const N: usize> From<[Service; N]> for Services {
    fn from(arr: [Service; N]) -> Self {
        Self(HashSet::from(arr))
    }
}

impl Services {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert(&mut self, value: Service) -> bool {
        self.0.insert(value)
    }

    pub fn remove(&mut self, value: Service) -> bool {
        self.0.remove(&value)
    }

    pub fn urls(&self) -> Vec<&str> {
        self.0.iter().map(|s| s.url()).unique().collect()
    }

    pub fn contains(&self, value: &Service) -> bool {
        self.0.contains(value)
    }
}

pub enum Line {
    INDEighthAvenue,
}

impl Line {
    pub fn services(&self) -> Services {
        match self {
            Line::INDEighthAvenue => Services::from([Service::A, Service::C, Service::E]),
        }
    }
}
