//! Generic op code viewer
//!
//! This is the common viewer for op codes. Each CPU has its own draw() function that
//! draws code information to a terminal. Dynamic dispatch is used so that new type of CPU
//! can be added easily.
//! Below is an example of usage.
//!
//! ```no_run
//! use sbc8micro::disassembler::i8080_opcode::OpcodeView as op_i8080; // Use i8080 opcodes
//! use sbc8micro::disassembler::mos6502_opcode::OpcodeView as op_mos6502; // Use mos6502 opcodes
//! use sbc8micro::disassembler::opcode_viewer::view;
//!
//! fn opcode_viewer() -> Result<(), Box<dyn std::error::Error>> {
//!    let i8080 = true;
//!    if i8080 {
//!        let op_view = op_i8080::new();
//!        view(&op_view)
//!    } else {
//!        let op_view = op_mos6502::new();
//!        view(&op_view)
//!    }
//! }
//! ```
//!
use crate::{disassembler::DrawOpcode, ui::app::AppState};
use ratatui::{
    DefaultTerminal, Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
    widgets::{ScrollbarState, TableState},
};

use color_eyre::Result;

const DESCRIPTION_HEIGHT: u16 = 12; //12 rows is Description area including borders;

pub struct OpcodeViewer<'a, T> {
    /// View of opcode that is drawn
    view: &'a dyn DrawOpcode<T>,
    /// Status of scroll bar
    scroll_state: ScrollbarState,
    // Note: TableState should be stored in your application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    table_state: TableState,
    opcodes_page_size: u16,
}

impl<'a, T> OpcodeViewer<'a, T> {
    pub fn table_state(&self) -> TableState {
        self.table_state
    }

    pub fn scroll_state(&self) -> ScrollbarState {
        self.scroll_state
    }
    pub fn view(&self) -> &dyn DrawOpcode<T> {
        self.view
    }
    pub fn new(view: &'a dyn DrawOpcode<T>) -> Self {
        let len = view.opcodes().len();
        Self {
            view,
            scroll_state: ScrollbarState::new(len - 1),
            table_state: TableState::default().with_selected(0),
            opcodes_page_size: 0,
        }
    }
    fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.view().opcodes().len() - 1 {
                    i // stay at the end
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.scroll_state = self.scroll_state.position(i);
        self.set_page(i);
        // Set selected after set_page as set page sets selected to the beginning of the page
        // and in case of up down only we want to keep it within the page
        self.table_state.select(Some(i));
    }

    fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.scroll_state = self.scroll_state.position(i);
        self.set_page(i);
        // Set selected after set_page as set page sets selected to the beginning of the page
        // and in case of up down only we want to keep it within the page
        self.table_state.select(Some(i));
    }
    fn next_page(&mut self) {
        self.table_state.scroll_down_by(self.opcodes_page_size);
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.view().opcodes().len() - 1 {
                    self.view().opcodes().len() - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.scroll_state = self.scroll_state.position(i);
        self.set_page(i);
    }

    fn previous_page(&mut self) {
        self.table_state.scroll_up_by(self.opcodes_page_size);
        let i = self.table_state.selected().unwrap_or_default();
        self.scroll_state = self.scroll_state.position(i);
        self.set_page(i);
    }
    /// Calculate in which page the selected row is positioned and set
    /// initial index of the opcodes table for displaying in opcodes section
    fn set_page(&mut self, selected: usize) {
        let page_size = self.opcodes_page_size;
        let page_offset = (selected / page_size as usize) * page_size as usize;
        self.table_state.select(Some(page_offset));
        let offset = self.table_state.offset_mut();
        *offset = page_offset;
    }
    /// Sets selected row in opcodes table based on mnemonic starting witch character 'ch'
    fn set_index_by_char(&mut self, ch: char) {
        if let Some(index) = self.view().find_index_by_char(ch) {
            self.set_page(index);
            // Set page sets "selected" to the beginning of pagw
            // so let's reselect the required row
            self.table_state.select(Some(index));
        }
    }
    /// Runs opcode viewer
    ///
    /// This function can be used as standalone as can be seen in the examples
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<AppState> {
        loop {
            terminal.draw(|frame| self.view().draw(self, frame))?;
            self.opcodes_page_size = terminal.size()?.height - DESCRIPTION_HEIGHT; //12 rows is Description area including borders;
            let event = event::read()?;
            if let Some(Ok(())) = self.event_handler(event) {
                execute!(
                    terminal.backend_mut(),
                    EnterAlternateScreen,
                    DisableMouseCapture
                )?;
                return Ok(AppState::Home);
            }
        }
    }
    /// Processes events
    ///
    /// Processes events relevant for this viewer
    pub fn event_handler(&mut self, event: Event) -> Option<Result<()>> {
        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Esc => {
                    return Some(Ok(()));
                }
                KeyCode::Down => self.next_row(),
                KeyCode::Up => self.previous_row(),
                KeyCode::PageDown => self.next_page(),
                KeyCode::PageUp => self.previous_page(),
                KeyCode::Char(ch) => {
                    let ch = ch.to_ascii_uppercase();
                    if ch.is_ascii_uppercase() {
                        self.set_index_by_char(ch)
                    }
                }
                _ => {}
            }
        }

        None
    }
}

/// Renders an opcode viewre on terminal
pub fn view<T>(view: &dyn DrawOpcode<T>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = &mut Terminal::new(backend)?;
    let _ = OpcodeViewer::new(view).run(terminal);
    disable_raw_mode()?;
    ratatui::restore();
    Ok(())
}
