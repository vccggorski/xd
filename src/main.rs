use core::time::Duration;
use env_logger;
use serialport;
use serialport::SerialPort;
use std::thread;
fn main() {
    env_logger::init();

    use std::io::Read;
    use std::io::Write;
    let mut input = std::io::stdin();
    let mut output = std::io::stdout();
    let mut serial1 = serialport::new("/dev/zvitek/ttyUART1", 30_000)
        .open_native()
        .unwrap();
    let mut serial2 = serialport::new("/dev/zvitek/ttyUART2", 4_000_000)
        .open_native()
        .unwrap();
    let _ = thread::spawn(move || loop {
        let mut bytes = [0_u8; 128];
        match input.read(&mut bytes) {
            Ok(bytes_read) => {
                serial1.write_all(&bytes[..bytes_read]).unwrap();
                serial1.flush().unwrap();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    });
    let mut bytes = [0_u8; 128];
    loop {
        match serial2.read(&mut bytes) {
            Ok(bytes_read) => {
                output.write_all(&bytes[..bytes_read]).unwrap();
                output.flush().unwrap();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
