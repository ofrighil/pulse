use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Service {
    A,  // A Eighth Avenue Express
    C,  // C Eighth Avenue Local
    E,  // E Eighth Avenue Local
    SR, // S Rockaway Park Shuttle, designation H

    B,  // B Sixth Avenue Express
    D,  // D Sixth Avenue Express
    F,  // F Queens Boulevard Express/Sixth Avenue Local
    FX, // <F> Queens Boulevard Express/Sixth Avenue Local
    M,  // M Queens Boulevard/Sixth Avenue Local
    SF, // S Franklin Avenue Shuttle, designation S

    G, // G Brooklyn-Queens Crosstown

    J, // J Nassau Street Local
    Z, // Z Nassau Street Express

    N, // N Broadway Express
    Q, // Q Broadway Express
    R, // R Broadway Local
    W, // W Broadway Local

    L, // L 14th Street-Canarsie Local

    One,    // 1 Broadway-Seventh Avenue Local
    Two,    // 2 Seventh Avenue Express
    Three,  // 3 Seventh Avenue Express
    Four,   // 4 Lexington Avenue Express
    Five,   // 5 Lexington Avenue Express
    Six,    // 6 Lexington Avenue Local
    SixX,   // <6> Pelham Express
    Seven,  // 7 Flushing Local
    SevenX, // <7> Flushing Express
    S,      // 42nd Street Shuttle, designation 0

    SIR, // Staten Island Railway
}

impl Service {
    fn url(&self) -> &str {
        match self {
            Service::A | Service::C | Service::E | Service::SR => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace"
            }
            Service::B | Service::D | Service::F | Service::FX | Service::M | Service::SF => {
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
            | Service::SixX
            | Service::Seven
            | Service::SevenX
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
            "FX" => Service::FX,
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
            "6X" => Service::SixX,
            "7" => Service::Seven,
            "7X" => Service::SevenX,
            "GS" => Service::S,
            "SI" => Service::SIR,
            _ => panic!(""),
        }
    }
}

#[derive(Clone)]
pub struct Services(HashSet<Service>);

impl<const N: usize> From<[Service; N]> for Services {
    fn from(arr: [Service; N]) -> Self {
        Self(HashSet::from(arr))
    }
}

impl From<Vec<Service>> for Services {
    fn from(vec: Vec<Service>) -> Self {
        let mut services = Services::new();
        for v in vec {
            services.insert(v);
        }

        services
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
    INDSixthAvenue,
    INDCrosstown,
    BMTCanarise,
    BMTNassauStreet,
    BMTBroadway,
    IRTBroadwaySeventhAvenue,
    IRTLexingtonAvenue,
    IRTFlushing,
    // INDSecondAvenue,
    Shuttles,
}

impl Line {
    pub fn services(&self) -> Services {
        match self {
            Line::INDEighthAvenue => Services::from([Service::A, Service::C, Service::E]),
            Line::INDSixthAvenue => {
                Services::from([Service::B, Service::D, Service::F, Service::M])
            }
            Line::INDCrosstown => Services::from([Service::G]),
            Line::BMTCanarise => Services::from([Service::L]),
            Line::BMTNassauStreet => Services::from([Service::J, Service::Z]),
            Line::BMTBroadway => Services::from([Service::N, Service::Q, Service::R, Service::W]),
            Line::IRTBroadwaySeventhAvenue => {
                Services::from([Service::One, Service::Two, Service::Three])
            }
            Line::IRTLexingtonAvenue => {
                Services::from([Service::Four, Service::Five, Service::Six])
            }
            Line::IRTFlushing => Services::from([Service::Four, Service::Five, Service::Six]),
            Line::Shuttles => Services::from([Service::S]),
        }
    }
}
