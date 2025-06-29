//////////////////////////////////////////////////////////
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
use crate::disassembler::{Opcode, load_opcodes};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log::LevelFilter;
use ratatui::prelude::CrosstermBackend;
use ratatui::{
    DefaultTerminal, Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table, TableState,
    },
};

#[derive(Default)]
pub struct OpcodeViewer {
    opcodes: Vec<Opcode>,
    scroll_state: ScrollbarState,
    // Note: TableState should be stored in your application state (not constructed in your render
    // method) so that the selected row is preserved across renders
    table_state: TableState,
}

impl OpcodeViewer {
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        let data_vec = opcodes;
        Self {
            scroll_state: ScrollbarState::new(data_vec.len() - 1),
            opcodes: data_vec,
            table_state: TableState::default().with_selected(0),
        }
    }
    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.opcodes.len() - 1 {
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
                    self.opcodes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

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

    #[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(9)].as_ref())
            .split(frame.area());
        let selected = self.table_state.selected();
        let rows: Vec<Row> = self
            .opcodes
            .iter()
            .enumerate()
            .map(|(i, op)| {
                let style = match selected {
                    Some(row) if row == i => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                };
                Row::new(vec![
                    Cell::from(op.opcode.clone()),
                    Cell::from(op.mnemonic.clone()),
                    Cell::from(op.mode.clone()),
                    Cell::from(format!("{}", op.bytes)),
                    Cell::from(format!("{}", op.cycles)),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(8),
                Constraint::Length(15),
                Constraint::Length(18),
                Constraint::Length(8),
                Constraint::Length(6),
            ],
        )
        .header(
            Row::new(vec!["Opcode", "Mnemonic", "Mode", "Bytes", "Cycles"])
                .style(Style::default().fg(Color::Green)),
        )
        .block(Block::default().title("6502 Opcodes").borders(Borders::ALL));
        frame.render_stateful_widget(table, chunks[0], &mut self.table_state);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunks[0],
            &mut self.scroll_state,
        );
        let selected_row = match self.table_state.selected() {
            Some(row) => row,
            _ => usize::MAX,
        };
        let description_text = self.opcodes[selected_row]
            .description
            .clone()
            .unwrap_or_else(|| "No description available.".to_string());
        let desc_block = Paragraph::new(description_text)
            .block(Block::default().title("Description").borders(Borders::ALL)); // vertical scrolling;
        frame.render_widget(desc_block, chunks[1]);
    }
}
pub fn view(opcodes: &str) -> Result<(), Box<dyn std::error::Error>> {
    simple_logging::log_to_file("test.log", LevelFilter::Trace)?;
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let app_result = OpcodeViewer::new(load_opcodes(opcodes)).run(terminal);
    disable_raw_mode()?;
    ratatui::restore();
    app_result
}
