#[cfg(feature = "blocking")]
fn main() {
    use lzma_rs::*;
    lzma_decompress(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut std::io::stdout(),
    )
    .unwrap();
}

#[cfg(feature = "blocking-no-std")]
fn main() {
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
    use lzma_rs::decompress::*;
    std::io::copy(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut Stream::new(std::io::stdout()),
    )
    .unwrap();
}

#[cfg(feature = "stream-no-std")]
fn main() {
    use lzma_rs::*;
    use lzma_rs::decompress::*;
    let mut memory = vec![0; 9_000_000];
    let mm = allocator::MemoryDispenser::new(&mut memory);
    std::io::copy(
        &mut std::io::BufReader::new(std::io::stdin()),
        &mut Stream::no_std_new(&mm, std::io::stdout()),
    )
    .unwrap();
    eprintln!("Allocator state: {:#?}", mm);
}
