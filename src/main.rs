use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    text::{Text, Line},
    prelude::*,
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let mut term_lines = vec![
        Line::from("THE FIRST LINE"),
        Line::from("THE SECOND LINE"),
        Line::from("THE THIRD LINE"),
        Line::from("THE FOURTH LINE"),
        Line::from("THE FIFTH LINE"),
        ];


        // let text= Text::from(vec![
        //     Line::from("hello world 1").left_aligned(),
        //     Line::from("hello world 2"),
        //     Line::from("hello world 3").right_aligned(),



    term_lines.push(Line::from("AND A SIXTH LINE..."));


    // Line::from("THE FIFTH LINE")

    // let mut term_lines = term_content.iter().map(|line_content| Line::from("SOME LINE ... ??")).collect();

    // let mut term_lines = [];
    // for line_content in term_content {
    //     term_lines.push(Line::from(line_content));
    // }

    let term_text = Text::from(term_lines).green().bold();

    terminal.draw(|frame| {
        frame.render_widget(term_text, frame.area());
    })?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}