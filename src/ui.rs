use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::tui_menus::app::{App, AppState};

fn create_title() -> Text<'static> {
    Text::from(vec![
        Line::from("Press `Esc`, `Ctrl-C` or `q` to stop running."),
        Line::from(" ██████╗ ██╗  ██╗███████╗███████╗███████╗    ████████╗██╗   ██╗██╗"),
        Line::from("██╔════╝ ██║  ██║██╔════╝██╔════╝██╔════╝    ╚══██╔══╝██║   ██║██║"),
        Line::from("██║      ███████║█████╗  █████╗  ███████╗       ██║   ██║   ██║██║"),
        Line::from("██║      ██╔══██║██╔══╝  ██╔══╝  ╚════██║       ██║   ██║   ██║██║"),
        Line::from("╚██████╗ ██║  ██║███████╗███████╗███████║       ██║   ╚██████╔╝██║"),
        Line::from(" ╚═════╝ ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝       ╚═╝    ╚═════╝ ╚═╝"),
        Line::from("░ A chess game made in 💤 ░"),
        Line::from(""),
    ])
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(current_state) = self.state.last() {
            match current_state {
                AppState::MainMenu => self.render_main_menu(area, buf),
                AppState::Game => self.render_game_menu(area, buf),
                AppState::Help => self.render_help(area, buf),
                AppState::Credits => self.render_credits(area, buf),
            }
        }
    }
}

impl App {
    fn render_main_menu(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("CHESS-TUI!")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        // Create menu items with proper styling
        let mut menu_lines = Vec::new();
        for (i, item) in self.menu_selector.items.iter().enumerate() {
            if i == self.menu_selector.selected_index {
                menu_lines.push(Line::from(vec![
                    Span::styled("→ ", Style::default().fg(Color::Yellow)),
                    Span::styled(item.clone(), Style::default().fg(Color::Yellow)),
                ]));
            } else {
                menu_lines.push(Line::from(vec![Span::raw("  "), Span::raw(item.clone())]));
            }
        }

        // Combine all text elements
        let mut text = create_title().lines;
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

    fn render_game_menu(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Game Menu")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = Text::from(vec![
            Line::from("Game in progress..."),
            Line::from(format!("Counter: {}", self.counter)),
        ]);

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }

    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Help")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = Text::from(vec![
            Line::from("Help section..."),
            Line::from(format!("Counter: {}", self.counter)),
        ]);

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }

    fn render_credits(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Credits")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = Text::from(vec![
            Line::from("Credits section..."),
            Line::from(format!("Counter: {}", self.counter)),
        ]);

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }
}
