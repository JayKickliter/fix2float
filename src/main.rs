use std::io::{self, Read, Write};
use std::mem;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn as_bytes<T>(src: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(src.as_ptr() as *const u8, src.len() * mem::size_of::<T>())
    }
}

fn as_bytes_mut<T>(src: &mut [T]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut(src.as_ptr() as *mut u8, src.len() * mem::size_of::<T>())
    }
}

type ReadBuf = [i16; 4 * 1024];

fn main() {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let mut readbuf: ReadBuf = [0; 4 * 1024];
    let mut writebuf: Vec<f32> = Vec::with_capacity(2 * 1024);

    loop {
        writebuf.truncate(0);
        let len = match stdin_handle.read(as_bytes_mut(&mut readbuf)) {
            Ok(0) => std::process::exit(0),
            Ok(len) => len / mem::size_of::<u16>(),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => std::process::exit(1),
        };
        for elem in &readbuf[..len] {
            writebuf.push(*elem as f32 / std::i16::MAX as f32);
        }
        stdout_handle.write_all(as_bytes(writebuf.as_slice())).expect("Could not write to stdout");
    }
}
