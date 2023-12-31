{
  description = "Rodolphe's Advent of Code 2023 adventures";
  inputs =  {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }@inputs: 
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    buildInputs = with pkgs; [ 
        rustc
        cargo
        rust-analyzer
        git
    ];
    nativeBuildInputs = with pkgs; [ pkg-config ];
 in
  with pkgs; {  
    devShells.x86_64-linux.default = mkShell {
        inherit buildInputs nativeBuildInputs;
    };
  };
}
