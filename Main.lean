import «PolarsLean»

opaque SeriesPointed : NonemptyType
def Series : Type := SeriesPointed.type
instance : Nonempty Series := SeriesPointed.property

@[extern "give_me_a_series"]
private opaque giveMeASeries: Unit → Series

@[extern "polars_lean_print_series"]
private opaque Series.print : @& Series → String

def main : IO Unit := do
  IO.println s!"Hello Rust!"
  let s := giveMeASeries ()
  IO.println (Series.print s)
  IO.println (Series.print s)
  IO.println (Series.print s)
  IO.println (Series.print s)
