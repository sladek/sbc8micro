//! Single side single density 8" floppy disk emulator
//!
//! This is an emulation of single side single density 8" floppy disk as used by Intellec MDS-800 system 
//! which then can be used for running disk operating system like CP/M 80 in sbc8micro emulator
use std::fs::File;
use std::io::{ErrorKind, Write};
use crate::io::ErrorIndicators;
use crate::disk::Result;
use crate::disk::Disk;

#[derive(Clone)]
pub enum DataDeletedData {
    Data = 0x0B,
    DeletedData = 0x08,
}

// Hdd disk file
pub struct Hdd {
    name: String,
    read_only: bool,
    disk: File,
}

impl Hdd {

    pub fn new(name: &str, read_only: bool) -> Result<Self> {
        let file = Self::open_file_image(name, read_only);
        let mut is_new = false;
        let floppy = match file {
            Ok(file) => {
                if file.metadata().unwrap().len() != Self::DISK_CAPACITY as u64 {
                    // If file has different size than the size for Floppy return NotReady indicator
                    return Err(ErrorIndicators::NotReady);
                };
                file
            },
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    is_new = true;
                    // File doesn't exist, let's create the empty one.
                    match File::create(name) {
                        Ok(mut file) => {
                            if file.write_all(&[0x55u8; Hdd::DISK_CAPACITY]).is_err() {
                                return Err(ErrorIndicators::WriteError);
                            };
                            // We need to reopen a file as RO or RW based on input parameters so we drop now
                            // the freshly created file
                            drop(file);
                            // And reopen it again
                            match Self::open_file_image(name, read_only) {
                                Ok(file) => {
                                    file
                                }
                                Err(_) => {
                                    return Err(ErrorIndicators::NotReady);
                                }
                            }
                        }
                        Err(_) => {
                            return Err(ErrorIndicators::NotReady);
                        }
                    }
                } else {
                    return Err(ErrorIndicators::WriteProtect);
                }
            }
        };
        let mut floppy = Self {
            name: name.to_string(),
            read_only,
            disk: floppy,
        };
        if is_new {
            // File is new, let's try to format it
            let format_result = floppy.format();
            if format_result.is_err() {
                return Err(format_result.err().unwrap());
            }

        };
        Ok(floppy)
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
}

impl Disk for Hdd {
    const NUM_OF_SECTORS_PER_TRACK: u8 = 255; // Valid number of sector is 1 - 255
    const NUM_OF_TRACKS: u8 = 255; // Valid number of track is 0 - 255
    const DISK_CAPACITY: usize = 12614850;

    fn get_disk(&self) -> &File {
        &self.disk
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
#[cfg(test)]
mod tests {
    use std::fs;
    use crate::disk::{Disk, Sector, hdd8m::{ErrorIndicators, Hdd}};

    fn init_disk(file_name: &str) {
        _ = fs::remove_file(file_name);
        // Let's create new floppy image and format
        _ = Hdd::new(file_name, false).unwrap().format();
    }    
    fn remove_disk(file_name: &str) {
        _ = fs::remove_file(file_name);
    }

    #[test]
    fn test_format() {
        let file_name = "disks/test_hd.img";
        init_disk(file_name);
        let mut floppy = Hdd::new(file_name, false).unwrap();
        let result = Hdd::format(&mut floppy);
        assert_eq!(true, result.is_ok());
        // Read first sector and compare CRC.
        let sector = floppy.read_sector(0, 1).unwrap();
        assert_eq!(sector.crc_id, sector.crc_id());
        assert_eq!(sector.crc_data, sector.crc_data());
        remove_disk(file_name);
    }
    #[test]
    fn test_new_file() {
        let file_name = "disks/test_hd_1.img";
        init_disk(file_name);
        let floppy = Hdd::new(file_name, false).unwrap();
        let len = floppy.disk.metadata().unwrap().len();
        assert_eq!(len as usize, Hdd::DISK_CAPACITY);
        remove_disk(file_name);
    }
    #[test]
    fn test_read() {
        let file_name = "disks/test_hd_2.img";
        init_disk(file_name);
        let floppy = Hdd::new(file_name, true).unwrap();
        let sector = floppy.read_sector(254, 255).unwrap();
        remove_disk(file_name);
        assert_eq!(sector.crc_id, sector.crc_id());
        assert_eq!(sector.crc_data, sector.crc_data());
    }
    #[test]
    fn test_read_invalid_track_0() {
        let file_name = "disks/test_hd_3.img";
        init_disk(file_name);
        let floppy = Hdd::new(file_name, true).unwrap();
        let res = floppy.read_sector(0, 1);
        remove_disk(file_name);
        assert!(res.is_ok());
    }
    #[test]
    fn test_read_invalid_track_255() {
        let file_name = "disks/test_hd_4.img";
        init_disk(file_name);
        let floppy = Hdd::new(file_name, true).unwrap();
        remove_disk(file_name);
        let res = floppy.read_sector(255, 255);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
    #[test]
    fn test_read_invalid_sector_0() {
        let file_name = "disks/test_hd_5.img";
        init_disk(file_name);
        let floppy = Hdd::new(file_name, true).unwrap();
        let res = floppy.read_sector(254, 0);
        remove_disk(file_name);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
    #[test]
    fn test_write_last_sector() {
        let track_nr = 254;
        let sector_nr = 255;
        let file_name = "disks/test_hd_6.img";
        init_disk(file_name);
        let mut floppy = Hdd::new(file_name, false).unwrap();
        let data = [0xff; 128];
        let sector = Sector::new(track_nr, sector_nr, &data);
        _ = floppy.seek_write_sector(sector);
        let sector = floppy.read_sector(track_nr, sector_nr).unwrap();
        remove_disk(file_name);
        assert_eq!(sector.crc_id, sector.crc_id());
        assert_eq!(sector.crc_data, sector.crc_data());
    }
    #[test]
    fn test_format_track_ok() {
        let file_name = "test_hd_7.img";
        init_disk(file_name);
        let mut disk = Hdd::new(file_name, false).unwrap();
        let res = disk.format_track(1);
        remove_disk(file_name);
        assert_eq!(Ok(()), res);
    }
    #[test]
    fn test_format_track() {
        let big_track_number = 255;
        let file_name = "test_hd_8.img";
        init_disk(file_name);
        let mut disk = Hdd::new(file_name, false).unwrap();
        let res = disk.format_track(big_track_number);
        remove_disk(file_name);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
}
