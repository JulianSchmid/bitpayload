/// A description of a field in a payload.
#[derive(Clone)]
pub enum Field {
    /// A struct field contains a list of subfields with names.
    Struct(StructField),
    /// An array with a static fixed size.
    StaticArray(Box<StaticArrayField>),
    /// An array that can have a dynamic size.
    DynamicArray(Box<DynamicArrayField>),
    StringType(StringDesc),

    Bool(BoolField),

    UInt8(IntField<u8, BytePosition>), //TODO replace with a non endian version
    UInt16(IntField<u16, MultiBytePosition>),
    UInt32(IntField<u32, MultiBytePosition>),
    UInt64(IntField<u64, MultiBytePosition>),
    SInt8(IntField<i8, BytePosition>), //TODO replace with a non endian version
    SInt16(IntField<i16, MultiBytePosition>),
    SInt32(IntField<i32, MultiBytePosition>),
    SInt64(IntField<i64, MultiBytePosition>),

    Float32(FloatDesc<f32>),
    Float64(FloatDesc<f64>)
}

/// Description of a value that has at most a byte of size
#[derive(Clone)]
pub struct BytePosition {

    /// Offset in bits
    pub offset: usize,

    /// Size in bits
    pub size: u8,

}

/// Description of the position of a multi byte value & it's endianess
#[derive(Clone)]
pub struct MultiBytePosition {

    /// Offset in bits
    pub offset: usize,

    /// Size in bits
    pub size: u8,

    /// The endianess of the value
    pub endianess: Endianness
}

#[derive(Clone)]
pub struct BoolField {
    pub position: BytePosition
}

#[derive(Clone)]
pub struct IntField<T, Pos> {
    pub position: Pos,

    marker: std::marker::PhantomData<T>

    //TODO: Add interpretations (e.g. enums, bitmasks etc.)
}

#[derive(Clone)]
pub struct FloatDesc<T> {
    pub position: MultiBytePosition,

    marker: std::marker::PhantomData<T>
}

#[derive(Clone)]
pub struct StructField {
    pub fields: Vec<NamedField>
}

#[derive(Clone)]
pub struct NamedField {
    pub name: String,
    pub field: Field
}

#[derive(Clone)]
pub struct StaticArrayField {
    pub size: usize,
    /// Type of the entries in the array
    pub entry_type: Field
}

///
#[derive(Clone)]
pub enum SizeField {
    //TODO add all variants
}

#[derive(Clone)]
pub struct DynamicArrayField {
    /// The minimum number of elements in the array
    pub min_size: usize,

    /// The maximum number of elements in the array
    pub max_size: usize,
    
    /// Type of the field describing the size of the array.
    pub size_type: SizeField,
    
    /// Type of the entries in the array
    pub entry_type: Field
}

#[derive(Clone)]
pub struct StringDesc {
    //TODO
}

/// Enum describing the endianess of a value on a network.
#[derive(Clone)]
pub enum Endianness {
    BigEndian,
    LittleEndian
}