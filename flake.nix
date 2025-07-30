{
  description = "iOS Fonts Configurator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
            libiconv
          ];

          shellHook = ''
            echo "Happy coding! 🦀"
          '';
        };

        # For compatibility with older nix versions
        devShell = self.devShells.${system}.default;
      }
    );
}
