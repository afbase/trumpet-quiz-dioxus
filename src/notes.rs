//! The notes of the quiz and their fingerings.
//!
//! Everything here is written pitch for B♭ trumpet in treble clef, covering the
//! range of `trumpet-fingering-chart.webp`: F♯3 (three ledger lines below the
//! staff) up to C6 (two ledger lines above it).

/// Lowest note on the chart, as a MIDI note number (F♯3).
pub const LOWEST: u8 = 54;
/// Highest note on the chart, as a MIDI note number (C6).
pub const HIGHEST: u8 = 84;

/// Valves pressed for each chromatic note from [`LOWEST`] to [`HIGHEST`],
/// written as the valve numbers held down ("" is open).
const FINGERINGS: [&str; (HIGHEST - LOWEST + 1) as usize] = [
    "123", "13", "23", "12", "1", "2", // F♯3 G3 A♭3 A3 B♭3 B3
    "", "123", "13", "23", "12", "1", // C4 C♯4 D4 E♭4 E4 F4
    "2", "", "23", "12", "1", "2", // F♯4 G4 A♭4 A4 B♭4 B4
    "", "12", "1", "2", "", "1", // C5 C♯5 D5 E♭5 E5 F5
    "2", "", "23", "12", "1", "2", // F♯5 G5 A♭5 A5 B♭5 B5
    "",  // C6
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat,
}

/// One note as it is spelled on the staff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    /// MIDI note number of the written pitch.
    pub midi: u8,
    /// Letter name, 'A'..='G'.
    pub letter: char,
    pub accidental: Accidental,
}

impl Note {
    /// Spell `midi` on the staff. Notes that need an accidental can be written
    /// two ways; `prefer_flat` picks between them (it is ignored for naturals).
    pub fn spell(midi: u8, prefer_flat: bool) -> Self {
        use Accidental::*;
        let (letter, accidental) = match (midi % 12, prefer_flat) {
            (0, _) => ('C', Natural),
            (1, false) => ('C', Sharp),
            (1, true) => ('D', Flat),
            (2, _) => ('D', Natural),
            (3, false) => ('D', Sharp),
            (3, true) => ('E', Flat),
            (4, _) => ('E', Natural),
            (5, _) => ('F', Natural),
            (6, false) => ('F', Sharp),
            (6, true) => ('G', Flat),
            (7, _) => ('G', Natural),
            (8, false) => ('G', Sharp),
            (8, true) => ('A', Flat),
            (9, _) => ('A', Natural),
            (10, false) => ('A', Sharp),
            (10, true) => ('B', Flat),
            _ => ('B', Natural),
        };
        Note {
            midi,
            letter,
            accidental,
        }
    }

    /// Vertical position on the treble staff in half staff-spaces above the
    /// bottom line: E4 is 0, F4 is 1, the top line (F5) is 8.
    pub fn staff_position(&self) -> i32 {
        let step = match self.letter {
            'C' => 0,
            'D' => 1,
            'E' => 2,
            'F' => 3,
            'G' => 4,
            'A' => 5,
            _ => 6,
        };
        // The octave number belongs to the letter, so take it from the natural
        // pitch rather than the sounding one.
        let natural = match self.accidental {
            Accidental::Natural => self.midi,
            Accidental::Sharp => self.midi - 1,
            Accidental::Flat => self.midi + 1,
        };
        let octave = i32::from(natural / 12) - 1;
        octave * 7 + step - (4 * 7 + 2)
    }

    /// Staff positions that need a ledger line drawn for this note.
    pub fn ledger_lines(&self) -> Vec<i32> {
        let pos = self.staff_position();
        if pos <= -2 {
            (pos..=-2).filter(|p| p % 2 == 0).collect()
        } else if pos >= 10 {
            (10..=pos).filter(|p| p % 2 == 0).collect()
        } else {
            Vec::new()
        }
    }

    /// Which of valves 1, 2 and 3 are pressed.
    pub fn valves(&self) -> [bool; 3] {
        let pressed = FINGERINGS[usize::from(self.midi - LOWEST)];
        ['1', '2', '3'].map(|v| pressed.contains(v))
    }

    /// Name for display, e.g. "B♭".
    pub fn name(&self) -> String {
        let suffix = match self.accidental {
            Accidental::Natural => "",
            Accidental::Sharp => "♯",
            Accidental::Flat => "♭",
        };
        format!("{}{suffix}", self.letter)
    }

    /// The fingering in words, e.g. "valves 1 and 3".
    pub fn fingering_text(&self) -> String {
        let down: Vec<&str> = ["1", "2", "3"]
            .into_iter()
            .zip(self.valves())
            .filter_map(|(n, pressed)| pressed.then_some(n))
            .collect();
        match down.as_slice() {
            [] => "open".to_string(),
            [one] => format!("valve {one}"),
            [init @ .., last] => format!("valves {} and {last}", init.join(", ")),
        }
    }
}

