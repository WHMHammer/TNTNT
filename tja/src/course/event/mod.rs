pub mod branch;
pub mod nextsong;
use branch::Branches;
use nextsong::Nextsong;

pub enum EventType {
    Empty,         // 0
    Don,           // 1
    Ka,            // 2
    BigDon,        // 3
    BigKa,         // 4
    Drumroll,      // 5
    BigDrumroll,   // 6
    Balloon,       // 7
    End,           // 8
    BigBalloon,    // 9
    DualPlayerDon, // A
    DualPlayerKa,  // B
    Bomb,          // C
    ADLIB,         // F
    Purple,        // G
    MEASUREEND,    // ,
    MEASURE(u8, u8),
    BPMCHANGE(f64),
    DELAY(f64),
    SCROLL(f64),
    GOGOSTART,
    GOGOEND,
    BARLINEOFF,
    BARLINEON,
    BRANCH(Branches),
    SECTION,
    LYRIC(String),
    LEVELHOLD,
    NEXTSONG(Nextsong),
}

impl std::fmt::Debug for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EventType::*;
        match self {
            Empty => write!(f, "0"),
            Don => write!(f, "1"),
            Ka => write!(f, "2"),
            BigDon => write!(f, "3"),
            BigKa => write!(f, "4"),
            Drumroll => write!(f, "5"),
            BigDrumroll => write!(f, "6"),
            Balloon => write!(f, "7"),
            End => write!(f, "8"),
            BigBalloon => write!(f, "9"),
            DualPlayerDon => write!(f, "A"),
            DualPlayerKa => write!(f, "B"),
            Bomb => write!(f, "C"),
            ADLIB => write!(f, "F"),
            Purple => write!(f, "G"),
            MEASUREEND => write!(f, ","),
            MEASURE(numerator, denominator) => write!(f, "#MEASURE {},{}", numerator, denominator),
            BPMCHANGE(bpm) => write!(f, "#BPMCHANGE {}", bpm),
            DELAY(delay) => write!(f, "#DELAY {}", delay),
            SCROLL(scroll) => write!(f, "#SCROLL {}", scroll),
            GOGOSTART => write!(f, "#GOGOSTART"),
            GOGOEND => write!(f, "#GOGOEND"),
            BARLINEOFF => write!(f, "#BARLINEOFF"),
            BARLINEON => write!(f, "#BARLINEON"),
            BRANCH(branches) => write!(f, "{:?}", branches),
            SECTION => write!(f, "#SECTION"),
            LYRIC(lyric) => write!(f, "#LYRIC {}", lyric),
            LEVELHOLD => write!(f, "#LEVELHOLD"),
            NEXTSONG(nextsong) => write!(f, "{:?}", nextsong),
        }
    }
}

pub struct Event {
    pub offset: f64, // delay before the event takes place
    pub event_type: EventType,
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EventType::*;
        match self.event_type {
            BRANCH(_) => write!(f, "{:?}", self.event_type),
            NEXTSONG(_) => write!(f, "\n{:?}", self.event_type),
            _ => write!(f, "{:?}\t@ {:.3}s", self.event_type, self.offset),
        }
    }
}