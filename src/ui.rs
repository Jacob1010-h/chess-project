use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::{App, AppState};
use crate::game::Game;

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

impl Widget for &mut App {
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

    fn render_game_menu(&mut self, area: Rect, buf: &mut Buffer) {
        // Split the area into board (left 2/3) and move list (right 1/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
            .split(area);

        self.game = Some(Game::new());

        self.render_chess_board(chunks[0], buf, self.game.as_ref().unwrap());
    }

    fn render_chess_board(&self, area: Rect, buf: &mut Buffer, game: &Game) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));

        let board_dimensions = game.get_board().dimensions();
        // 8x8 chess board
        let cell_width = area.width / board_dimensions.0 as u16;
        let cell_height = area.height / board_dimensions.1 as u16;

        // Render the board squares
        for row in 0..board_dimensions.1 {
            for col in 0..board_dimensions.0 {
                let is_light = (row + col) % 2 == 0;
                let color = if is_light {
                    Color::Rgb(210, 180, 140) // Light square
                } else {
                    Color::Rgb(139, 69, 19) // Dark square
                };

                let x = area.x + col as u16 * cell_width;
                let y = area.y + row as u16 * cell_height;
                let cell_area = Rect::new(x, y, cell_width, cell_height);

                // Draw the square background
                buf.set_style(cell_area, Style::default().bg(color));

                // Draw pieces (replace with your game state)
                if let Some(piece) = game.get_board().get_piece_at((row, col)) {
                    let symbol = piece.symbol();
                    buf.set_string(
                        x + cell_width / 2,
                        y + cell_height / 2,
                        symbol.to_string(),
                        Style::default()
                            .fg(piece.color.to_color())
                            .add_modifier(Modifier::BOLD),
                    );
                }
            }
        }

        // Draw coordinates if there's space
        if cell_width >= 3 && cell_height >= 2 {
            for i in 0..8 {
                // Files (a-h at bottom)
                buf.set_string(
                    area.x + (i as u16 * cell_width) + cell_width / 2,
                    area.y + area.height - 1,
                    format!("{}", (b'a' + i) as char),
                    Style::default(),
                );

                // Ranks (1-8 at left)
                buf.set_string(
                    area.x,
                    area.y + (i as u16 * cell_height) + cell_height / 2,
                    format!("{}", 8 - i),
                    Style::default(),
                );
            }
        }

        block.render(area, buf);
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
