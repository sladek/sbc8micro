//////////////////////////////////////////////////////////
/// This is the common viewer for op codes. Each CPU has its own draw() function that
/// draws code information to terminal. Dynamic dispatch is used so that new type of CPU
/// Can be added easily
/// Below is an example of usage.
///
/// ```
/// mod disassembler;
/// mod disassembler;
/// mod memory;
///
/// use crate::disassembler::i8080_opcodes::OpcodeView as op_i8080; // Use i8080 opcodes
/// use crate::disassembler::mos6502_opcodes::OpcodeView as op_mos6502; // Use mos6502 opcodes
/// use crate::disassembler::opcode_viewer::view;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let i8080 = true;
///    if i8080 {
///        let op_view = op_i8080::new();
///        view(&op_view)
///    } else {
///        let op_view = op_mos6502::new();
///        view(&op_view)
///    }
/// }
/// ```
//////////////////////////////////////////////////////////
use crate::disassembler::DrawOpcode;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log::LevelFilter;
use ratatui::prelude::CrosstermBackend;
use ratatui::{
    DefaultTerminal, Terminal,
    widgets::{ScrollbarState, TableState},
};

//#[derive(Default)]
pub struct OpcodeViewer<'a, T> {
    view: &'a dyn DrawOpcode<T>,
    scroll_state: ScrollbarState,
    // Note: TableState should be stored in your application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    table_state: TableState,
}

impl<'a, T> OpcodeViewer<'a, T> {
    pub fn table_state(&self) -> TableState {
        self.table_state.clone()
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
        }
    }
    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.view().opcodes().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.view().opcodes().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|frame| self.view().draw(self, frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            return Ok(());
                        }
                        KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn view<T>(view: &dyn DrawOpcode<T>) -> Result<(), Box<dyn std::error::Error>> {
    simple_logging::log_to_file("test.log", LevelFilter::Trace)?;
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let app_result = OpcodeViewer::new(view).run(terminal);
    disable_raw_mode()?;
    ratatui::restore();
    app_result
}
