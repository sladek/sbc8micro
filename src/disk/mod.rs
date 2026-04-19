pub mod sssd8fd;
pub mod hdd8m;

use crc::CRC_16_IBM_3740;
use std::fs::File;
use crate::io::ErrorIndicators;
use std::io::Seek;
use std::io::Write;
use std::os::windows::fs::FileExt;

pub const DATA_SIZE: usize = 128;
const ID_ADDRESS_MARK: u8 = 0xFE; // ID Address Mark identifier byte
const SECTOR_SIZE: u16 = 195; // Number of bytes in one complete sector
const FORMAT_PATTERN: u8 = 0xE5;

//Define our own result
pub type Result<T> = core::result::Result<T, ErrorIndicators>;

#[derive(Clone)]
pub enum DataDeletedData {
    Data = 0x0B,
    DeletedData = 0x08,
}

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

pub trait Disk {
    const NUM_OF_TRACKS: u8;
    const NUM_OF_SECTORS_PER_TRACK: u8;
    const DISK_CAPACITY: usize;

    fn get_disk(&self) -> &File;
    fn get_name(&self) -> String;
    fn open_file_image(name: &str, read_only: bool) -> std::io::Result<File> {
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
    fn format(&mut self) -> Result<()> {
        if self.get_disk().seek(std::io::SeekFrom::Start(0)).is_err() {
            return Err(ErrorIndicators::SeekError);
        };
        let data = [FORMAT_PATTERN; 128];
        for track in 0..Self::NUM_OF_TRACKS {
            for sector in 1..=Self::NUM_OF_SECTORS_PER_TRACK {
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
    fn format_track(&mut self, track: u8) -> Result<()> {
        Self::check_ranges(track, 1)?;
        let data = [FORMAT_PATTERN; 128];
        let offset = Self::seek_offset(track, 1);
        if self.get_disk()
            .seek(std::io::SeekFrom::Start(offset as u64))
            .is_err()
        {
            return Err(ErrorIndicators::SeekError);
        }
        for sector in 1..=Self::NUM_OF_SECTORS_PER_TRACK {
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
    fn seek_write_sector(&mut self, sector: Sector) -> Result<()> {
        Self::check_ranges(sector.track_address, sector.sector_address)?;
        let offset = Self::seek_offset(sector.track_address, sector.sector_address);
        if self.get_disk()
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
    fn seek(&mut self, track_number: u8) -> Result<()> {
        Self::check_ranges(track_number, 1)?;
        let offset = Self::seek_offset(track_number, 1);
        if self.get_disk()
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
        self.get_disk()
            .write_all(&[sector.id, sector.track_address, 0, sector.sector_address, 0])?;
        self.get_disk().write_all(&sector.crc_id().to_be_bytes())?;
        self.get_disk().write_all(&gap)?;
        self.get_disk().write_all(&[sector.data_deleted_data])?;
        self.get_disk().write_all(&sector.data)?;
        self.get_disk().write_all(&sector.crc_data().to_be_bytes())?;
        self.get_disk().write_all(&gap)
    }
    /// Seak offset on the disk
    ///
    /// Seeks an offset on the disk based on track and sector number
    fn seek_offset(track_nr: u8, sector_nr: u8) -> u32 {
        (track_nr as u32 * (Self::NUM_OF_SECTORS_PER_TRACK as u32) * (SECTOR_SIZE as u32 - 1))
            + ((sector_nr - 1) as u32 * (SECTOR_SIZE as u32 - 1))
    }
    /// Check track and sector ranges
    ///
    /// Check if track numbers or sector numbers are in the allowed range
    fn check_ranges(track: u8, sector: u8) -> Result<()> {
        if track > Self::NUM_OF_TRACKS - 1 {
            return Err(ErrorIndicators::SeekError);
        }
        if sector == 0 || sector > Self::NUM_OF_SECTORS_PER_TRACK {
            return Err(ErrorIndicators::SeekError);
        }
        Ok(())
    }
    /// Read sector
    ///
    /// Reads one sector from specified floppy file
    fn read_sector(&self, track: u8, sector: u8) -> Result<Sector> {
        Self::check_ranges(track, sector)?;
        let mut buf = [0u8; 194];
        let offset = Self::seek_offset(track, sector);
        let res = self.get_disk().seek_read(&mut buf[..], offset as u64);
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
