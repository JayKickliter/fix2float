#[macro_use]
extern crate structopt;

mod opts;
use opts::*;
use std::io::{self, Read, Write};
use std::ops::Mul;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn as_bytes<T>(src: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            src.as_ptr() as *const u8,
            src.len() * std::mem::size_of::<T>(),
        )
    }
}

fn as_bytes_mut<T>(src: &mut [T]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut(
            src.as_ptr() as *mut u8,
            src.len() * std::mem::size_of::<T>(),
        )
    }
}

fn convert<Tin, Tout>(src: &mut Read, dst: &mut Write, scalar: Option<Tin>)
where
    Tin: Copy + Default + Mul<Output = Tin>,
    Tout: From<Tin>,
{
    let mut ibuf: Vec<Tin> = Vec::with_capacity(DEFAULT_BUF_SIZE / std::mem::size_of::<Tin>());
    let mut obuf: Vec<Tout> = Vec::with_capacity(DEFAULT_BUF_SIZE / std::mem::size_of::<Tout>());

    loop {
        ibuf.resize(
            DEFAULT_BUF_SIZE / std::mem::size_of::<Tin>(),
            Tin::default(),
        );
        obuf.truncate(0);
        let len = match src.read(as_bytes_mut(ibuf.as_mut())) {
            Ok(0) => std::process::exit(0),
            Ok(len) => len / std::mem::size_of::<Tin>(),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => std::process::exit(1),
        };
        for &elem in &ibuf[..len] {
            let scaled: Tin = match scalar {
                None => elem,
                Some(multiplier) => elem * multiplier,
            };
            obuf.push(scaled.into());
        }
        dst.write_all(as_bytes(obuf.as_slice()))
            .expect("Could not write to stdout");
    }
}