/// Pick the next note from two random numbers in `[0, 1)`: any pitch on the
/// chart except `previous`, with accidentals spelled sharp or flat at random.
pub fn pick(previous: Option<u8>, r_pitch: f64, r_spelling: f64) -> Note {
    let candidates: Vec<u8> = (LOWEST..=HIGHEST)
        .filter(|m| Some(*m) != previous)
        .collect();
    let index = ((r_pitch * candidates.len() as f64) as usize).min(candidates.len() - 1);
    Note::spell(candidates[index], r_spelling >= 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valves(midi: u8) -> [bool; 3] {
        Note::spell(midi, false).valves()
    }

    #[test]
    fn fingerings_match_the_chart() {
        const O: bool = false;
        const X: bool = true;
        // One row per line of the chart, four notes each.
        let chart = [
            [X, X, X],
            [X, O, X],
            [O, X, X],
            [X, X, O], // F♯3 G3 G♯3 A3
            [X, O, O],
            [O, X, O],
            [O, O, O],
            [X, X, X], // A♯3 B3 C4 C♯4
            [X, O, X],
            [O, X, X],
            [X, X, O],
            [X, O, O], // D4 D♯4 E4 F4
            [O, X, O],
            [O, O, O],
            [O, X, X],
            [X, X, O], // F♯4 G4 G♯4 A4
            [X, O, O],
            [O, X, O],
            [O, O, O],
            [X, X, O], // A♯4 B4 C5 C♯5
            [X, O, O],
            [O, X, O],
            [O, O, O],
            [X, O, O], // D5 D♯5 E5 F5
            [O, X, O],
            [O, O, O],
            [O, X, X],
            [X, X, O], // F♯5 G5 G♯5 A5
            [X, O, O],
            [O, X, O],
            [O, O, O], // A♯5 B5 C6
        ];
        assert_eq!(chart.len(), usize::from(HIGHEST - LOWEST) + 1);
        for (i, expected) in chart.iter().enumerate() {
            let midi = LOWEST + i as u8;
            assert_eq!(&valves(midi), expected, "midi {midi}");
        }
    }

    #[test]
    fn enharmonic_spellings_share_a_fingering() {
        for midi in LOWEST..=HIGHEST {
            assert_eq!(
                Note::spell(midi, false).valves(),
                Note::spell(midi, true).valves()
            );
        }
    }

    #[test]
    fn staff_positions() {
        let pos = |midi, flat| Note::spell(midi, flat).staff_position();
        assert_eq!(pos(64, false), 0); // E4, bottom line
        assert_eq!(pos(77, false), 8); // F5, top line
        assert_eq!(pos(60, false), -2); // middle C
        assert_eq!(pos(54, false), -6); // F♯3
        assert_eq!(pos(54, true), -5); // G♭3
        assert_eq!(pos(84, false), 12); // C6
        assert_eq!(pos(82, false), 10); // A♯5
        assert_eq!(pos(82, true), 11); // B♭5
        assert_eq!(pos(61, false), -2); // C♯4 sits where C4 does
        assert_eq!(pos(61, true), -1); // D♭4
    }

    #[test]
    fn ledger_lines() {
        let ledgers = |midi, flat| Note::spell(midi, flat).ledger_lines();
        assert_eq!(ledgers(54, false), vec![-6, -4, -2]); // F♯3
        assert_eq!(ledgers(54, true), vec![-4, -2]); // G♭3
        assert_eq!(ledgers(60, false), vec![-2]); // C4
        assert_eq!(ledgers(62, false), Vec::<i32>::new()); // D4
        assert_eq!(ledgers(79, false), Vec::<i32>::new()); // G5
        assert_eq!(ledgers(81, false), vec![10]); // A5
        assert_eq!(ledgers(83, false), vec![10]); // B5
        assert_eq!(ledgers(84, false), vec![10, 12]); // C6
    }

    #[test]
    fn names_and_fingering_text() {
        assert_eq!(Note::spell(70, true).name(), "B♭");
        assert_eq!(Note::spell(70, false).name(), "A♯");
        assert_eq!(Note::spell(60, true).name(), "C");
        assert_eq!(Note::spell(60, false).fingering_text(), "open");
        assert_eq!(Note::spell(59, false).fingering_text(), "valve 2");
        assert_eq!(Note::spell(62, false).fingering_text(), "valves 1 and 3");
        assert_eq!(Note::spell(54, false).fingering_text(), "valves 1, 2 and 3");
    }

    #[test]
    fn pick_covers_the_range_and_never_repeats() {
        assert_eq!(pick(None, 0.0, 0.0).midi, LOWEST);
        assert_eq!(pick(None, 0.999_999, 0.0).midi, HIGHEST);
        assert_eq!(pick(Some(LOWEST), 0.0, 0.0).midi, LOWEST + 1);
        assert_eq!(pick(Some(HIGHEST), 0.999_999, 0.0).midi, HIGHEST - 1);
        for previous in LOWEST..=HIGHEST {
            for i in 0..1000 {
                let note = pick(Some(previous), f64::from(i) / 1000.0, 0.0);
                assert_ne!(note.midi, previous);
                assert!((LOWEST..=HIGHEST).contains(&note.midi));
            }
        }
        assert_eq!(pick(None, 0.0, 0.2).accidental, Accidental::Sharp);
        assert_eq!(pick(None, 0.0, 0.7).accidental, Accidental::Flat);
    }
}
