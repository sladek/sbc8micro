use intelhex::IntelHexFile;
pub fn main() {
    match IntelHexFile::load_file("examples/serial_8080.hex") {
        Ok(file) => {
//            display_file_info(&file, 5);

            let n_records = file.records.len();
            for i in 0..n_records {
                let record = &file.records[i];
                let addr = record.addr;
                let data = record.data.to_vec();
                let rtype = &record.rtype; 
                println!("Record type: {:?} - Addr: {:04X}H - Data: {:?}", rtype, addr, data);
            }
        },
        Err(err) => println!("{:?}", err)
    }
}