fn main() {
    use structopt::StructOpt;
    let opt = Opt::from_args();
    let mut reader: Box<Read> = match opt.input {
        None => Box::new(std::io::stdin()),
        Some(path) => Box::new(std::fs::File::open(path).expect("Invalid source.")),
    };
    let mut writer: Box<Write> = match opt.output {
        None => Box::new(std::io::stdout()),
        Some(path) => Box::new(std::fs::File::open(path).expect("Invalid destination.")),
    };

    // NOTE: commented out match arms are invalid. They are left
    // commented out on purpose so to they can possible be made to
    // work in the near future.
    match (opt.in_type, opt.out_type) {
        (Type::I8, Type::I8) => convert::<i8, i8>(&mut reader, &mut writer, None),
        // (Type::I8, Type::U8) => convert::<i8, u8>(&mut reader, &mut writer, None),
        (Type::I8, Type::I16) => convert::<i8, i16>(&mut reader, &mut writer, None),
        // (Type::I8, Type::U16) => convert::<i8, u16>(&mut reader, &mut writer, None),
        (Type::I8, Type::I32) => convert::<i8, i32>(&mut reader, &mut writer, None),
        // (Type::I8, Type::U32) => convert::<i8, u32>(&mut reader, &mut writer, None),
        (Type::I8, Type::I64) => convert::<i8, i64>(&mut reader, &mut writer, None),
        // (Type::I8, Type::U64) => convert::<i8, u64>(&mut reader, &mut writer, None),
        (Type::I8, Type::F32) => convert::<i8, f32>(&mut reader, &mut writer, None),
        (Type::I8, Type::F64) => convert::<i8, f64>(&mut reader, &mut writer, None),

        // (Type::U8, Type::I8) => convert::<u8, i8>(&mut reader, &mut writer, None),
        (Type::U8, Type::U8) => convert::<u8, u8>(&mut reader, &mut writer, None),
        (Type::U8, Type::I16) => convert::<u8, i16>(&mut reader, &mut writer, None),
        (Type::U8, Type::U16) => convert::<u8, u16>(&mut reader, &mut writer, None),
        (Type::U8, Type::I32) => convert::<u8, i32>(&mut reader, &mut writer, None),
        (Type::U8, Type::U32) => convert::<u8, u32>(&mut reader, &mut writer, None),
        (Type::U8, Type::I64) => convert::<u8, i64>(&mut reader, &mut writer, None),
        (Type::U8, Type::U64) => convert::<u8, u64>(&mut reader, &mut writer, None),
        (Type::U8, Type::F32) => convert::<u8, f32>(&mut reader, &mut writer, None),
        (Type::U8, Type::F64) => convert::<u8, f64>(&mut reader, &mut writer, None),

        // (Type::I16, Type::I8) => convert::<i16, i8>(&mut reader, &mut writer, None),
        // (Type::I16, Type::U8) => convert::<i16, u8>(&mut reader, &mut writer, None),
        (Type::I16, Type::I16) => convert::<i16, i16>(&mut reader, &mut writer, None),
        // (Type::I16, Type::U16) => convert::<i16, u16>(&mut reader, &mut writer, None),
        (Type::I16, Type::I32) => convert::<i16, i32>(&mut reader, &mut writer, None),
        // (Type::I16, Type::U32) => convert::<i16, u32>(&mut reader, &mut writer, None),
        (Type::I16, Type::I64) => convert::<i16, i64>(&mut reader, &mut writer, None),
        // (Type::I16, Type::U64) => convert::<i16, u64>(&mut reader, &mut writer, None),
        (Type::I16, Type::F32) => convert::<i16, f32>(&mut reader, &mut writer, None),
        (Type::I16, Type::F64) => convert::<i16, f64>(&mut reader, &mut writer, None),

        // (Type::U16, Type::I8) => convert::<u16, i8>(&mut reader, &mut writer, None),
        // (Type::U16, Type::U8) => convert::<u16, u8>(&mut reader, &mut writer, None),
        // (Type::U16, Type::I16) => convert::<u16, i16>(&mut reader, &mut writer, None),
        (Type::U16, Type::U16) => convert::<u16, u16>(&mut reader, &mut writer, None),
        (Type::U16, Type::I32) => convert::<u16, i32>(&mut reader, &mut writer, None),
        (Type::U16, Type::U32) => convert::<u16, u32>(&mut reader, &mut writer, None),
        (Type::U16, Type::I64) => convert::<u16, i64>(&mut reader, &mut writer, None),
        (Type::U16, Type::U64) => convert::<u16, u64>(&mut reader, &mut writer, None),
        (Type::U16, Type::F32) => convert::<u16, f32>(&mut reader, &mut writer, None),
        (Type::U16, Type::F64) => convert::<u16, f64>(&mut reader, &mut writer, None),

        // (Type::I32, Type::I8) => convert::<i32, i8>(&mut reader, &mut writer, None),
        // (Type::I32, Type::U8) => convert::<i32, u8>(&mut reader, &mut writer, None),
        // (Type::I32, Type::I16) => convert::<i32, i16>(&mut reader, &mut writer, None),
        // (Type::I32, Type::U16) => convert::<i32, u16>(&mut reader, &mut writer, None),
        (Type::I32, Type::I32) => convert::<i32, i32>(&mut reader, &mut writer, None),
        // (Type::I32, Type::U32) => convert::<i32, u32>(&mut reader, &mut writer, None),
        (Type::I32, Type::I64) => convert::<i32, i64>(&mut reader, &mut writer, None),
        // (Type::I32, Type::U64) => convert::<i32, u64>(&mut reader, &mut writer, None),
        // (Type::I32, Type::F32) => convert::<i32, f32>(&mut reader, &mut writer, None),
        (Type::I32, Type::F64) => convert::<i32, f64>(&mut reader, &mut writer, None),

        // (Type::U32, Type::I8) => convert::<u32, i8>(&mut reader, &mut writer, None),
        // (Type::U32, Type::U8) => convert::<u32, u8>(&mut reader, &mut writer, None),
        // (Type::U32, Type::I16) => convert::<u32, i16>(&mut reader, &mut writer, None),
        // (Type::U32, Type::U16) => convert::<u32, u16>(&mut reader, &mut writer, None),
        // (Type::U32, Type::I32) => convert::<u32, i32>(&mut reader, &mut writer, None),
        (Type::U32, Type::U32) => convert::<u32, u32>(&mut reader, &mut writer, None),
        (Type::U32, Type::I64) => convert::<u32, i64>(&mut reader, &mut writer, None),
        (Type::U32, Type::U64) => convert::<u32, u64>(&mut reader, &mut writer, None),
        // (Type::U32, Type::F32) => convert::<u32, f32>(&mut reader, &mut writer, None),
        (Type::U32, Type::F64) => convert::<u32, f64>(&mut reader, &mut writer, None),

        // (Type::I64, Type::I8) => convert::<i64, i8>(&mut reader, &mut writer, None),
        // (Type::I64, Type::U8) => convert::<i64, u8>(&mut reader, &mut writer, None),
        // (Type::I64, Type::I16) => convert::<i64, i16>(&mut reader, &mut writer, None),
        // (Type::I64, Type::U16) => convert::<i64, u16>(&mut reader, &mut writer, None),
        // (Type::I64, Type::I32) => convert::<i64, i32>(&mut reader, &mut writer, None),
        // (Type::I64, Type::U32) => convert::<i64, u32>(&mut reader, &mut writer, None),
        (Type::I64, Type::I64) => convert::<i64, i64>(&mut reader, &mut writer, None),
        // (Type::I64, Type::U64) => convert::<i64, u64>(&mut reader, &mut writer, None),
        // (Type::I64, Type::F32) => convert::<i64, f32>(&mut reader, &mut writer, None),
        // (Type::I64, Type::F64) => convert::<i64, f64>(&mut reader, &mut writer, None),

        // (Type::U64, Type::I8) => convert::<u64, i8>(&mut reader, &mut writer, None),
        // (Type::U64, Type::U8) => convert::<u64, u8>(&mut reader, &mut writer, None),
        // (Type::U64, Type::I16) => convert::<u64, i16>(&mut reader, &mut writer, None),
        // (Type::U64, Type::U16) => convert::<u64, u16>(&mut reader, &mut writer, None),
        // (Type::U64, Type::I32) => convert::<u64, i32>(&mut reader, &mut writer, None),
        // (Type::U64, Type::U32) => convert::<u64, u32>(&mut reader, &mut writer, None),
        // (Type::U64, Type::I64) => convert::<u64, i64>(&mut reader, &mut writer, None),
        (Type::U64, Type::U64) => convert::<u64, u64>(&mut reader, &mut writer, None),
        // (Type::U64, Type::F32) => convert::<u64, f32>(&mut reader, &mut writer, None),
        // (Type::U64, Type::F64) => convert::<u64, f64>(&mut reader, &mut writer, None),

        // (Type::F32, Type::I8) => convert::<f32, i8>(&mut reader, &mut writer, None),
        // (Type::F32, Type::U8) => convert::<f32, u8>(&mut reader, &mut writer, None),
        // (Type::F32, Type::I16) => convert::<f32, i16>(&mut reader, &mut writer, None),
        // (Type::F32, Type::U16) => convert::<f32, u16>(&mut reader, &mut writer, None),
        // (Type::F32, Type::I32) => convert::<f32, i32>(&mut reader, &mut writer, None),
        // (Type::F32, Type::U32) => convert::<f32, u32>(&mut reader, &mut writer, None),
        // (Type::F32, Type::I64) => convert::<f32, i64>(&mut reader, &mut writer, None),
        // (Type::F32, Type::U64) => convert::<f32, u64>(&mut reader, &mut writer, None),
        (Type::F32, Type::F32) => convert::<f32, f32>(&mut reader, &mut writer, None),
        (Type::F32, Type::F64) => convert::<f32, f64>(&mut reader, &mut writer, None),

        // (Type::F64, Type::I8) => convert::<f64, i8>(&mut reader, &mut writer, None),
        // (Type::F64, Type::U8) => convert::<f64, u8>(&mut reader, &mut writer, None),
        // (Type::F64, Type::I16) => convert::<f64, i16>(&mut reader, &mut writer, None),
        // (Type::F64, Type::U16) => convert::<f64, u16>(&mut reader, &mut writer, None),
        // (Type::F64, Type::I32) => convert::<f64, i32>(&mut reader, &mut writer, None),
        // (Type::F64, Type::U32) => convert::<f64, u32>(&mut reader, &mut writer, None),
        // (Type::F64, Type::I64) => convert::<f64, i64>(&mut reader, &mut writer, None),
        // (Type::F64, Type::U64) => convert::<f64, u64>(&mut reader, &mut writer, None),
        // (Type::F64, Type::F32) => convert::<f64, f32>(&mut reader, &mut writer, None),
        (Type::F64, Type::F64) => convert::<f64, f64>(&mut reader, &mut writer, None),
        (in_type, out_type) => eprintln!("Invalid conversion: {} -> {}", in_type, out_type),
    };
}
