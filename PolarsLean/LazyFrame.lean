import PolarsLean.DataFrame

private opaque LazyFramePointed : NonemptyType
def LazyFrame : Type := LazyFramePointed.type
instance : Nonempty LazyFrame := LazyFramePointed.property

@[extern "polars_lean_lazy_frame_sort"]
opaque LazyFrame.sort (self : LazyFrame) (by_column : @& String) : LazyFrame

@[extern "polars_lean_lazy_frame_collect"]
opaque LazyFrame.collect (self : LazyFrame) : Except PolarsError DataFrame

@[extern "polars_lean_lazy_frame_describe_plan"]
opaque LazyFrame.describePlan (self : @& LazyFrame) : String

@[extern "polars_lean_lazy_frame_describe_optimized_plan"]
opaque LazyFrame.describeOptimizedPlan (self : @& LazyFrame) : Except PolarsError String

@[extern "polars_lean_lazy_frame_scan_csv"]
opaque LazyFrame.scanCsv (path : @& String) : IO LazyFrame
