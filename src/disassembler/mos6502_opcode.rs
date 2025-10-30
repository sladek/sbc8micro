//! Opcode for MOS Technology mos6502 CPU
use crate::disassembler::{DrawOpcode, mos6502_opcodes::OPCODES, opcode_viewer::OpcodeViewer};
use ratatui::{
    Frame,
    layout::Layout,
    prelude::{Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
};

#[derive(Default, Debug, Clone, serde::Deserialize)]
pub struct Opcode {
    /// Opcode as an hexadecimal string - "0F", "55", "AA", ...
    opcode: String,
    /// Mnemonic -  "ADC oper", "ASL oper,X", "CMP (oper),Y", ...
    mnemonic: String,
    /// Mode - "immediate", "relative", "absolute", "zeropage,X", "absolute,X", ...
    mode: String,
    /// Number of bytes the instruction occupies in memory
    bytes: u8,
    /// Number of CPU cycles needed to execute the instruction
    cycles: String,
    /// Description of instruction
    description: Option<String>,
}

#[derive(Debug, Default)]
pub struct OpcodeView<Opcode> {
    opcodes: Vec<Opcode>,
}

impl OpcodeView<Opcode> {
    /// Parses OPCODES and returns an instance of opcode view
    pub fn new() -> Self {
        Self {
            opcodes: serde_json::from_str(OPCODES).unwrap(),
        }
    }
}

impl DrawOpcode<Opcode> for OpcodeView<Opcode> {
    /// Returns a vector of opcodes
    fn opcodes(&self) -> &Vec<Opcode> {
        &self.opcodes
    }
    /// Find index of the opcode where mnemonic starts with character ch.
    fn find_index_by_char(&self, ch: char) -> Option<usize> {
        self.opcodes
            .iter()
            .position(|opcode| opcode.mnemonic.starts_with(ch))
    }
    /// Shows opcode description in a frame
    fn draw(&self, viewer: &OpcodeViewer<Opcode>, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(9)].as_ref())
            .split(frame.area());
        let selected = viewer.table_state().selected();
        let rows: Vec<Row> = viewer
            .view()
            .opcodes()
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
                    Cell::from(op.bytes.to_string()),
                    Cell::from(op.cycles.clone()),
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
                Constraint::Length(6),
                Constraint::Length(7),
            ],
        )
        .header(
            Row::new(vec!["Opcode", "Mnemonic", "Mode", "Bytes", "Cycles"])
                .style(Style::default().fg(Color::Green)),
        )
        .block(
            Block::default()
                .title("mos6502 Opcodes")
                .borders(Borders::ALL),
        );
        frame.render_stateful_widget(table, chunks[0], &mut viewer.table_state());
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunks[0],
            &mut viewer.scroll_state(),
        );
        let selected_row = match viewer.table_state().selected() {
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
