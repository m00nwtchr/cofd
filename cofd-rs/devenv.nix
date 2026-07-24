{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
let
  name = "cofd";
in
{
  cachix.pull = [ "m00nwtchr" ];

  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
    ]
    ++ lib.optionals (!config.container.isBuilding) [
      git
      cargo-nextest
      cargo-audit
    ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    mold.enable = true;
    channel = "stable";
    components = [ "rust-src" ];
  };

  processes = {
    ${name}.exec = "cargo run";
  };

  treefmt = {
    enable = true;
    config.programs = {
      nixfmt.enable = true;
      rustfmt.enable = true;
    };
  };

  git-hooks.hooks = {
    treefmt.enable = true;
    clippy.enable = true;
  };

  tasks = {
    "${name}:tests" = {
      exec = "cargo nextest run";
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
