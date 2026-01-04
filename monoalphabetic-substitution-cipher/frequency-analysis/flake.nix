{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # Disable all Nix hardening flags to prevent interference with Cargo builds.
          # These flags are designed for C/C++ and can cause issues with:
          # - MUSL builds (fortify adds glibc-specific functions)
          # - Crates that vendor C libraries (e.g., git2 vendoring libgit2)
          # Rust already provides memory safety, so these hardening flags provide
          # minimal benefit while causing build problems.
          hardeningDisable = [ "all" ];

          buildInputs = [
            # Rust with standard toolchain.
            pkgs.rust-bin.stable.latest.default
          ];
        };
      }
    );
}
