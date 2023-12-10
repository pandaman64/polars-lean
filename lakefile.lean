import Lake
open Lake DSL

package «polars-lean» where
  -- add package configuration options here

lean_lib «PolarsLean» where
  -- add library configuration options here

@[default_target]
lean_exe «polars-lean» where
  root := `Main
  -- Enables the use of the Lean interpreter by the executable (e.g.,
  -- `runFrontend`) at the expense of increased binary size on Linux.
  -- Remove this line if you do not need such functionality.
  supportInterpreter := true

-- Add all regular files under `base` to the input files
def inputTree (base : FilePath) : SchedulerM (Array (BuildJob FilePath)) := do
  let go (base : FilePath) : IO (Array FilePath) := do
    let paths ← base.walkDir
    paths.filterM (fun p => do
      pure ((←p.metadata).type == IO.FS.FileType.file)
    )
  let paths ← (go base).catchExceptions (fun e => panic! s!"Failed to list {base}: {e}")
  paths.mapM (fun p => inputFile p)

extern_lib libpolars_lean pkg := do
  let name := nameToStaticLib "polars_lean"
  let rustDir := pkg.dir / "rust"
  let dependencies := #[
    (←inputFile <| rustDir / "Cargo.toml"),
    (←inputFile <| rustDir / "Cargo.lock")
  ] ++ (←inputTree <| rustDir / "src")
  let rustLibFile := rustDir / "target" / "release" / name

  let libDir := pkg.buildDir / "lib"
  let libFile := libDir / name
  buildFileAfterDepArray libFile dependencies (fun _ => do
    proc {
      cmd := "which",
      args := #[ "cargo" ]
    }
    proc {
      cmd := "cargo",
      args := #[ "build", "--release" ],
      cwd := rustDir
    }
    IO.FS.createDirAll libDir
    -- No file copy in Std?
    proc {
      cmd := "cp",
      args := #[ rustLibFile.toString, libFile.toString ]
    }
  )
