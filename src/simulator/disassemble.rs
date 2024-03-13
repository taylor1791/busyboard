use crate::eater::{Disassembly, I};
use ratatui::prelude::{Line, Span, Stylize};

pub fn disassemble<'a>(disassembly: &'a [Disassembly], ip: u8, bytes: &'a [u8], previous_bytes: &'a [u8]) -> Vec<Line<'a>> {
    let mut lines = vec![];

    for segment in disassembly {
        let offset = segment.offset() as usize;
        let ip = ip as usize;

        match segment {
            Disassembly::Data { data, .. } => {
                for i in (0..data.len()).step_by(2) {
                    let mut line = vec![Span::raw(format!("{:02x}: ", offset + i))];

                    let n = format!("{:02x}", data[i]);
                    let n = if offset + i == ip { n.magenta().bold().underlined() } else { n.into() };
                    let n = if has_changed(data, i, previous_bytes, offset + i) { n.green() } else { n.into() };
                    line.push(n);

                    line.push(Span::raw(" "));

                    if i + 1 < data.len() {
                        let n = format!("{:02x}", data[i + 1]);
                        let n = if offset + i + 1 == ip { n.magenta().bold().underlined() } else { n.into() };
                        let n = if has_changed(data, i + 1, previous_bytes, offset + i + 1) { n.green() } else { n.into() };
                        line.push(n);
                    }

                    lines.push(Line::from(line));
                }
            },
            Disassembly::Instruction { instruction, .. } => {
                let mut line = vec![Span::raw(format!("{:02x}: ", offset))];

                let formatted = to_string(&instruction).bold();
                let formatted = if offset == ip { formatted.magenta().bold().underlined() } else { formatted };
                let formatted = if has_changed(bytes, offset, previous_bytes, offset) { formatted.green() } else { formatted };
                line.push(formatted);

                line.push(Span::raw(" "));

                if let I::Ldi(..) | I::Lda(..) | I::Sta(..) | I::Add(..) |
                    I::Sub(..) | I::Jmp(..) | I::Jpz(..) | I::Jpc(..) = instruction {
                    let data = format!("{:02x}", bytes[offset + 1]);
                    let data = if offset + 1 == ip { data.magenta().bold().underlined() } else { data.into() };
                    let data = if has_changed(bytes, offset + 1, previous_bytes, offset + 1) { data.green() } else { data.into() };
                    line.push(data);
                }

                lines.push(Line::from(line));
            },
        }
    }

    lines
}

fn to_string(i: &I) -> String {
    match i {
        I::Nop(..) => "Nop",
        I::Ldi(..) => "Ldi",
        I::Lda(..) => "Lda",
        I::Sta(..) => "Sta",
        I::Add(..) => "Add",
        I::Sub(..) => "Sub",
        I::Jmp(..) => "Jmp",
        I::Jpz(..) => "Jpz",
        I::Jpc(..) => "Jpc",
        I::Out(..) => "Out",
        I::Hlt(..) => "Hlt",
    }.to_string()
}

fn has_changed(bytes: &[u8], current: usize, previous_bytes: &[u8], previous: usize) -> bool {
    if previous >= previous_bytes.len() {
        return true;
    }

    bytes[current] != previous_bytes[previous]
}
