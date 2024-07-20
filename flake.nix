{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
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
        ];
        hardeningDisable = [ "all" ];
      };
    }
  );
}
