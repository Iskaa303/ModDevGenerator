{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    targets = [ "wasm32-unknown-unknown" ];
  };

  packages = [
    pkgs.binaryen
    pkgs.dioxus-cli
    pkgs.wasm-bindgen-cli
  ];

  scripts.dev.exec = "dx serve --platform web";
  scripts.build.exec = "dx build --release --platform web --debug-symbols false";
}
