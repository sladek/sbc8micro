use crate::commands::memory::Memory;
use crate::disk::Disk;
use crate::ui::app::App;
use crate::ui::app::AppState;
use crate::io::fdhdc;
use crate::disk::hdd8m::Hdd;
use crate::disk::sssd8fd::Floppy;
use crate::io::ErrorIndicators;


pub struct FdHdC;

impl FdHdC {
    pub fn fdhdc(app: &mut App, command: Vec<&str>) -> Result<AppState, String> {
        app.is_cpu_set()?; // Check if cpu is defined
        let cpu = app.cpu_ui.as_mut().unwrap();
        if command.len() > 6 || command.len() < 3 {
            app.messages.push(
                "ERROR - Invalid number of parameters. Usage: fdhdc <address> <disk 1> [disk 2] [disk 3] [disk 4]"
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
        let mut disk_index = 2;
        // Let'a collect names of floppy disks we want to attach to the controller.
        let floppy_char = 'A';
        let mut assigned_disks = String::new();
        let mut disk_char_index = 0;
        let mut fdhdc = Box::new(fdhdc::FdHdC::new(cpu.get_memory_ref())); // Base address 0x78
        let mut disk_number = 0;
        while disk_index < command.len() {
            let disk_file = command[disk_index].to_string();
            let (file_name, ro_flag) = process_disk_name(disk_file.clone());
            let disk_char = char::from_u32(floppy_char as u32 + disk_char_index).unwrap();
            let disk_str = format!(" {disk_char}:{disk_file}");
            assigned_disks.push_str(&disk_str);
            /*
             * First 2 disks (A:, B:) are floppy disks and last disks (C:, D:) are hard disks
             * This shouldn't be changed as it is hardcoded in CBIOS.ASM 
             */
            if disk_number < 2 {
                // Let's process floppy disks firs
                let disk = match Floppy::new(&file_name, ro_flag) {
                    Ok(disk) => { 
                        disk 
                    }
                    Err(err) => {
                        match err {
                            ErrorIndicators::NotReady => {
                                return Err(format!("ERROR[{:?}] - Floppy file: {:?} has different size than expected {:?} bytes. Please check if the file is correct floppy disk image.", err, file_name, Floppy::DISK_CAPACITY));
                            }
                            _ => {
                                return Err(format!("ERROR[{:?}] - Cannot open floppy file: {:?}. Please check if the file name is correct.", err, file_name));
                            }
                        }
                    }
                };
                if let Err(err) = fdhdc.set_disk(fdhdc::DiskTypes::Fdd(disk), disk_number) {
                    match err {
                        ErrorIndicators::AddressError => {
                            return Err(format!("ERROR - File {file_name} has already been assigned.").to_string());
                        }
                        _ => {
                            return Err(format!("ERROR[{:?}] - Cannot set the floppy to {file_name}.", err))
                        }
                    }
                };
            } else {
                // Let's process hard disks
                let disk = match Hdd::new(&file_name, ro_flag) {
                    Ok(disk) => { 
                        disk 
                    }
                    Err(err) => {
                        match err {
                            ErrorIndicators::NotReady => {
                                return Err(format!("ERROR[{:?}] - Floppy file: {:?} has different size than expected {:?} bytes. Please check if the file is correct hard disk image.", err, file_name, Hdd::DISK_CAPACITY));
                            }
                            _ => {
                                return Err(format!("ERROR[{:?}] - Cannot open disk file: {:?}. Please check if the file name is correct.", err, file_name));
                            }
                        }
                    }
                };
                if let Err(err) = fdhdc.set_disk(fdhdc::DiskTypes::Hdd(disk), disk_number) {
                    match err {
                        ErrorIndicators::AddressError => {
                            return Err(format!("ERROR - File {file_name} has already been assigned.").to_string());
                        }
                        _ => {
                            return Err(format!("ERROR[{:?}] - Cannot set the hard disk to {file_name}.", err))
                        }
                    }
                };
            }
            disk_number += 1;
            disk_char_index +=  1;
            disk_index += 1;
        }
        if m_flag == "M" {
            fdhdc.set_memory_base_address(base_address);
        } else {
            fdhdc.set_base_address(base_address as u8)
        };
        let parameters = format!(
            "Parameters: base address[{}], floppy drives:{assigned_disks}",
            command[1]
        );
        app.messages.push(parameters);
        if m_flag == "M" {
            cpu.get_memory().map_port(fdhdc)?;
        } else {
            match cpu.get_io_memory() {
                Some(io_memory) => {
                    io_memory.map_port(fdhdc)?;
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
    use crate::commands::fdhdc::process_disk_name;

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