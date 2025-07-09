//////////////////////////////////////////////////////////
use crate::disassembler::Draw;
/// Usage of the viewer is simple. Just provide <OPCODES> string
/// And call view. Below is an example.
///
/// ```
/// mod disassembler;
///
/// use crate::disassembler::mos6502_opcodes::OPCODES;
/// use crate::disassembler::opcode_viewer::view;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     view(OPCODES)
/// }
/// ```
//////////////////////////////////////////////////////////
//use crate::disassembler::{Opcode, load_opcodes};
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
pub struct OpcodeViewer<T> {
    view: Box<dyn Draw<T>>,
    scroll_state: ScrollbarState,
    // Note: TableState should be stored in your application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    table_state: TableState,
}

impl<T> OpcodeViewer<T> {
    pub fn table_state(&self) -> TableState {
        self.table_state.clone()
    }

    pub fn scroll_state(&self) -> ScrollbarState {
        self.scroll_state.clone()
    }
    pub fn view(&self) -> &Box<dyn Draw<T>> {
        &self.view
    }
    pub fn new(view: Box<dyn Draw<T>>) -> Self {
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

//pub fn view<T: Draw<T>>(opcodes: Vec<T>) -> Result<(), Box<dyn std::error::Error>> {
pub fn view<T>(view: Box<dyn Draw<T>>) -> Result<(), Box<dyn std::error::Error>> {
    simple_logging::log_to_file("test.log", LevelFilter::Trace)?;
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    /*
       let codes = match opcodes {
           Ok(value) => value,
           Err(err) => {
               _ = disable_raw_mode();
               ratatui::restore();
               panic!("cannot load opcode: [{}]", err.to_string())
           }
       };
    */
    let app_result = OpcodeViewer::new(view).run(terminal);
    disable_raw_mode()?;
    ratatui::restore();
    app_result
}
