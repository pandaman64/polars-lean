import PolarsLean

def main : IO Unit := do
  IO.println s!"Hello Rust!"
  let iris ← DataFrame.readCsv "data/iris.csv"
  IO.println iris.print
