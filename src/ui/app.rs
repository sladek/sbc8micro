use crate::commands::{MIN_DISASM_RANGE, command::Command};
use crate::cpu::{Cpu, CpuUi};
//use color_eyre::eyre::Ok;
use crate::commands::cpu_not_set_error;
use crate::ui::{COMMAND_HISTORY_SIZE, COMMAND_HISTORY_SIZE_INIT_INDEX, OUTPUT_HISTORY_SIZE};
use ratatui::crossterm::event::KeyModifiers;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::DisableMouseCapture,
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::LeaveAlternateScreen,
    },
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
};
use std::io::Result;

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
    /// Application state
    pub app_state: AppState,
    /// Status of Output window
    output_view_status: OutputViewStatus,
    /// Status of scroll bar
    output_scroll_status: ScrollbarState,
    command_history: CommandHistory,
    /// CPU currently in use
    pub cpu: Cpu,
    pub cpu_ui: Option<Box<dyn CpuUi>>,
    pub dump: Dump,
    pub disasm: Disasm,
}
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Default, Debug, Clone)]
pub struct CommandHistory {
    /// Commands histrory
    command_history: Vec<String>,
    command_history_size: usize,
    command_history_position: i16,
}
impl CommandHistory {
    pub fn new() -> Self {
        Self {
            command_history: Vec::new(),
            command_history_size: COMMAND_HISTORY_SIZE,
            command_history_position: COMMAND_HISTORY_SIZE_INIT_INDEX,
        }
    }
    /// Returns next line from command history
    fn command_history_up(&mut self) -> Option<String> {
        if self.command_history.is_empty() {
            return None;
        }
        if self.command_history_position < self.command_history.len() as i16 - 1 {
            self.command_history_position += 1;
        }
        if self.command_history_position < 0 {
            self.command_history_position += 0;
        }
        Some(self.command_history[self.command_history_position as usize].clone())
    }
    /// Returns previous line from command history
    fn command_history_down(&mut self) -> Option<String> {
        if self.command_history_position == 0 {
            return Some("".to_string());
        }
        //        let current_position = self.command_history_position;
        self.command_history_position -= 1;
        Some(self.command_history[self.command_history_position as usize].clone())
    }
}
#[derive(Default, Clone)]
pub struct OutputViewStatus {
    // Moves output windows up and down based on offset
    // If +x moves up x lines, if -x moves down x lines
    line_offset: i16,
    page_offset: i16,
    max_lines: usize,
}
/// Keeps data about view, like start_line
///
/// The view will start on the line specified in start_line.
impl OutputViewStatus {
    pub fn new() -> Self {
        Self {
            line_offset: 0,
            page_offset: 0,
            max_lines: OUTPUT_HISTORY_SIZE,
        }
    }
    pub fn set_line_offset(&mut self, offset: i16) {
        self.line_offset = offset;
    }
    pub fn get_line_offset(&self) -> i16 {
        self.line_offset
    }
    pub fn set_page_offset(&mut self, offset: i16) {
        self.page_offset = offset;
    }
    pub fn get_page_offset(&self) -> i16 {
        self.page_offset
    }
    pub fn get_output_history_size(&self) -> usize {
        self.max_lines
    }
    pub fn set_output_history_size(&mut self, size: usize) {
        self.max_lines = size;
    }
}

#[derive(Debug, Clone)]
pub enum AppState {
    Home,
    Opcodes8080,
    Opcodes6502,
    Quit,
}
#[derive(Default, Clone)]
pub struct Dump {
    pub start: u16,
    pub end: u16,
    pub range: u16,
}
#[derive(Default, Clone)]
pub struct Disasm {
    pub start: u16,
    //    pub end: u16,
    pub range: u16,
}

impl Disasm {
    pub fn new() -> Self {
        Self {
            start: 0u16,
            //            end: 63u16,
            range: MIN_DISASM_RANGE,
        }
    }
    /// Set start address of the dump range
    pub fn set_start_address(&mut self, start_addr: u16) {
        self.start = start_addr;
    }
    /// Set end address of the dump range
    //    pub fn set_end_address(&mut self, end_addr: u16) {
    //        self.end = end_addr;
    //    }
    /// Set end address of the dump range
    pub fn set_range(&mut self, range: u16) {
        self.range = range;
    }
}

