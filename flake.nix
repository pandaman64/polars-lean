{
  description = "Lean wrapper for polars";

  outputs = { self, nixpkgs }:
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
        ];
      };
    };
}