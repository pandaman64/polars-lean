import PolarsLean.DataType

private opaque SeriesPointed : NonemptyType
def Series : Type := SeriesPointed.type
instance : Nonempty Series := SeriesPointed.property

@[extern "polars_lean_series_from_array"]
opaque Series.fromArray (dt : @& DataType) (name : @& String) (array : @& Array dt.asType) : Series

@[extern "polars_lean_print_series"]
opaque Series.print : @& Series → String
