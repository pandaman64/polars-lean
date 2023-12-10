import PolarsLean.Error
import PolarsLean.Series

private opaque DataFramePointed : NonemptyType
def DataFrame : Type := DataFramePointed.type
instance : Nonempty DataFrame := DataFramePointed.property

@[extern "polars_lean_data_frame_from_series_array"]
opaque DataFrame.fromSeriesArray (array : @& Array Series) : Except PolarsError DataFrame

@[extern "polars_lean_print_data_frame"]
opaque DataFrame.print (self : @& DataFrame) : String

/-
IO Functions
-/
@[extern "polars_lean_data_frame_read_csv"]
private opaque DataFrame.extReadCsv (path : @& String) : EIO PolarsError DataFrame

def DataFrame.readCsv (path : String) : IO DataFrame := do
  DataFrame.extReadCsv path
  |> EIO.toIO (fun e => IO.userError e.toString)
