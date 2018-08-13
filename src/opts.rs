use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

impl FromStr for Type {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i8" => Ok(Type::I8),
            "u8" => Ok(Type::U8),
            "i16" => Ok(Type::I16),
            "u16" => Ok(Type::U16),
            "i32" => Ok(Type::I32),
            "u32" => Ok(Type::U32),
            "i64" => Ok(Type::I64),
            "u64" => Ok(Type::U64),
            "f32" => Ok(Type::F32),
            "f64" => Ok(Type::F64),
            _ => Err("invalid type"),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I8 => write!(f, "i8"),
            Type::U8 => write!(f, "u8"),
            Type::I16 => write!(f, "i16"),
            Type::U16 => write!(f, "u16"),
            Type::I32 => write!(f, "i32"),
            Type::U32 => write!(f, "u32"),
            Type::I64 => write!(f, "i64"),
            Type::U64 => write!(f, "u64"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "streamconv")]
pub struct Opt {
    /// Input file, `stdin` if not specified.
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Output file, `stdin` if not specified.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    pub in_type: Type,
    pub out_type: Type,
}
