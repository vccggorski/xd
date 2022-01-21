use env_logger;
#[cfg(feature = "blocking")]
fn main() {
    env_logger::init();
    use lzma_rs::*;
    lzma_decompress(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut std::io::stdout(),
    )
    .unwrap();
}

#[cfg(feature = "blocking-no-std")]
fn main() {
    env_logger::init();
    use lzma_rs::*;
    let mut memory = vec![0; 9_000_000];
    let mm = allocator::MemoryDispenser::new(&mut memory);
    no_std_lzma_decompress_with_options(
        &mm,
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut std::io::stdout(),
        &Default::default(),
    )
    .unwrap();
    eprintln!("Allocator state: {:#?}", mm);
}

#[cfg(feature = "stream")]
fn main() {
    env_logger::init();
    use lzma_rs::decompress::*;
    use std::io::Read;
    use std::io::Write;
    let mut stream = Stream::new(std::io::stdout());
    let mut bytes = [0_u8; 1 << 22];
    let mut source = std::io::BufReader::new(std::io::stdin());
    loop {
        let bytes_read = source.read(&mut bytes).unwrap();
        if bytes_read == 0 {
            break
        }
        stream.write_all(&bytes[..bytes_read]).unwrap();
        let status = stream.get_stream_status();
        eprintln!("{:?}", &status);
        if let StreamStatus::Finished = status {
            break;
        }
    }
    stream.finish().unwrap();
}

#[cfg(feature = "stream-no-std")]
fn main() {
    env_logger::init();
    use lzma_rs::decompress::*;
    use lzma_rs::*;
    use std::io::Read;
    use std::io::Write;
    let mut memory = vec![0; 9_000_000];
    let mm = allocator::MemoryDispenser::new(&mut memory);
    let mut stream = Stream::no_std_new(&mm, std::io::stdout());
    let mut bytes = [0_u8; 1 << 22];
    let mut source = std::io::BufReader::new(std::io::stdin());
    loop {
        let bytes_read = source.read(&mut bytes).unwrap();
        if bytes_read == 0 {
            break
        }
        stream.write_all(&bytes[..bytes_read]).unwrap();
        let status = stream.get_stream_status();
        eprintln!("{:?}", &status);
        if let StreamStatus::Finished = status {
            break;
        }
    }
    stream.finish().unwrap();
    eprintln!("Allocator state: {:#?}", mm);
}
