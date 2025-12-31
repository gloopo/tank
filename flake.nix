{
    description = "basic bevy flake";
	inputs = {
	    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	};
	outputs = { self, nixpkgs }:
    let
	    pkgs = nixpkgs.legacyPackages."x86_64-linux";
	in {
	    devShells."x86_64-linux".default = pkgs.mkShell {
		nativeBuildInputs = [ pkgs.pkg-config ];
		buildInputs = with pkgs; [
            xorg.libX11 
            xorg.libXcursor 
            xorg.libXi 
            xorg.libXrandr 
            alsa-lib 
            wayland 
            libxkbcommon 
            udev
		];
		env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
	    env.LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath[
                pkgs.vulkan-loader
                pkgs.libxkbcommon
                pkgs.wayland
                pkgs.alsa-lib
                pkgs.udev
		    ]}";
		};
	};
}
