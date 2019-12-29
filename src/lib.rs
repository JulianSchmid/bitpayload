use std::fmt::{Formatter, Debug};

pub enum Type {
    BaseType(BaseTypeDef),
    Struct(StructDef),
    EnumDef(EnumDef),
    StaticArray(Box<StaticArrayDef>),
    DynamicArray(Box<DynamicArrayDef>),
    StringType(StringTypeDef),
}

pub struct BaseTypeDef {
    /// The type this value is decoded to
    pub decoded_type: BaseType,

    // TODO: bitsize, minvalue, maxvalue, encoding decoding formulas?

    /// Endianess of the encoded value
    pub endianess: Endianness
}

/// Signed & unsigned integer types
pub enum IntType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    SInt8,
    SInt16,
    SInt32,
    SInt64,
}

/// Integer values
pub enum IntValue {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    SInt8(i8),
    SInt16(i16),
    SInt32(i32),
    SInt64(i64),
}

pub enum BaseType {
    Int(IntType),
    Bool,
    Float32,
    Float64
}

pub struct StructDef {
    pub fields: Vec<FieldDef>
}

pub struct FieldDef {
    pub name: String,
    pub value_type: Type
}

pub struct StaticArrayDef {
    pub size: usize,
    /// Type of the entries in the array
    pub entry_type: Type
}

pub struct DynamicArrayDef {
    /// The minimum number of elements in the array
    pub min_size: usize,

    /// The maximum number of elements in the array
    pub max_size: usize,
    
    /// Type of the field describing the size of the array.
    pub size_type: IntType,

    /// Type of the entries in the array
    pub entry_type: Type
}

pub struct EnumDef {
    pub underlying_type: IntType,
    pub values: Vec<EnumValueDef>
}

pub struct EnumValueDef {
    pub name: String,
    pub encoded_value: IntValue
}

pub struct StringTypeDef {
    //TODO
}

/// Enum describing the endianess of a value on a network.
///
/// Note: I could not find an enum in the rust core library,
/// so I defined my own
pub enum Endianness {
    BigEndian,
    LittleEndian
}

impl std::fmt::Debug for IntType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter::<'_>) -> Result<(), std::fmt::Error> {
        use IntType::*;
        match self {
            UInt8  => fmt.write_str("u8"),
            UInt16 => fmt.write_str("u16"),
            UInt32 => fmt.write_str("u32"),
            UInt64 => fmt.write_str("u64"),
            SInt8  => fmt.write_str("i8"),
            SInt16 => fmt.write_str("i16"),
            SInt32 => fmt.write_str("i32"),
            SInt64 => fmt.write_str("i64"),
        }
    }
}

impl std::fmt::Debug for IntValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter::<'_>) -> Result<(), std::fmt::Error> {
        use IntValue::*;
        match self {
            UInt8(value)  => value.fmt(fmt),
            UInt16(value) => value.fmt(fmt),
            UInt32(value) => value.fmt(fmt),
            UInt64(value) => value.fmt(fmt),
            SInt8(value)  => value.fmt(fmt),
            SInt16(value) => value.fmt(fmt),
            SInt32(value) => value.fmt(fmt),
            SInt64(value) => value.fmt(fmt),
        }
    }
}

impl EnumDef {

    /// Write the enum definition with the given indentation
    fn fmt_ident(&self, indent: usize, fmt: &mut Formatter::<'_>) -> Result<(), std::fmt::Error> {

        write_indent(indent, fmt)?;

        fmt.write_str("enum : ")?;
        self.underlying_type.fmt(fmt)?;
        fmt.write_str(" {")?;

        let mut first_value = true;
        for value in &self.values {
            if first_value {
                first_value = false;
            } else {
                fmt.write_str(", ")?;
            }
            fmt.write_str("\n")?;
            write_indent(indent + 1, fmt)?;

            fmt.write_str(&value.name)?;
            fmt.write_str(" = ")?;
            value.encoded_value.fmt(fmt)?;
        }

        fmt.write_str("\n")?;
        write_indent(indent, fmt)?;
        fmt.write_str(" }")
    }
}

impl std::fmt::Debug for EnumDef {
    fn fmt(&self, fmt: &mut std::fmt::Formatter::<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str("enum : ")?;
        self.underlying_type.fmt(fmt)?;
        fmt.write_str(" {")?;

        if fmt.alternate() {
            self.fmt_ident(0, fmt)
        } else {
            let mut first_value = true;
            for value in &self.values {
                if first_value {
                    first_value = false;
                    fmt.write_str(" ")?;
                } else {
                    fmt.write_str(", ")?;
                }
                fmt.write_str(&value.name)?;
                fmt.write_str(" = ")?;
                value.encoded_value.fmt(fmt)?;
            }
            fmt.write_str(" }")
        }
    }
}

fn write_indent(indent: usize, fmt: &mut Formatter::<'_>) -> Result<(), std::fmt::Error> {
    for _ in 0..indent {
        fmt.write_str("    ")?
    }
    Ok(())
}
