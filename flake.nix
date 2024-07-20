{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, utils, rust-overlay }: utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Dependencies for gnu toolchain
          binutils
          autoconf
          automake
          curl
          python3
          libmpc
          mpfr
          gmp
          gawk
          bison
          flex
          texinfo
          gperf
          libtool
          patchutils
          bc
          zlib
          expat
          ninja
          git
          cmake
          libslirp

          # Rust
          (rust-bin.stable.latest.default.override {
            targets = [ "riscv32i-unknown-none-elf" ];
          })
        ];
        hardeningDisable = [ "all" ];
      };
    }
  );
}
