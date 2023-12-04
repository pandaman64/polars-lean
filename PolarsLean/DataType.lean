inductive DataType where
  | uint8
  | uint16
  | uint32
  | uint64
  | int8
  | int16
  | int32
  | int64
  -- | Float32
  | float64
  -- TODO: complex types

abbrev DataType.asType : DataType → Type
  | .uint8 => UInt8
  | .uint16 => UInt16
  | .uint32 => UInt32
  | .uint64 => UInt64
  | .int8 => Int
  | .int16 => Int
  | .int32 => Int
  | .int64 => Int
  | .float64 => Float
