import PolarsLean

def main : IO Unit := do
  IO.println s!"Hello Rust!"
  let series := Series.fromArray DataType.uint8 #[1, 2, 4, 5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.uint16 #[1, 2, 4, 5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.uint32 #[1, 2, 4, 5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.uint64 #[1, 2, 4, 5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.int8 #[1, -2, 4, -5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.int16 #[1, -2, 4, -5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.int32 #[1, -2, 4, -5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.int64 #[1, -2, 4, -5, 7]
  IO.println series.print

  let series := Series.fromArray DataType.float64 #[1.25, -2.5, 4.75, -5.0, 7.25]
  IO.println series.print
