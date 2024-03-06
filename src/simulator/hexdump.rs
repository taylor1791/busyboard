use ratatui::{
    prelude::{Line, Span, Stylize, Widget},
    widgets::{Block, Padding, Paragraph},
};

pub fn hexdump(ip: u8, bytes: &[u8]) -> impl Widget {
    let mut lines = vec![];

    let mut b = 0;
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let mut line = Vec::with_capacity(17);
        line.push(format!("{:02x}:", i * 16).bold().green());

        for byte in chunk {
            line.push(Span::raw(" "));
            if b == ip {
                line.push(format!("{:02x}", byte).black().on_blue());
            } else {
                line.push(Span::raw(format!("{:02x}", byte)));
            }
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
