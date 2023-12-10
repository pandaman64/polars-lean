-- TODO: represent as an inductive type reflecting the cases
private opaque PolarsErrorPointed : NonemptyType
def PolarsError : Type := PolarsErrorPointed.type
instance : Nonempty PolarsError := PolarsErrorPointed.property

@[extern "polars_lean_polars_error_to_string"]
opaque PolarsError.toString (self : @& PolarsError) : String
