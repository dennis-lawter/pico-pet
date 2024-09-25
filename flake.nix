{
  description = "Flake for building the pico-pet";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";  # Use rust overlay for more control over Rust versions
  };

  outputs = { self, nixpkgs, rust-overlay }: {
    devShells.default = nixpkgs.mkShell {
      buildInputs = [
        nixpkgs.alsaLib
        nixpkgs.pkg-config
        nixpkgs.udev
        (rust-overlay.packages.nightly."2024-01-01".rust  # Replace with actual date for Rust 1.77 nightly
        .override { extensions = [ "rust-src" "rustfmt" "clippy" ]; })
        nixpkgs.rustPlatform.rustPackages.flip-link
      ];

      # Optional: Set RUSTUP_TOOLCHAIN to ensure it always uses the specified toolchain
      shellHook = ''
        export RUSTUP_TOOLCHAIN=nightly-2024-01-01  # Rust 1.77 nightly
      '';
    };
  };
}
