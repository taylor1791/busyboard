use super::Out;
use ratatui::{
    prelude::{Line, Span, Stylize, Widget},
    widgets::Paragraph,
};
use std::cell::Ref;

pub fn out(out: Ref<Out>) -> impl Widget {
    let mut headings = vec![Span::raw("     ")];
    let mut data = vec![Span::raw(" "), Span::raw("Out:").cyan()];

    let n = if out.n < 16 { 0 } else { out.n - 16 };
    for i in n..(n+16) {
        let heading = format!(" {:2x}", i);
        headings.push(Span::raw(heading));
    }

    for i in n..out.n {
        let byte = format!(" {:02x}", out.data[i % 16]);
        let byte = if out.new && i == (out.n - 1) { byte.green() } else { byte.into() };
        data.push(byte);
    }

    Paragraph::new(vec![Line::from(headings), Line::from(data)])
}
