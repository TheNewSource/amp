extern crate bloodhound;
extern crate rustbox;
extern crate scribe;

use application::modes::open::OpenMode;
use terminal::Terminal;
use rustbox::Color;
use scribe::buffer::{Token, Category};
use pad::PadStr;

pub fn display(terminal: &Terminal, tokens: &Vec<Token>, mode: &OpenMode) {
    terminal.clear();

    // Place the cursor on the search input line, right after its contents.
    terminal.set_cursor(mode.input.len() as isize, 5);

    // Draw the list of search results.
    for (line, result) in mode.results.iter().enumerate() {
        let color = if line == mode.selected_index() { Color::Black } else { Color::Default };
        let padded_content = result.path.as_path().to_str().unwrap().pad_to_width(terminal.width());
        terminal.print(0, line, rustbox::RB_NORMAL, Color::White, color, &padded_content);
    }

    // Draw the divider.
    let line = 5;
    let padded_content = mode.input.pad_to_width(terminal.width());
    terminal.print(0, line, rustbox::RB_BOLD, Color::Black, Color::White, &padded_content);

    let mut line = 6;
    let mut offset = 0;
    let line_limit = terminal.height() - 5;
    'print_loop: for token in tokens.iter() {
        let color = match token.category {
            Category::Keyword    => Color::Magenta,
            Category::Identifier => Color::Yellow,
            Category::String     => Color::Red,
            Category::Comment    => Color::Blue,
            Category::Method     => Color::Cyan,
            _                    => Color::Default,
        };

        for character in token.lexeme.chars() {
            if character == '\n' {
                line += 1;
                offset = 0;
            } else if line < line_limit {
                terminal.print_char(offset, line, rustbox::RB_NORMAL, color, Color::Default, character);
                offset += 1;
            }
        }
    }

    terminal.present();
}