{
  description = "A flake to build a bare-metal RISC-V Rust project";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "aarch64-darwin";
    pkgs = nixpkgs.legacyPackages.${system};
    riscv-toolchain = import nixpkgs {
      localSystem = "${system}";
      crossSystem = {
        config = "riscv32-linux";
      };
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustup
        pkgs.cmake
        pkgs.gcc
        pkgs.binutils
      ];
      shellHook = ''
        rustup install nightly
        rustup default nightly
        rustup target add riscv32im-unknown-none-elf --toolchain nightly
        echo "Ready to build for bare-metal RISC-V!"
      '';
    };
  };
}
