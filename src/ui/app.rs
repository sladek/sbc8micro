use crate::commands::command::Command;
use crate::cpu::{Cpu, CpuUi};



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
    output_view_status: ViewStatus,
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
#[derive(Default)]
pub struct ViewStatus {
    // Moves output windows up and down based on offset
    // If +x moves up x lines, if -x moves down x lines
    line_offset: i16,
    page_offset: i16,
    max_lines: u16,
}
/// Keeps data about view, like start_line
/// 
/// The view will start on the line specified in start_line.
impl ViewStatus {
    pub fn new() -> Self{
        Self {
            line_offset: 0,
            page_offset: 0,
            max_lines: 500,
        }
    }
    pub fn set_line_offset(&mut self, offset: i16) {
        self.line_offset = offset;
    }
    pub fn get_line_offset(&self) -> i16{
        self.line_offset
    }
    pub fn set_page_offset(&mut self, offset: i16) {
        self.page_offset = offset;
    }
    pub fn get_page_offset(&self) -> i16 {
        self.page_offset
    }
}

#[derive(Debug, Clone)]
pub enum AppState {
    Home,
    Opcodes8080,
    Opcodes6502,
    Quit,
}
#[derive(Default)]
pub struct Dump {
    pub start: u16,
    pub end: u16,
    pub range: u16,
}
#[derive(Default)]
pub struct Disasm {
    pub start: u16,
    pub end: u16,
    pub range: u16,
}

impl Disasm {
    pub fn new() -> Self {
        Self {
            start: 0u16,
            end: 63u16,
            range: 64,
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
            output_view_status: ViewStatus::new(),
            cpu: Cpu::None,
            cpu_ui: None,
            dump: Dump::new(),
            disasm: Disasm:: new(),
        }
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
    /// Submits a message from command line UI and renders a ersult
    fn submit_message(&mut self) -> AppState {
        let comm = format!("$ {}", self.input.clone());
        self.messages.push(comm);
        let command = self.input.clone();
        self.input.clear();
        self.reset_cursor();
        self.output_view_status.set_line_offset(0);
        match Command::new().command(self, command) {
            Ok(state) => state,
            Err(err) => {
                self.messages.push(err.to_string());
                AppState::Home
            }
        }
    }
    /// Move output history up
    fn move_output_up_line(&mut self) {
        self.output_view_status.set_line_offset(self.output_view_status.get_line_offset() - 1);
    }
    /// Move output history down
    fn move_output_down_line(&mut self) {
        self.output_view_status.set_line_offset(self.output_view_status.get_line_offset() + 1);
    }
    /// Move output history up one page
    fn move_output_up_page(&mut self) {
         self.output_view_status.set_page_offset(self.output_view_status.get_page_offset() - 1);
    }
    /// Move output history down one page
    fn move_output_down_page(&mut self) {
        self.output_view_status.set_page_offset(self.output_view_status.get_page_offset() + 1);
    }

    /// Renders UIs and processes events
    ///
    /// Renders UIs and processes events from terminal (keyboard, mouse)
    /// This is a central part of this application as it renders different UIs
    /// (command UI or opcodes help hor i8080 or mos6502 and others can also be added)
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<AppState> {
        let mut event: Event;
        loop {
            terminal.draw(|frame| {
                self.draw(frame)
            })?;
            event = event::read()?;
            if let Some(Ok(state)) = self.event_handler(event) {
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                return  Ok(state);
            }
            
        }
    }

/*

    /// Renders UIs and processes events
    ///
    /// Renders UIs and processes events from terminal (keyboard, mouse)
    /// This is a central part of this application as it renders different UIs
    /// (command UI or opcodes help hor i8080 or mos6502 and others can also be added)
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let op8080_view = &mut i8080_opcode::OpcodeView::new();
        let mos6502_view = &mut mos6502_opcode::OpcodeView::new();
        let mut i8080_viewer = OpcodeViewer::new(op8080_view);
        let mut mos6502_viewer = OpcodeViewer::new(mos6502_view);
        let mut event: Event;
        loop {
            terminal.draw(|frame| match self.app_state {
                AppState::Home => self.draw(frame),
                AppState::Asm8080 => op8080_view.draw(&i8080_viewer, frame),
                AppState::Asm6502 => mos6502_view.draw(&mos6502_viewer, frame),
            })?;
            event = event::read()?;
            match self.app_state {
                AppState::Home => {
                    if let Some(Ok(())) = self.event_handler(event) {
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        return Ok(());
                    }
                }
                AppState::Asm8080 => {
                    if let Some(Ok(())) = i8080_viewer.event_handler(&mut self.app_state, event) {
                        self.app_state = AppState::Home;
                    }
                }
                AppState::Asm6502 => {
                    if let Some(Ok(())) = mos6502_viewer.event_handler(&mut self.app_state, event) {
                        self.app_state = AppState::Home;
                    }
                }
            }
        }
    }


*/

    /// Handles events from command UI
    ///
    /// Handles evens like events from keyboard and mouse events. Normally it is hardcoded in run function,
    /// but for this implementation we want to process events for multiple UIs so it is called as function
    /// from run function.
    fn event_handler(&mut self, event: Event) -> Option<Result<AppState>> {
        if let Event::Key(key) = event {
            match self.input_mode {
                InputMode::Normal if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('e') => {
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
                    KeyCode::Up => self.move_output_up_line(),
                    KeyCode::Down => self.move_output_down_line(),
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
        let [help_area, input_area, messages_area] = vertical.areas(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
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
        frame.render_widget(input, input_area);
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
        }
        // Removes first elements from the messages vector so that the it fits to
        // the messages area.
        let msg_area_height = messages_area.height - 2;
        if self.messages.len() as u16 > self.output_view_status.max_lines {
            let start_idx = self.messages.len() - self.output_view_status.max_lines as usize;
            self.messages.drain(0..start_idx);
        }
        // Let's calculate line/page up/down values
        let page_index: i16;
        let page_offset = self.output_view_status.get_page_offset();
        // Calculate lines of page index
        if page_offset != 0 {
            page_index = page_offset * msg_area_height as i16;
            self.output_view_status.set_page_offset(0);
            self.output_view_status.set_line_offset(self.output_view_status.get_line_offset() + page_index);
        }
        // Now calculate lines of line index
        let mut start_idx: usize = 0;
        if self.messages.len() as u16 > msg_area_height{
            start_idx = self.messages.len() - msg_area_height as usize;
        }
        let line_offset = self.output_view_status.get_line_offset();
        let idx =  start_idx as i16 + line_offset;
        if idx <= 0 {
            // Stop at the bottom of Output window
            self.output_view_status.set_line_offset(-(start_idx as i16));
            start_idx = 0;
        } else if idx < start_idx as i16{
            // Move Output window up or down
            start_idx = idx as usize;
        } else {
            // Stop at the top of Output window
            self.output_view_status.set_line_offset(0);
        }
        let messages: Vec<ListItem> = self
            .messages[start_idx ..]
            .iter()
            .map(|m| {
                let content = Line::from(Span::raw(m.to_string()));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Output"));
        frame.render_widget(messages, messages_area);
    }
}
