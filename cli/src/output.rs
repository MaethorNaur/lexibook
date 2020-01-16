use crate::errors::Result;
use lexibook::sound_system::phone;
use lexibook::sound_system::rules::{History, Transformation};
use lexibook::sound_system::SoundSystem;
use prettytable::format::Alignment;
use prettytable::{color, Attr, Cell, Row, Table};
use std::convert::From;
use std::fs::File;

pub fn stdout(table: &Table) -> Result<()> {
    table.printstd();
    Ok(())
}
static PLACES: [phone::ConsonantPlace; 11] = [
    phone::ConsonantPlace::Bilabial,
    phone::ConsonantPlace::LabioDental,
    phone::ConsonantPlace::Dental,
    phone::ConsonantPlace::Alveolar,
    phone::ConsonantPlace::PostAlveolar,
    phone::ConsonantPlace::Retroflex,
    phone::ConsonantPlace::Palatal,
    phone::ConsonantPlace::Velar,
    phone::ConsonantPlace::Uvular,
    phone::ConsonantPlace::Pharyngeal,
    phone::ConsonantPlace::Glottal,
];
static MANNERS: [phone::ConsonantManner; 10] = [
    phone::ConsonantManner::Nasal,
    phone::ConsonantManner::Stop,
    phone::ConsonantManner::SibilantFricative,
    phone::ConsonantManner::Fricative,
    phone::ConsonantManner::Trill,
    phone::ConsonantManner::Tap,
    phone::ConsonantManner::LateralFricative,
    phone::ConsonantManner::Approximant,
    phone::ConsonantManner::LateralApproximant,
    phone::ConsonantManner::LateralTap,
];
static BACKNESSES: [phone::Backness; 3] = [
    phone::Backness::Front,
    phone::Backness::Central,
    phone::Backness::Back,
];
static HEIGHT: [phone::Height; 7] = [
    phone::Height::Close,
    phone::Height::NearClose,
    phone::Height::CloseMid,
    phone::Height::Mid,
    phone::Height::OpenMid,
    phone::Height::NearOpen,
    phone::Height::Open,
];

pub fn csv(table: &Table, filename: &str) -> Result<()> {
    File::create(filename)
        .map_err(From::from)
        .and_then(|buffer| table.to_csv(buffer))
        .map(|_| info!("Wrote into: {}", filename))
        .map_err(From::from)
}

pub fn create_phonology_tables(sound_system: SoundSystem) -> (Table, Table) {
    let phonology = sound_system.phonology();
    let (consonants, vowels): (Vec<phone::Phone>, Vec<phone::Phone>) =
        phonology.iter().partition(|phone| match phone {
            phone::Phone::Consonant(_) => true,
            _ => false,
        });

    (consonants_table(consonants), vowels_table(vowels))
}

fn consonants_table(consonants: Vec<phone::Phone>) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new(""),
        Cell::new_align("Labial", Alignment::CENTER)
            .with_style(Attr::Bold)
            .with_hspan(2),
        Cell::new_align("Coronal", Alignment::CENTER)
            .with_style(Attr::Bold)
            .with_hspan(4),
        Cell::new_align("Dorsal", Alignment::CENTER)
            .with_style(Attr::Bold)
            .with_hspan(3),
        Cell::new_align("Laryngeal", Alignment::CENTER)
            .with_style(Attr::Bold)
            .with_hspan(2),
    ]));
    table.add_row(Row::new(vec![
        Cell::new(""),
        Cell::new_align("Bilabial", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Labio­dental", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Dental", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Alveolar", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Post­alveolar", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Retro­flex", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Palatal", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Velar", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Uvular", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Pharyngeal", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Glottal", Alignment::CENTER).with_style(Attr::Bold),
    ]));

    MANNERS.iter().for_each(|current_manner| {
        let row = Row::new(PLACES.iter().fold(
            vec![
                            Cell::new_align(&current_manner.to_string(), Alignment::CENTER)
                                .with_style(Attr::Bold),
                        ],
            |mut row, current_place| {
                let cell = consonants
                    .iter()
                    .map(|consonant| match consonant {
                        phone::Phone::Consonant(phone::Consonant { place, manner, .. })
                            if (place == current_place
                                || (*current_place == phone::ConsonantPlace::Velar
                                    && *place == phone::ConsonantPlace::LabioVelar))
                                && manner == current_manner =>
                        {
                            let c: char = (*consonant).into();
                            c.to_string()
                        }
                        _ => "".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let cell_row = if cell.trim().is_empty() {
                    Cell::new(cell.trim()).with_style(Attr::BackgroundColor(color::BRIGHT_WHITE))
                } else {
                    Cell::new(cell.trim())
                };
                row.push(cell_row);
                row
            },
        ));
        table.add_row(row);
    });
    table
}

fn vowels_table(vowels: Vec<phone::Phone>) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new(""),
        Cell::new_align("Front", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Central", Alignment::CENTER).with_style(Attr::Bold),
        Cell::new_align("Back", Alignment::CENTER).with_style(Attr::Bold),
    ]));
    HEIGHT.iter().for_each(|current_height| {
        let row = BACKNESSES.iter().fold(
            vec![
                Cell::new_align(&current_height.to_string(), Alignment::CENTER)
                    .with_style(Attr::Bold),
            ],
            |mut row, current_backness| {
                let cell = vowels
                    .iter()
                    .map(|vowel| match vowel {
                        phone::Phone::Vowel(phone::Vowel {
                            backness, height, ..
                        }) if backness == current_backness && height == current_height => {
                            let c: char = (*vowel).into();
                            c.to_string()
                        }
                        _ => "".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let cell_row = if cell.trim().is_empty() {
                    Cell::new(cell.trim()).with_style(Attr::BackgroundColor(color::BRIGHT_WHITE))
                } else {
                    Cell::new(cell.trim())
                };
                row.push(cell_row);
                row
            },
        );
        table.add_row(Row::new(row));
    });
    table
}
pub fn create_table(
    sound_system: &SoundSystem,
    words: Vec<String>,
    transformations: Transformation,
) -> Table {
    let mut table = Table::new();
    let mut header = vec![Cell::new("Generated Word")
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(color::CYAN))];
    let rules = transformations
        .history
        .iter()
        .map(|t| &t.rule)
        .collect::<Vec<_>>();
    rules.iter().for_each(|rule| {
        header.push(
            Cell::new(rule)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_RED)),
        )
    });
    if !rules.is_empty() {
        header.push(
            Cell::new("Final word")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN)),
        );
        header.push(
            Cell::new("IPA")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN)),
        );
    }
    table.add_row(Row::new(header));
    words.iter().enumerate().for_each(|(i, word)| {
        let mut row = vec![Cell::new(word)];
        rules.iter().enumerate().for_each(|(rule, _)| {
            if let Some(History { words: history, .. }) = transformations.history.get(rule) {
                let previous = if rule == 0 {
                    word
                } else {
                    let History {
                        words: previous_history,
                        ..
                    } = &transformations.history[rule - 1];
                    &previous_history[i]
                };
                let current = &history[i];
                let to_add = if current == previous {
                    Cell::new("")
                } else {
                    Cell::new(current)
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::RED))
                };
                row.push(to_add);
            }
        });
        if !transformations.output.is_empty() {
            let word = &transformations.output[i];
            row.push(Cell::new(&word).with_style(Attr::Bold));
            row.push(
                Cell::new(&format!("/{}/", sound_system.ipa_representation(&word)))
                    .with_style(Attr::Italic(true)),
            );
        }
        table.add_row(Row::new(row));
    });
    table
}
