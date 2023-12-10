{
  description = "Lean wrapper for polars";

  inputs.lean.url = "github:leanprover/lean4?ref=v4.3.0";
  inputs.lean.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, lean, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.rustPlatform.bindgenHook
        ];
        packages = [
          pkgs.nixpkgs-fmt
          pkgs.nil
          pkgs.rustc
          pkgs.cargo
          pkgs.gdb
          lean.defaultPackage.x86_64-linux
        ];

        RUST_SRC_PATH = ''${pkgs.rustPlatform.rustLibSrc}'';
      };
    };
}
