use ratatui::{
    prelude::{Line, Span, Stylize, Widget},
    widgets::{Block, Padding, Paragraph},
};

pub fn hexdump(ip: u8, bytes: &[u8], previous_bytes: &[u8]) -> impl Widget {
    let mut lines = vec![Line::from("     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f")];

    let mut b = 0;
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let mut line = Vec::with_capacity(17);
        line.push(format!("{:02x}:", i * 16).cyan());

        for byte in chunk {
            line.push(Span::raw(" "));

            let n = format!("{:02x}", byte);
            let n = if b == ip { n.magenta().bold().underlined() } else { Span::raw(n) };
            let n = if *byte != previous_bytes[b as usize] { n.green() } else { n };

            line.push(n);

            b += 1;
        }

        lines.push(Line::from(line));
    }

    let dump = Paragraph::new(lines)
        .block(Block::bordered()
        .title_top(Line::from(" Hex Dump ").left_aligned())
        .padding(Padding::horizontal(1)));

    dump
}
