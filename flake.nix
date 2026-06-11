{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs = { nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    libs = with pkgs; [
      wayland
      libxkbcommon
      libGL
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [ rustup ] ++ libs;
      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libs}:$LD_LIBRARY_PATH
      '';
    };
  };
}
