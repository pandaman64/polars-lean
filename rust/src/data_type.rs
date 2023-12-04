#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float64,
}
