//! Simple example that echoes received serial traffic to stdout
use serialport;
use serialport::DataBits;
use serialport::Parity;
use serialport::SerialPort;
use serialport::StopBits;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/// This defines a short interval for receiver's thread, that improves the performance rapidly
///
///  Check description in start function
const THREAD_SLEEP_IN_MILIS: u64 = 50;

/// An error type for serial port operations
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    /// The kind of error this is
    pub kind: ErrorKind,
    /// A description of the error suitable for end-users
    pub description: String,
}
impl Error {
    /// Instantiates a new error
    pub fn new<T: Into<String>>(kind: ErrorKind, description: T) -> Self {
        Error {
            kind,
            description: description.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BufferOverflow,
    Unknown,
}

pub struct Serial {
    tx_rdy: bool,
    rx_port: Option<Box<dyn SerialPort>>,
    tx_port: Option<Box<dyn SerialPort>>,
    channel: (Sender<u8>, Receiver<u8>),
}
impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}
impl Serial {
    /// Creates serial port object.
    ///
    /// At this stage no real port is open
    pub fn new() -> Self {
        Self {
            tx_rdy: true,
            rx_port: None,
            tx_port: None,
            channel: mpsc::channel::<u8>(),
        }
    }
    /// Opens serial port with specific name and baud rate
    pub fn open(
        &mut self,
        port_name: &str,
        baud_rate: u32,
        data_bits: DataBits,
        stop_bits: StopBits,
        parity: Parity,
    ) -> Result<&mut Self, serialport::Error> {
        match serialport::new(port_name, baud_rate)
            .data_bits(data_bits)
            .stop_bits(stop_bits)
            .parity(parity)
            .timeout(Duration::from_millis(1))
            .open()
        {
            Ok(port) => {
                // Let's clone the port as the original one will be moved to to separated thread for reading
                // in background so it doesn't block the original thread
                let tx_port = port.try_clone();
                self.rx_port = Some(port);
                match tx_port {
                    Ok(tx_port) => {
                        self.tx_port = Some(tx_port);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                return Err(err);
            }
        };
        Ok(self)
    }
    /// Starts the thread with receiving side of the serial port
    ///
    /// As the read() function is blocking, the receiving function is moved to another thread and communicating with main thread via channel
    /// This way the channel keeps received data even though the main thread is busy with something else. There are different possibilities
    /// like serial_mio or serial_tokio, but those are quite an overkill for this application
    pub fn start(&mut self) -> Result<JoinHandle<u8>, serialport::Error> {
        // Create the channel for the request
        let reply_tx = self.channel.0.clone();
        // Start a new thread to handle the request
        match &self.rx_port {
            Some(port) => {
                let mut port = port.try_clone()?;
                Ok(thread::spawn(move || {
                    let mut buf = [0u8; 1];
                    // Main loop of the thread. It reads data from serial port and send it to the main thread via channel.
                    loop {
                        // short sleep before the next iteration. It improves the performance of receiving and sending the data at the same time.
                        // Without this, the sending was a lot slower and visible on the serial terminal when sending out some longer text after
                        // receiving command from terminal. This "trick" probably releases the thread allowing for faster sending.
                        thread::sleep(Duration::from_millis(THREAD_SLEEP_IN_MILIS));
                        if let Ok(mut u) = port.read(&mut buf) {
                            while u != 0 {
                                let t = buf[0];
                                _ = reply_tx.send(t);
                                u -= 1;
                            }
                        };
                    }
                }))
            }
            None => Err(serialport::Error {
                description: "Serial port not found".to_string(),
                kind: serialport::ErrorKind::NoDevice,
            }),
        }
    }
    /// Returns rx_rdy flag
    pub fn is_tx_rdy(&self) -> bool {
        self.tx_rdy
    }
    /// Sets tx_rdy flag
    pub fn set_tx_rdy(&mut self, value: bool) {
        self.tx_rdy = value;
    }
    /// Gets tx_port
    pub fn get_tx_port(&mut self) -> &Option<Box<dyn SerialPort>> {
        &self.tx_port
    }
    /// Gets rx_port
    pub fn get_rx_port(&mut self) -> &Option<Box<dyn SerialPort>> {
        &self.rx_port
    }
    /// Get channel
    pub fn get_channel(&mut self) -> &(Sender<u8>, Receiver<u8>) {
        &self.channel
    }
    /// Reads a data from channel
    ///
    /// Reads a data from receiver channel and returns Some(u8) if there is some data
    /// or None if channel doesn't contain any data
    pub fn read_data(&mut self) -> Option<u8> {
        if let Ok(data) = self.channel.1.try_recv() {
            return Some(data);
        }
        None
    }
    /// Write a data to serial port
    ///
    /// Writes a data to serial port 
    pub fn write_data(&mut self, data: u8) -> Result<(), Error> {
        //        self.out_buffer[0] = data;
        let buffer = [data];
        match self.tx_port.as_mut() {
            Some(port) => match port.write(&buffer) {
                Ok(_num) => Ok(()),
                Err(err) => Err(Error {
                    description: err.to_string(),
                    kind: ErrorKind::Unknown,
                }),
            },
            None => Err(Error {
                description: "tx_port is not set.".to_string(),
                kind: ErrorKind::Unknown,
            }),
        }
    }
}
