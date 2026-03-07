use crate::commands::memory::Memory;
use crate::ui::app::App;
use crate::ui::app::AppState;
use crate::io::isbc201::Isbc201;
use crate::disk::sssd8fd::Floppy;

pub struct Fdc;

impl Fdc {
    pub fn fdc(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() > 6 || command.len() < 3 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: fdc <address> <disk 1> [disk 2] [disk 3] [disk 4]"
                    .to_string(),
            );
            return Ok(AppState::Home);
        }
        let mut port_address = command[1].to_uppercase();
        let mut m_flag = "";
        if port_address.starts_with("M") {
            m_flag = "M";
            port_address = port_address[1..].to_string();
        }
        let base_address = Memory::from_hex_string(port_address)?;
        if base_address > 0xff && m_flag.is_empty() {
            app.messages.push(format!(
                "ERROR - Address cannot be bigger than 0xff, but it is 0x{:02X}",
                base_address
            ));
            return Ok(AppState::Home);
        }
        let mut floppy_index = 2;
        // Let'a collect names of floppy disks we want to attach to the controller.
        let floppy_char = 'A';
        let mut assigned_floppies = String::new();
        let mut floppy_char_index = 0;
        let mut fdc = Box::new(Isbc201::new(cpu.get_memory_ref())); // Base address 0x78
        let mut floppu_number = 0;
        while floppy_index < command.len() {
            let floppy_file = command[floppy_index].to_string();
            let (file_name, ro_flag) = process_disk_name(floppy_file.clone());
            let floppy_char = char::from_u32(floppy_char as u32 + floppy_char_index).unwrap();
            let floppy_str = format!(" {floppy_char}:{floppy_file}");
            assigned_floppies.push_str(&floppy_str);
            let floppy = match Floppy::new(&file_name, ro_flag) {
                Ok(floppy) => {
                            floppy
                }
                Err(err) => {
                    return Err(format!("ERROR[{:?}] - Cannot open floppy file: {:?}. Please check if the file name is correct.", err, file_name));
                }
            };
            if fdc.set_floppy(floppy, floppu_number).is_err() {
                return Err(format!("ERROR - File {file_name} has already been assigned").to_string());
            } else {
            floppu_number += 1;
            floppy_char_index += 1;
            floppy_index += 1;
            }
        }
        if m_flag == "M" {
            fdc.set_memory_base_address(base_address);
        } else {
            fdc.set_base_address(base_address as u8)
        };
        let parameters = format!(
            "Parameters: base address[{}], floppy drives:{assigned_floppies}",
            command[1]
        );
        app.messages.push(parameters);
        if m_flag == "M" {
            cpu.get_memory().map_port(fdc)?;
        } else {
            match cpu.get_io_memory() {
                Some(io_memory) => {
                    io_memory.map_port(fdc)?;
                }
                None => {
                    app.messages.push(
                        "ERROR - This CPU doesn't suppor Io mapping, please use memory mapping"
                            .to_string(),
                    );
                }
            }
        }



        Ok(AppState::Home)
    }
}
// Processes filename containing 'read only' flag
//
// File name from command line can contain 'read only' flag in the form of [RO] suffix. 
// this function separates file name and 'read only' flag
fn process_disk_name(name: String) -> (String, bool) {
    let mut read_only: bool = false;
    if name.to_uppercase().ends_with("[RO]") {
        read_only = true;
        let file_name = &name[0 .. name.len() - 4];
        return (String::from(file_name), read_only )

    }
    (name, read_only)
}
#[cfg(test)]
mod tests {
    use crate::commands::fdc::process_disk_name;

    #[test]
    fn test_disk_name() {
        let (name, ro) = process_disk_name("disk1.img".to_string());
        assert_eq!("disk1.img", name);
        assert_eq!(false, ro);
    }
    #[test]
    fn test_disk_name_ro() {
        let (name, ro) = process_disk_name("disk1.img[ro]".to_string());
        assert_eq!("disk1.img", name);
        assert_eq!(true, ro);
    }
}