//! Single side single density 8" floppy disk emulator
//!
//! This is an emulation of single side single density 8" floppy disk as used by Intellec MDS-800 system which then can be used for running disk operating system like CP/M 80 in sbc8micro emulator
use crc::CRC_16_IBM_3740;
use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use std::io;
use std::os::windows::fs::FileExt;
use crate::io::ErrorIndicators;

const NUM_OF_SECTORS_PER_TRACK: u8 = 26; // Valid number of sector is 1 - 26
const NUM_OF_TRACKS: u8 = 77; // Valid number of track is 0 - 76
const DATA_SIZE: usize = 128;
const ID_ADDRESS_MARK: u8 = 0xFE; // ID Address Mark identifier byte
const SECTOR_SIZE: u16 = 195; // Number of bytes in one complete sector
const FORMAT_PATTERN: u8 = 0xE5;
const FLOPPY_CAPACITY: usize = 388388; // Full capacitu of 8" floppy disk

#[derive(Clone)]
pub enum DataDeletedData {
    Data = 0x0B,
    DeletedData = 0x08,
}
//Define our own result
pub type Result<T> = core::result::Result<T, ErrorIndicators>;

/// Sector have the following format
/// | 'ID' ADDRESS MARK (1 byte) | 'TRACK ADDRESS' (1 byte) | 0 | 'SECTOR ADDRESS' (1 byte) | 0 | CRC CHECK BITS | GAP (28 bytes) | 'DATA/DELETED DATA' ADDRESS MARK (1 byte) | DATA (128 bytes) | CRC CHECK BITS | GAP (28 bytes) |
///
#[derive(Debug, PartialEq, Clone)]
pub struct Sector {
    id: u8, // 'ID' address mark
    track_address: u8,
    sector_address: u8,
    crc_id: u16,
    data_deleted_data: u8, // 'DATA/DELETED DATA' address mark
    data: [u8; DATA_SIZE],
    crc_data: u16,
}

impl Sector {
    pub fn new(track_address: u8, sector_address: u8, data: &[u8; DATA_SIZE]) -> Self {
        Sector {
            track_address,
            sector_address,
            data: *data,
            ..Default::default()
        }
    }
    /// Calculate CRC
    ///
    /// Calculates CRC. The algorithm CRC_16_IBM_3740 is chosen as I couldn't find other implementation
    /// But for emulator it is not that important as it is not used by the CPU. It is used by the controller to verify
    /// data consistency. And it will also be used to verify if disk is valid disk. So not only size of the disk is used for verification
    /// but also the validity of the first sector is verified.
    fn crc(data: &[u8]) -> u16 {
        let crc = crc::Crc::<u16>::new(&CRC_16_IBM_3740);
        let mut digest = crc.digest();
        digest.update(data);
        digest.finalize()
    }
    /// Calculate CRC of id field
    ///
    /// Calculates CRC from 'ID' ADDRESS MARK, TRACK ADDRESS and SECTOR ADDRESS
    fn crc_id(&self) -> u16 {
        let id_data = [self.id, self.track_address, 0, self.sector_address, 0];
        Self::crc(&id_data)
    }
    /// Calculate CRC of data
    ///
    /// Calculates CRC from 'DATA/DELETED DATA' ADDRESS MARK and DATA bytes
    fn crc_data(&self) -> u16 {
        let ext_data: Vec<_> = [self.data_deleted_data]
            .into_iter()
            .chain(self.data)
            .collect();
        Self::crc(&ext_data)
    }
    /// Get sector data
    /// 
    /// Returns data[128] from sector
    pub fn get_data(&self) -> [u8; DATA_SIZE] {
        self.data
    }
    /// Sets data_deleted_data mark
    pub fn set_data_deleted_data(&mut self, data_deleted_data: DataDeletedData) {
        self.data_deleted_data = data_deleted_data as u8;
    }
    /// Sets data_deleted_data mark
    pub fn get_data_deleted_data(&mut self) -> u8 {
        self.data_deleted_data
    }
}
// Default implementation for Sectoe
impl Default for Sector {
    fn default() -> Self {
        Self {
            id: ID_ADDRESS_MARK,
            track_address: 0,
            sector_address: 0,
            crc_id: 0,
            data_deleted_data: DataDeletedData::Data as u8,
            data: [0; DATA_SIZE],
            crc_data: 0,
        }
    }
}

