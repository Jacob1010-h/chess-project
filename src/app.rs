use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use crate::game::Game;

#[derive(Debug, PartialEq)]
pub enum AppState {
    MainMenu,
    Game,
    Help,
    Credits,
}

#[derive(Default, Debug, Clone)]
pub struct MenuSelector {
    pub selected_index: usize,
    pub num_items: usize,
    pub items: Vec<String>,
}

impl MenuSelector {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            num_items: 3,
            items: vec![
                "Normal game".to_string(),
                "Help section".to_string(),
                "Credits".to_string(),
            ],
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.num_items - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select(&mut self, index: usize) {
        if index < self.num_items {
            self.selected_index = index;
        }
    }

    pub fn get(&self) -> Option<&String> {
        if self.selected_index < self.num_items {
            Some(&self.items[self.selected_index])
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub events: EventHandler,
    pub menu_selector: MenuSelector,
    pub state: Vec<AppState>,
    pub game: Option<Game>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            menu_selector: MenuSelector::new(),
            state: vec![AppState::MainMenu],
            game: None, // Game instance will be created when entering the game state
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?;
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    fn handle_main_menu_keys(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Up => self.menu_selector.previous(),
            KeyCode::Down => self.menu_selector.next(),
            KeyCode::Enter => self.select_app_state(),
            _ => {}
        }
        Ok(())
    }

    fn select_app_state(&mut self) {
        match self.menu_selector.selected_index {
            0 => self.state.push(AppState::Game),
            1 => self.state.push(AppState::Help),
            2 => self.state.push(AppState::Credits),
            _ => {}
        }
    }

    fn handle_window_keys(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => self.quit(),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if self.state.last() == Some(&AppState::MainMenu) {
            self.handle_main_menu_keys(key_event)?;
        }
        self.handle_window_keys(key_event)?;

        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        if self.state.len() > 1 {
            self.state.pop();
        } else {
            self.state.clear();
        }
        if self.state.is_empty() {
            self.running = false;
        }
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
