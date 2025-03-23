# Open a shell with all dependencies needed to run cargo build --release ...
#
#   echo extra-experimental-features = nix-command flakes >> ~/.config/nix/nix.conf
#   nix develop
#
# or
#
#   nix --extra-experimental-features nix-command --extra-experimental-features flakes develop
#
# then run
#
#   cargo build --release --target="armv7-unknown-linux-gnueabihf"
{
  description = "Build dependencies for Rust target armv7-unknown-linux-gnueabihf";
  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        crossSystem = {
          config = "armv7l-linux-gnueabihf";
        };
        overlays = [ (import rust-overlay) ];
        pkgs     = import nixpkgs {
          inherit system overlays crossSystem;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          # buildInputs = with pkgs; [
          #   rust-bin.stable.latest.default
          # ];
          packages = with pkgs; [
            rust-bin.stable.latest.default
          ];
        };
        devShells.nightly = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.nightly.latest.default
          ];
        };
      }
    );
}
