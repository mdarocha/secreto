{ inputs, pkgs, ... }:

let
  pkgs-unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };
in
{
  env = {
    APP_NAME = "Secreto";
    APP_ID = "pl.mdarocha.Secreto";
  };

  packages = with pkgs-unstable; [
    gtk4.dev
    libadwaita.dev
    pango.dev
    libgpg-error.dev
    gpgme.dev
  ];

  languages.rust.enable = true;

  pre-commit.hooks.rustfmt.enable = true;
}
