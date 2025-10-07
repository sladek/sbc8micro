//! # [Ratatui] User Input example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
//mod ui;

//use crossterm::{event, execute, terminal::LeaveAlternateScreen, event::DisableMouseCapture};
use ratatui::{
    Terminal,
    crossterm::{
        event::EnableMouseCapture,
        execute,
        terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};
use sbc8micro::{disassembler::mos6502_opcode, ui::app::{App, AppState}};
use sbc8micro::disassembler::i8080_opcode;
use sbc8micro::disassembler::opcode_viewer::OpcodeViewer;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    enable_raw_mode()?;
    color_eyre::install()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    //    let terminal = ratatui::init();

    
    let mut app = App::new();
    let mut state = AppState::Home;
    loop {
        match state {
            AppState::Home => {
                let _ = terminal.clear();
                state = app.run(&mut terminal)?;
            }
            AppState::Opcodes8080 => {
                let _ = terminal.clear();
                let op8080_view = &mut i8080_opcode::OpcodeView::new();
                let mut i8080_viewer = OpcodeViewer::new(op8080_view);
                state = i8080_viewer.run(&mut terminal)?;
            }
            AppState::Opcodes6502 => {
                let _ = terminal.clear();
                let op6502_view = &mut mos6502_opcode::OpcodeView::new();
                let mut mos6502_viewer = OpcodeViewer::new(op6502_view);
                state = mos6502_viewer.run(&mut terminal)?;
            }
            _ => {
                break
            }
        }       
    }
    disable_raw_mode()?;
    ratatui::restore();
    let _ = terminal.clear();
    println!("Terminal size {:?}", terminal.size());
    Ok(())
}
