//! Trumpet fingering flash cards.
//!
//! A note is shown on a treble staff for three seconds, then its fingering is
//! shown underneath for three seconds, then a new random note takes its place,
//! forever.

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

mod glyphs;
mod notes;

use notes::{Accidental, Note};

/// Id of the element the app mounts into. Pages that embed the quiz provide a
/// `<div>` with this id.
const ROOT_ID: &str = "trumpet-quiz-root";

/// How long each half of a card (note alone, then note with valves) lasts.
const PHASE_MS: u32 = 3_000;

// Inlined rather than linked with `asset!`: asset URLs are absolute, which
// breaks when the build output is copied into a page at some other path.
const CSS: &str = include_str!("style.css");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::web::Config::new().rootname(ROOT_ID))
        .launch(App);
}

fn random_note(previous: Option<u8>) -> Note {
    notes::pick(previous, js_sys::Math::random(), js_sys::Math::random())
}

#[component]
fn App() -> Element {
    let mut note = use_signal(|| random_note(None));
    let mut revealed = use_signal(|| false);

    use_future(move || async move {
        loop {
            TimeoutFuture::new(PHASE_MS).await;
            revealed.set(true);
            TimeoutFuture::new(PHASE_MS).await;
            let next = random_note(Some(note.peek().midi));
            revealed.set(false);
            note.set(next);
        }
    });

    rsx! {
        style { {CSS} }
        div { class: "tq",
            Staff { note: note() }
            Answer { note: note(), revealed: revealed() }
            div {
                class: if revealed() { "tq-timer tq-timer--answer" } else { "tq-timer tq-timer--question" },
            }
        }
    }
}

// Staff geometry, in SVG user units.
const VIEW_WIDTH: f64 = 300.0;
const VIEW_HEIGHT: f64 = 156.0;
/// Distance between two staff lines.
const STAFF_SPACE: f64 = 12.0;
/// y of the bottom staff line (E4).
const BOTTOM_LINE_Y: f64 = 96.0;
const STAFF_LEFT: f64 = 16.0;
const STAFF_RIGHT: f64 = VIEW_WIDTH - 16.0;
const CLEF_X: f64 = 26.0;
/// x of the centre of the notehead.
const NOTE_X: f64 = 174.0;
/// Gap between an accidental and the notehead.
const ACCIDENTAL_GAP: f64 = 5.0;
/// How far a ledger line sticks out on each side of the notehead.
const LEDGER_OVERHANG: f64 = 5.0;
const GLYPH_SCALE: f64 = STAFF_SPACE / glyphs::UNITS_PER_STAFF_SPACE;

/// y of a staff position (half staff-spaces above the bottom line).
fn staff_y(position: i32) -> f64 {
    BOTTOM_LINE_Y - f64::from(position) * STAFF_SPACE / 2.0
}

/// Transform placing a glyph's origin at `(x, y)`, scaled to the staff. Font
/// outlines have y pointing up, hence the flip.
fn glyph_transform(x: f64, y: f64) -> String {
    format!("translate({x} {y}) scale({GLYPH_SCALE} -{GLYPH_SCALE})")
}

#[component]
fn Staff(note: Note) -> Element {
    let y = staff_y(note.staff_position());
    let head_width = glyphs::WHOLE_NOTE_WIDTH * GLYPH_SCALE;
    let head_left = NOTE_X - head_width / 2.0;

    let accidental = match note.accidental {
        Accidental::Natural => None,
        Accidental::Sharp => Some((glyphs::SHARP, glyphs::SHARP_WIDTH)),
        Accidental::Flat => Some((glyphs::FLAT, glyphs::FLAT_WIDTH)),
    }
    .map(|(path, width)| (path, head_left - ACCIDENTAL_GAP - width * GLYPH_SCALE));

    rsx! {
        svg {
            class: "tq-staff",
            view_box: "0 0 {VIEW_WIDTH} {VIEW_HEIGHT}",
            role: "img",
            "aria-label": "A note on a treble clef staff",
            // The five lines of the staff, bottom to top.
            for line in 0..5 {
                line {
                    x1: STAFF_LEFT,
                    x2: STAFF_RIGHT,
                    y1: staff_y(line * 2),
                    y2: staff_y(line * 2),
                    stroke_width: 1.0,
                }
            }
            // The G clef curls around the G line, second from the bottom.
            path { d: glyphs::G_CLEF, transform: glyph_transform(CLEF_X, staff_y(2)) }
            for position in note.ledger_lines() {
                line {
                    x1: head_left - LEDGER_OVERHANG,
                    x2: head_left + head_width + LEDGER_OVERHANG,
                    y1: staff_y(position),
                    y2: staff_y(position),
                    stroke_width: 1.5,
                }
            }
            if let Some((path, x)) = accidental {
                path { d: path, transform: glyph_transform(x, y) }
            }
            path { d: glyphs::WHOLE_NOTE, transform: glyph_transform(head_left, y) }
        }
    }
}

/// The note's name and fingering. Always laid out, so the card keeps its size,
/// but invisible (and hidden from assistive tech) until `revealed`.
#[component]
fn Answer(note: Note, revealed: bool) -> Element {
    rsx! {
        div {
            class: if revealed { "tq-answer tq-answer--shown" } else { "tq-answer tq-answer--hidden" },
            aria_hidden: !revealed,
            p { class: "tq-name", "{note.name()}" }
            ul { class: "tq-valves", aria_hidden: true,
                for (number, down) in (1..=3).zip(note.valves()) {
                    li {
                        class: if down { "tq-valve tq-valve--down" } else { "tq-valve" },
                        "{number}"
                    }
                }
            }
            p { class: "tq-fingering", "{note.fingering_text()}" }
        }
    }
}
