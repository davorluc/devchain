{
  description = "A development shell for devchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "flashloan-dev";
      buildInputs = with pkgs; [
        cargo
        rustup
        rust-analyzer
      ];

      shellHook = ''
        echo "Entered development shell."
      '';
    };
  };
}
