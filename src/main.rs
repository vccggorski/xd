use allocator::MemoryDispenser;
use env_logger;
use io::Cursor;
use lzma_rs::decompress::*;
use lzma_rs::*;

fn main() {
    env_logger::init();
//    {
//        let mut memory = vec![0; 400_000];
//        let mm = MemoryDispenser::new(&mut memory);
//        let options = Options {
//            memlimit: Some(300_000),
//            ..Default::default()
//        };
//        let mut compressed_data = std::io::BufReader::new(
//            std::fs::File::open("/home/glaeqen/repos/lzma/xd/simple.lzma").unwrap(),
//        );
//        let mut decompressed_data = Vec::new();
//        no_std_lzma_decompress_with_options(
//            &mm,
//            &mut compressed_data,
//            &mut decompressed_data,
//            &options,
//        )
//        .unwrap();
//        println!("{:#x?}", &decompressed_data[..32]);
//    }
//    {
//        let options = Options {
//            ..Default::default()
//        };
//        let mut compressed_data =
//            std::io::BufReader::new(
//            std::fs::File::open("/home/glaeqen/repos/lzma/xd/simple.lzma").unwrap()
//            );
//        let mut decompressed_data = Vec::new();
//        lzma_decompress_with_options(
//            &mut compressed_data,
//            &mut decompressed_data,
//            &options,
//        )
//        .unwrap();
//        println!("{:#x?}", &decompressed_data[..32]);
//    }
    {
        let mut memory = vec![0; 400_000];
        let mm = MemoryDispenser::new(&mut memory);
        let mut stream = Stream::no_std_new(&mm, std::io::stdout());
        let mut stream = Stream::new(std::io::stdout());
        let mut compressed_data =
            std::io::BufReader::new(
            std::fs::File::open("/home/glaeqen/repos/lzma/xd/simple.lzma").unwrap()
            );
        std::io::copy(&mut compressed_data, &mut stream).unwrap();
    }
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
