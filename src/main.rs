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
    let mut stream = Stream::new(std::io::stdout());
    std::io::copy(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut stream,
    )
    .unwrap();
    stream.finish().unwrap();
}

#[cfg(feature = "stream-no-std")]
fn main() {
    env_logger::init();
    use lzma_rs::decompress::*;
    use lzma_rs::*;
    let mut memory = vec![0; 9_000_000];
    let mm = allocator::MemoryDispenser::new(&mut memory);
    let mut stream = Stream::no_std_new(&mm, std::io::stdout());
    std::io::copy(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut stream,
    )
    .unwrap();
    stream.finish().unwrap();
    eprintln!("Allocator state: {:#?}", mm);
}
