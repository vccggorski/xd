use heapless::Vec;
use core::cell::RefCell;

pub struct MemoryDispenser<'a> {
    memory: &'a mut [u8],
    used: RefCell<usize>,
}

pub struct OutOfMemory(());

impl<'a> MemoryDispenser<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self {
            memory: slice,
            used: 0_usize.into(),
        }
    }

    pub fn allocate<T: Default>(&self, count: usize) -> Result<&mut [T], OutOfMemory> {
        let t_size = core::mem::size_of::<T>();
        let used = *self.used.borrow();
        if used + t_size * count > self.memory.len() {
            return Err(OutOfMemory(()));
        }
        let output_slice = unsafe {
            core::slice::from_raw_parts_mut(self.memory.as_ptr().add(used) as *mut T, count)
        };
        *self.used.borrow_mut() += t_size * count;
        output_slice
            .iter_mut()
            .for_each(|v| *v = Default::default());
        Ok(output_slice)
    }
}

fn main() {
    let mut memory = [0_u8; 64];
    let mut memory_dispenser = MemoryDispenser::new(&mut memory);
    let a: &mut [Vec<u32, 2>] = memory_dispenser.allocate(2).unwrap_or_else(|_| panic!());
    let b: &mut [Vec<u32, 2>] = memory_dispenser.allocate(2).unwrap_or_else(|_| panic!());
    a[0].push(0xdeadbeef).unwrap();
    a[0].push(0xffffffff).unwrap();
    b[0].push(0x01010101).unwrap();
    b[1].push(0xabcddcba).unwrap();
    a[1].push(0xdeadbeef).unwrap();
    a[1].push(0xffffffff).unwrap();
    b[0].push(0xabcddcba).unwrap();
    b[1].push(0x01010101).unwrap();

    println!("{:#x?}", a);
    println!("{:#x?}", b);
    println!("{:x?}", memory);
}

/*
[
    [
        0xdeadbeef,
        0xffffffff,
    ],
    [
        0xdeadbeef,
        0xffffffff,
    ],
]
[
    [
        0x1010101,
        0xabcddcba,
    ],
    [
        0xabcddcba,
        0x1010101,
    ],
]
[2, 0, 0, 0, 0, 0, 0, 0, ef, be, ad, de, ff, ff, ff, ff, 2, 0, 0, 0, 0, 0, 0, 0, ef, be, ad, de, ff, ff, ff, ff, 2, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, ba, dc, cd, ab, 2, 0, 0, 0, 0, 0, 0, 0, ba, dc, cd, ab, 1, 1, 1, 1]
*/
