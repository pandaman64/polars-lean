import «PolarsLean»

@[extern "polars_lean_add"]
private opaque add : UInt32 → UInt32 → UInt32

def main : IO Unit :=
  IO.println s!"Hello, {add 100 200}!"
