{
  description = "Build environment for pico-pet";
  inputs =
  {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs, ... }@inputs:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system}.default = pkgs.mkShell
    {
      packages = with pkgs; [
        rustc
        cargo
        alsaLib
        pkg-config
        udev
      ];
      shellHook = ''
        rustup toolchain install nightly-2023-11-16
        rustup default nightly-2023-11-16
        rustup target add thumbv6m-none-eabi
        export RUSTC=$(rustup which rustc)
      '';
    };
  };
}
