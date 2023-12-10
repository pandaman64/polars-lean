import PolarsLean

def main : IO Unit := do
  IO.println s!"Hello Rust!"
  let iris ← DataFrame.readCsv "data/iris.csv"
  IO.println iris.print

  let iris' ← LazyFrame.scanCsv "data/iris.csv"

  let plan := iris'.sort "sepal_length"
  IO.println plan.describePlan
  IO.println (match plan.describeOptimizedPlan with
    | .ok s => s
    | .error e => e.toString
  )
  -- TODO: make this IO?
  let sorted := plan.collect
  match sorted with
  | .ok df => IO.println df.print
  | .error e => IO.println e.toString
