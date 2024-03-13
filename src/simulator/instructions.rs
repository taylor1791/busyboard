use super::{Edit, Mode};
use ratatui::prelude::{Line, Stylize};

pub fn instructions(mode: &Mode, turbo: bool) -> Line {
    let mut line = vec![format!(" {}:", mode.to_string()).bold().magenta()];

    match mode {
        Mode::Edit(Edit::IP) => line.extend(vec![
            " Next ".bold().into(), "<a>".blue().bold(),
            " Jump × 2 ".bold().into(), "<s>".blue().bold(),
            " Edit ".bold().into(), "<d>".blue().bold(),
            " Exit ".bold().into(), "<q> ".blue().bold(),
        ]),
        Mode::Edit(Edit::Data) => line.extend(vec![
            " Increment ".bold().into(), "<a>".blue().bold(),
            " Shift Left ".bold().into(), "<s>".blue().bold(),
            " Step ".bold().into(), "<d>".blue().bold(),
            " Exit ".bold().into(), "<q> ".blue().bold(),
        ]),
        Mode::Execute => line.extend(vec![
            if turbo { " Normal ".bold().into() } else { " Turbo ".bold().into() },
            "<a>".blue().bold(),
            " Step ".bold().into(), "<s>".blue().bold(),
            " Seek ".bold().into(), "<d>".blue().bold(),
            " Exit ".bold().into(), "<q> ".blue().bold(),
        ]),
        Mode::Step => line.extend(vec![
            " Execute ".bold().into(), "<a>".blue().bold(),
            " Step ".bold().into(), "<s>".blue().bold(),
            " Seek ".bold().into(), "<d>".blue().bold(),
            " Exit ".bold().into(), "<q> ".blue().bold(),
        ]),
        _ => {}
    }

    Line::from(line)
}
