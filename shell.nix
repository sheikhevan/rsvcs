{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
  ];

  shellHook = ''
    echo "rsbackup v0.0.1"
    echo "author: Evan Alvarez"
  '';
}