#[derive(Clone)]
enum InputMode {
    Normal,
    Editing,
}
//
impl Dump {
    pub fn new() -> Self {
        Self {
            start: 0u16,
            end: 127u16,
            range: 128,
        }
    }
    /// Set start address of the dump range
    pub fn set_start_address(&mut self, start_addr: u16) {
        self.start = start_addr;
    }
    /// Set end address of the dump range
    pub fn set_end_address(&mut self, end_addr: u16) {
        self.end = end_addr;
    }
    /// Set end address of the dump range
    pub fn set_range(&mut self, range: u16) {
        self.range = range;
    }
}
/// Implementattion of App
impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            app_state: AppState::Home,
            output_view_status: OutputViewStatus::new(),
            output_scroll_status: ScrollbarState::new(0),
            command_history: CommandHistory::new(),
            cpu: Cpu::None,
            cpu_ui: None,
            dump: Dump::new(),
            disasm: Disasm::new(),
        }
    }
    pub fn get_command_history_size(&self) -> usize {
        self.command_history.command_history_size
    }
    pub fn set_command_history_size(&mut self, size: usize) {
        self.command_history.command_history_size = size;
        if self.command_history.command_history.len() > size {
            self.command_history.command_history =
                self.command_history.command_history[0..size].to_vec();
            self.command_history.command_history_position = COMMAND_HISTORY_SIZE_INIT_INDEX;
        }
    }
    pub fn get_output_view_status(&mut self) -> &mut OutputViewStatus {
        &mut self.output_view_status
    }
    pub fn is_cpu_set(&mut self) -> std::result::Result<AppState, String> {
        if self.cpu_ui.is_some() {
            return Ok(AppState::Home);
        }
        cpu_not_set_error()
    }
    /// Moves cursor in input widget of command line Ui to the left
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }
    /// Moves cursor in input widget of command line Ui to the right
    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }
    /// Input a character to input widget of command line UI
    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    /// Restrict a value to interval [0, input.chars.count]
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
    /// Resets cursor in input widget of command line UI
    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
    /// Pushes command to command buffer
    ///
    /// It keeps maximum size of history either as default set by COMMAND_HISTORY_SIZE
    /// or set by the command "set command_history_size".
    fn history_push_command(&mut self, comm: String) {
        let history = &mut self.command_history.command_history;
        if !comm.is_empty() {
            history.insert(0, comm);
        }
        self.command_history.command_history_position = COMMAND_HISTORY_SIZE_INIT_INDEX;
        let size = self.command_history.command_history_size;
        if history.len() > size {
            *history = history.drain(0..size).collect();
        }
    }
    /// Submits a message from command line UI and renders a ersult
    fn submit_message(&mut self) -> AppState {
        self.history_push_command(self.input.clone());
        let comm = format!("$ {}", self.input.clone());
        self.messages.push(comm.clone());
        let command = self.input.clone();
        self.input.clear();
        self.reset_cursor();
        self.output_view_status.set_line_offset(0);
        match Command::new().command(self, command) {
            Ok(state) => {
                self.output_scroll_status = self.output_scroll_status.position(self.messages.len());
                state
            }
            Err(err) => {
                self.messages.push(err.to_string());
                self.output_scroll_status = self.output_scroll_status.position(self.messages.len());
                AppState::Home
            }
        }
    }
    /// Moves history of commands up
    fn move_command_history_up(&mut self) {
        if let Some(input) = self.command_history.command_history_up() {
            self.input = input;
            self.character_index = self.input.len();
        }
    }
    /// Moves history of commands up
    fn move_command_history_down(&mut self) {
        if let Some(input) = self.command_history.command_history_down() {
            self.input = input;
            self.character_index = self.input.len();
        }
    }
    /// Move output history up
    fn move_output_up_line(&mut self) {
        self.output_view_status
            .set_line_offset(self.output_view_status.get_line_offset() - 1);
    }
    /// Move output history down
    fn move_output_down_line(&mut self) {
        self.output_view_status
            .set_line_offset(self.output_view_status.get_line_offset() + 1);
    }
    /// Move output history up one page
    fn move_output_up_page(&mut self) {
        self.output_view_status
            .set_page_offset(self.output_view_status.get_page_offset() - 1);
    }
    /// Move output history down one page
    fn move_output_down_page(&mut self) {
        self.output_view_status
            .set_page_offset(self.output_view_status.get_page_offset() + 1);
    }

    /// Renders UIs and processes events
    ///
    /// Renders UIs and processes events from terminal (keyboard, mouse)
    /// This is a central part of this application as it renders different UIs
    /// (command UI or opcodes help hor i8080 or mos6502 and others can also be added)
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<AppState> {
        let mut event: Event;
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            event = event::read()?;
            if let Some(Ok(state)) = self.event_handler(event) {
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                return Ok(state);
            }
        }
    }

    /// Handles events from command UI
    ///
    /// Handles evens like events from keyboard and mouse events. Normally it is hardcoded in run function,
    /// but for this implementation we want to process events for multiple UIs so it is called as function
    /// from run function.
    fn event_handler(&mut self, event: Event) -> Option<Result<AppState>> {
        if let Event::Key(key) = event {
            match self.input_mode {
                InputMode::Normal if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('c') => {
                        self.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Some(Ok(AppState::Quit));
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        return Some(Ok(self.submit_message()));
                    }
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => self.input_mode = InputMode::Normal,
                    KeyCode::Up => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            self.move_command_history_up()
                        } else {
                            self.move_output_up_line()
                        }
                    }
                    KeyCode::Down => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            self.move_command_history_down()
                        } else {
                            self.move_output_down_line()
                        }
                    }
                    KeyCode::PageUp => self.move_output_up_page(),
                    KeyCode::PageDown => self.move_output_down_page(),
                    _ => {}
                },
                InputMode::Editing => {}
                InputMode::Normal => {}
            }
        }
        None
    }

    /// Draws "command" UI in terminal
    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, command_area, output_area] = vertical.areas(frame.area());
        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "c".bold(),
                    " to enter command mode.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to leave command mode, ".into(),
                    "Enter".bold(),
                    " to confirm command.".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Command"));
        frame.render_widget(input, command_area);
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                command_area.x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                command_area.y + 1,
            )),
        }
        // Removes first elements from the messages vector so that the it fits to
        // the messages area.
        let output_area_height = output_area.height - 2;
        if self.messages.len() > self.output_view_status.max_lines {
            let start_idx = self.messages.len() - self.output_view_status.max_lines;
            self.messages.drain(0..start_idx);
        }
        // Let's calculate line/page up/down values
        let page_index: i16;
        let page_offset = self.output_view_status.get_page_offset();
        // Calculate lines of page index
        if page_offset != 0 {
            page_index = page_offset * output_area_height as i16;
            self.output_view_status.set_page_offset(0);
            self.output_view_status
                .set_line_offset(self.output_view_status.get_line_offset() + page_index);
        }
        // Now calculate lines of line index
        let mut start_idx: usize = 0;
        if self.messages.len() as u16 > output_area_height {
            start_idx = self.messages.len() - output_area_height as usize;
        }
        let line_offset = self.output_view_status.get_line_offset();
        let idx = start_idx as i16 + line_offset;
        if idx <= 0 {
            // Stop at the bottom of Output window
            self.output_view_status.set_line_offset(-(start_idx as i16));
            start_idx = 0;
        } else if idx < start_idx as i16 {
            // Move Output window up or down
            start_idx = idx as usize;
        } else {
            // Stop at the top of Output window
            self.output_view_status.set_line_offset(0);
        }
        let messages: Vec<ListItem> = self.messages[start_idx..]
            .iter()
            .map(|m| {
                let content = Line::from(Span::raw(m.to_string()));
                ListItem::new(content)
            })
            .collect();
        let messages = &List::new(messages).block(Block::bordered().title("Output"));
        frame.render_widget(messages, output_area);
        // A bit of trial and error calculation of position of scroll thumb position. But it works now.
        let scroll_bar_content_length = if self.messages.len() <= output_area_height as usize {
            0
        } else {
            self.messages.len() - output_area_height as usize
        };
        let mut scrollbar_state =
            ScrollbarState::new(scroll_bar_content_length).position(start_idx);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            output_area,
            &mut scrollbar_state,
        );
    }
}
