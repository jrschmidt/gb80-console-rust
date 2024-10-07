use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    text::{Text, Line},
    prelude::*,
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let term_content = vec![
        "THE FIRST LINE",
        "THE SECOND LINE",
        "THE THIRD LINE",
        "THE FOURTH LINE",
        "THE FIFTH LINE",
        "AND A SIXTH LINE...",
        "LINE NUMBER 7",
        "LINE NUMBER 8",
        "LINE NUMBER 9",
        "LINE NUMBER 10",
        "LINE NUMBER 11",
        "LINE NUMBER 12",
        "LINE NUMBER 13",
        "LINE NUMBER 14",
        "LINE NUMBER 15",
        "LINE NUMBER 16",
        "LINE NUMBER 17",
        "LINE NUMBER 18",
        "LINE NUMBER 19",
        "LINE NUMBER 20",
        "LINE NUMBER 21",
        "LINE NUMBER 22",
        "LINE NUMBER 23",
        "LINE NUMBER 24"
        ];

    let term_text = text_from_string_slice_array(term_content);

    terminal.draw(|frame| {
        frame.render_widget(term_text, frame.area());
    })?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => break,
                    KeyCode::Char('b') => break,
                    _ => {}
                }
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn text_from_string_slice_array(content: Vec<&str>) -> Text {
    let mut t_lines = vec![];
    for i in 0..23 {
        t_lines.push(Line::from(content[i]));
    }

    let t_text = Text::from(t_lines).green().bold();
    return t_text;
}