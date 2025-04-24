use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::tui_menus::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("CHESS-T'U!")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let menu_items = [
            "Normal game multiplayer",
            "Play against a friend or computer",
            "Default mode",
            "Help section",
            "Credits",
        ];

        // Create the title text
        let title_text = Text::from(vec![
            Line::from("Press `Esc`, `Ctrl-C` or `q` to stop running."),
            Line::from(" ██████╗ ██╗  ██╗███████╗███████╗███████╗    ████████╗██╗   ██╗██╗"),
            Line::from("██╔════╝ ██║  ██║██╔════╝██╔════╝██╔════╝    ╚══██╔══╝██║   ██║██║"),
            Line::from("██║      ███████║█████╗  █████╗  ███████╗       ██║   ██║   ██║██║"),
            Line::from("██║      ██╔══██║██╔══╝  ██╔══╝  ╚════██║       ██║   ██║   ██║██║"),
            Line::from("╚██████╗ ██║  ██║███████╗███████╗███████║       ██║   ╚██████╔╝██║"),
            Line::from(" ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝       ╚═╝    ╚═════╝ ╚═╝"),
            Line::from("░ A chess game made in 💤 ░"),
            Line::from(""),
        ]);

        // Create menu items with proper styling
        let mut menu_lines = Vec::new();
        for (i, item) in menu_items.iter().enumerate() {
            if i == self.selected_index {
                menu_lines.push(Line::from(vec![
                    Span::styled("→ ", Style::default().fg(Color::Yellow)),
                    Span::styled(*item, Style::default().fg(Color::Yellow)),
                ]));
            } else {
                menu_lines.push(Line::from(vec![Span::raw("  "), Span::raw(*item)]));
            }
        }

        // Combine all text elements
        let mut text = title_text.lines;
        text.extend(menu_lines);
        text.push(Line::from(""));
        text.push(Line::from(format!("Counter: {}", self.counter)));

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }
}
