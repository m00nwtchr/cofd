{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
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
    ];

  languages.rust = {
    enable = true;
    mold.enable = true;
    channel = "nightly";
  };

  processes = {
    cofdtools.exec = "cargo run";
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
    "cofd:tests" = {
      after = [ "devenv:enterTest" ];
      exec = "cargo nextest run";
    };
  };

  outputs = {
    # package Rust app using Nix
    # cofd = config.languages.rust.import ./. {};
  };
  # See full reference at https://devenv.sh/reference/options/
}