// Floppy disk file
pub struct Floppy {
    name: String,
    read_only: bool,
    disk: File,
}

impl Floppy {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
}
/*
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorIndicators {
    NotReady = 0x80,                           // Disk is not ready
    WriteError = 0x40,                         // Error occured during writing to the disk
    WriteProtect = 0x20,                       // Disk is write protected
    OverUnderRun = 0x10, // Controler couldn't transfer the data before next request. Data is lost
    AddressError = 0x08, // Invalid sector or track is requested by CPU
    SeekError = 0x04,    // Head is not positioned over expected track
    CrcError = 0x02,     // CRCs calculated are not the same as specified in sector.
    DeletedRecord = 0x01, // Sector has deleted data address mark
    IdCrcError = 0x08 | 0x02, // AddressError | CrcError. Indicates that the CRC of ID field doesn't match
    NoAddressMark = 0x08 | 0x04 | 0x02, // AddressError | SeekError | CrcError. No address mark was encountered for a full revolution of the diskette.
    DataMarkError = 0x08 | 0x04 | 0x02 | 0x01, // // AddressError | SeekError | CrcError | Deleted Record. It indicates that the data field wos not preceded by eitherdata mark or a delete data mark.
}
*/
impl Floppy {
    pub fn new(name: &str, read_only: bool) -> Result<Self> {
        let file = Self::open_file_image(name, read_only);
        let mut is_new = false;
        let floppy = match file {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    is_new = true;
                    // File doesn't exist, let's create the empty one.
                    match File::create(name) {
                        Ok(mut file) => {
                            if file.write_all(&[0x55u8; FLOPPY_CAPACITY]).is_err() {
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

    fn open_file_image(name: &str, read_only: bool) -> io::Result<File> {
        let file: std::io::Result<File> = if read_only {
            std::fs::OpenOptions::new().read(true).open(name)
        } else {
            std::fs::OpenOptions::new()
                .write(true)
                .read(true)
                .open(name)
        };
        file
    }
    /// Format floppy disk.
    ///
    /// Formats existing disk or creates new formated disk if it doesn't exists.    
    pub fn format(&mut self) -> Result<()> {
        if self.disk.seek(std::io::SeekFrom::Start(0)).is_err() {
            return Err(ErrorIndicators::SeekError);
        };
        let data = [FORMAT_PATTERN; 128];
        for track in 0..NUM_OF_TRACKS {
            for sector in 1..=NUM_OF_SECTORS_PER_TRACK {
                Self::write_new_sector(
                    self,
                    Sector {
                        track_address: track,
                        sector_address: sector,
                        data_deleted_data: 0x7b,
                        data,
                        ..Default::default()
                    },
                )?
            }
        }
        Ok(())
    }
    /// Format one track of floppy disk
    ///
    /// Formats one track of the existing floppy disk file.
    pub fn format_track(&mut self, track: u8) -> Result<()> {
        Self::check_ranges(track, 1)?;
        let data = [FORMAT_PATTERN; 128];
        let offset = Self::seek_offset(track, 1);
        if self
            .disk
            .seek(std::io::SeekFrom::Start(offset as u64))
            .is_err()
        {
            return Err(ErrorIndicators::SeekError);
        }
        for sector in 1..=NUM_OF_SECTORS_PER_TRACK {
            Self::write_new_sector(
                self,
                Sector {
                    track_address: track,
                    sector_address: sector,
                    data_deleted_data: 0x7b,
                    data,
                    ..Default::default()
                },
            )?
        }
        Ok(())
    }
    /// Write a sector on current seek position
    ///
    /// Writes one sector to specific floppy file. It wraps write_sector_to_disk function as it returns std::io::Result which needs to be translated to Result<(), ErrorIndicators>
    fn write_new_sector(&mut self, sector: Sector) -> Result<()> {
        match self.write_sector_to_disk(sector) {
            Ok(()) => Ok::<(), ErrorIndicators>(()),
            Err(_) => Err(ErrorIndicators::WriteError),
        }
    }
    /// Write a sector on position defined in sector
    ///
    /// Writes one sector to specific floppy file. Calculates offset in the file and writes new sector to that position.
    /// Disk has to exist and needs to be formated or SeekError is generated.
    pub fn seek_write_sector(&mut self, sector: Sector) -> Result<()> {
        Self::check_ranges(sector.track_address, sector.sector_address)?;
        let offset = Floppy::seek_offset(sector.track_address, sector.sector_address);
        if self
            .disk
            .seek(std::io::SeekFrom::Start(offset as u64))
            .is_err()
        {
            return Err(ErrorIndicators::SeekError);
        }
        match self.write_sector_to_disk(sector) {
            Ok(()) => Ok::<(), ErrorIndicators>(()),
            Err(_) => Err(ErrorIndicators::WriteError),
        }
    }
    /// Seek track
    /// 
    /// Moves to the beginning of track specified by track_number
    pub fn seek(&mut self, track_number: u8) -> Result<()> {
        Self::check_ranges(track_number, 1)?;
        let offset = Floppy::seek_offset(track_number, 1);
        if self
            .disk
            .seek(std::io::SeekFrom::Start(offset as u64))
            .is_err()
        {
            return Err(ErrorIndicators::SeekError);
        }
        Ok(())
    }
    /// Write one sector to disk
    ///
    /// Writes one sector to specific file on the harddrive.
    fn write_sector_to_disk(&mut self, sector: Sector) -> std::io::Result<()> {
        let gap = [0u8; 28];
        // id_data array is used for CRC calculation
        self.disk
            .write_all(&[sector.id, sector.track_address, 0, sector.sector_address, 0])?;
        self.disk.write_all(&sector.crc_id().to_be_bytes())?;
        self.disk.write_all(&gap)?;
        self.disk.write_all(&[sector.data_deleted_data])?;
        self.disk.write_all(&sector.data)?;
        self.disk.write_all(&sector.crc_data().to_be_bytes())?;
        self.disk.write_all(&gap)
    }
    /// Seak offset on the disk
    ///
    /// Seeks an offset on the disk based on track and sector number
    fn seek_offset(track_nr: u8, sector_nr: u8) -> u32 {
        (track_nr as u32 * (NUM_OF_SECTORS_PER_TRACK as u32) * (SECTOR_SIZE as u32 - 1))
            + ((sector_nr - 1) as u32 * (SECTOR_SIZE as u32 - 1))
    }
    /// Check track and sector ranges
    ///
    /// Check if track numbers or sector numbers are in the allowed range
    fn check_ranges(track: u8, sector: u8) -> Result<()> {
        if track > NUM_OF_TRACKS - 1 {
            return Err(ErrorIndicators::SeekError);
        }
        if sector == 0 || sector > NUM_OF_SECTORS_PER_TRACK {
            return Err(ErrorIndicators::SeekError);
        }
        Ok(())
    }
    /// Read sector
    ///
    /// Reads one sector from specified floppy file
    pub fn read_sector(&self, track: u8, sector: u8) -> Result<Sector> {
        Self::check_ranges(track, sector)?;
        let mut buf = [0u8; 194];
        let offset = Self::seek_offset(track, sector);
        let disk = &self.disk;
        let res = disk.seek_read(&mut buf[..], offset as u64);
        if res.is_err() {
            return Err(ErrorIndicators::SeekError);
        };
        let sector = Sector {
            id: buf[0],
            track_address: buf[1],
            sector_address: buf[3],
            crc_id: ((buf[5] as u16) << 8u16) + buf[6] as u16,
            data_deleted_data: buf[35],
            data: buf[36..=163].try_into().unwrap(),
            crc_data: ((buf[164] as u16) << 8) + buf[165] as u16,
        };
        if sector.crc_id != sector.crc_id() {
            return Err(ErrorIndicators::IdCrcError);
        }
        if sector.crc_data != sector.crc_data() {
            return Err(ErrorIndicators::CrcError);
        }
        Ok(sector)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::disk::sssd8fd::{ErrorIndicators, FLOPPY_CAPACITY, Floppy, Sector};

    fn init_disk(file_name: &str) {
        _ = fs::remove_file(file_name);
        // Let's create new floppy image and format
        _ = Floppy::new(file_name, false).unwrap().format();
    }    
    fn remove_disk(file_name: &str) {
        _ = fs::remove_file(file_name);
    }

    #[test]
    fn test_format() {
        let file_name = "test.img";
        init_disk(file_name);
        let mut floppy = Floppy::new(file_name, false).unwrap();
        let result = Floppy::format(&mut floppy);
        assert_eq!(true, result.is_ok());
        // Read first sector and compare CRC.
        let sector = floppy.read_sector(0, 1).unwrap();
        assert_eq!(sector.crc_id, sector.crc_id());
        assert_eq!(sector.crc_data, sector.crc_data());
        remove_disk(file_name);
    }
    #[test]
    fn test_new_file() {
        let file_name = "test_1.img";
        init_disk(file_name);
        let floppy = Floppy::new(file_name, false).unwrap();
        let len = floppy.disk.metadata().unwrap().len();
        assert_eq!(len as usize, FLOPPY_CAPACITY);
        remove_disk(file_name);
    }
    #[test]
    fn test_read() {
        let file_name = "test_2.img";
        init_disk(file_name);
        let floppy = Floppy::new(file_name, true).unwrap();
        let sector = floppy.read_sector(76, 26).unwrap();
        remove_disk(file_name);
        assert_eq!(sector.crc_id, sector.crc_id());
        assert_eq!(sector.crc_data, sector.crc_data());
    }
    #[test]
    fn test_read_invalid_track_0() {
        let file_name = "test_3.img";
        init_disk(file_name);
        let floppy = Floppy::new(file_name, true).unwrap();
        let res = floppy.read_sector(0, 1);
        remove_disk(file_name);
        assert!(res.is_ok());
    }
    #[test]
    fn test_read_invalid_track_77() {
        let file_name = "test_9.img";
        init_disk(file_name);
        let floppy = Floppy::new(file_name, true).unwrap();
        remove_disk(file_name);
        let res = floppy.read_sector(77, 26);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
    #[test]
    fn test_read_invalid_sector_0() {
        let file_name = "test_4.img";
        init_disk(file_name);
        let floppy = Floppy::new(file_name, true).unwrap();
        let res = floppy.read_sector(76, 0);
        remove_disk(file_name);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
    #[test]
    fn test_read_invalid_sector_27() {
        let file_name = "test_5.img";
        init_disk(file_name);
        let floppy = Floppy::new(file_name, true).unwrap();
        let res = floppy.read_sector(76, 27);
        remove_disk(file_name);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
    #[test]
    fn test_write_last_sector() {
        let track_nr = 76;
        let sector_nr = 26;
        let file_name = "test_6.img";
        init_disk(file_name);
        let mut floppy = Floppy::new(file_name, false).unwrap();
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
        let file_name = "test_7.img";
        init_disk(file_name);
        let mut disk = Floppy::new(file_name, false).unwrap();
        let res = disk.format_track(1);
        remove_disk(file_name);
        assert_eq!(Ok(()), res);
    }
    #[test]
    fn test_format_track() {
        let big_track_number = 77;
        let file_name = "test_8.img";
        init_disk(file_name);
        let mut disk = Floppy::new(file_name, false).unwrap();
        let res = disk.format_track(big_track_number);
        remove_disk(file_name);
        assert_eq!(Err(ErrorIndicators::SeekError), res);
    }
}
