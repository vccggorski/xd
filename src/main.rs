use lzma_rs::*;
use lzma_rs::decompress::*;
use allocator::MemoryDispenser;
use io::Cursor;
use env_logger;

fn main() {
    env_logger::init();
    let mut memory = [0_u8; 18694];
    let mm = MemoryDispenser::new(&mut memory);
    let options = Options {
        memlimit: Some(125),
        ..Default::default()
    };
    let compressed_data = [
        0x5d, 0x00, 0x00, 0x80, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x30,
        0xe9, 0x14, 0xb4, 0x91, 0x15, 0x7b, 0xd4, 0x77, 0xff, 0xff, 0xf4, 0xcc, 0x80, 0x00,
    ];
    let mut decompressed_data = [0_u8; 32];
    no_std_lzma_decompress_with_options(
        &mm,
        &mut Cursor::new(&compressed_data),
        &mut Cursor::new(&mut decompressed_data[..]),
        &options,
    ).unwrap();
    println!("{:#x?}", decompressed_data);

    let options = Options {
        memlimit: Some(1024),
        ..Default::default()
    };
    let compressed_data = [
        0x5d, 0x00, 0x00, 0x80, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x30,
        0xe9, 0x14, 0xb4, 0x91, 0x15, 0x7b, 0xd4, 0x77, 0xff, 0xff, 0xf4, 0xcc, 0x80, 0x00,
    ];
    let mut decompressed_data = [0_u8; 32];
    lzma_decompress_with_options(
        &mut Cursor::new(&compressed_data),
        &mut Cursor::new(&mut decompressed_data[..]),
        &options,
    ).unwrap();
    println!("{:#x?}", decompressed_data);
}

/*
Result:
[
    0x61,
    0x61,
    0x61,
    0x61,
    0x61,
    0x62,
    0x62,
    0x62,
    0x62,
    0x62,
    0x63,
    0x63,
    0x63,
    0x63,
    0x63,
]
*/
