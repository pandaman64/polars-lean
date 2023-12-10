import PolarsLean

def main : IO Unit := do
  IO.println s!"Hello Rust!"
  let intSeries := Series.fromArray DataType.uint8 "😎" #[1, 2, 4, 5, 7]
  -- IO.println intSeries.print

  let floatSeries := Series.fromArray DataType.float64 "😝" #[1.25, -2.5, 4.75, -5.0, 7.25]
  -- IO.println floatSeries.print

  match DataFrame.fromSeriesArray #[intSeries, floatSeries] with
  | .error e => IO.println e.toString
  | .ok dataFrame => IO.println dataFrame.print
