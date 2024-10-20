{ inputs, pkgs, ... }:

let
  pkgs-unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };
in
{
  env.VERSION = "0.0.0-local";

  packages = with pkgs-unstable; [
    gtk4.dev
    libadwaita.dev
    pango.dev
    libgpg-error.dev
    gpgme.dev
    librsvg.dev
  ];

  languages.rust.enable = true;

  pre-commit.hooks.rustfmt.enable = true;
}
