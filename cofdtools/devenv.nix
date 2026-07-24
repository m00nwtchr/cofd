{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  # https://devenv.sh/basics/
  # env.GREET = "devenv";

  cachix.pull = [
    "m00nwtchr"
    "cargo2nix"
  ];

  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
      pkg-config
      # dioxus deps
      pango
      atk
      libsoup_3
      gdk-pixbuf
      webkitgtk_4_1
      xdotool
      # build tools
      tailwindcss_4

      dioxus-cli
      wasm-bindgen-cli
    ]
    ++ lib.optionals (!config.container.isBuilding) [
      git
      cargo-nextest
    ];

  languages.rust = {
    enable = true;
    # mold.enable = lib.mkForce false;
    targets = [
      "x86_64-unknown-linux-gnu"
      "wasm32-unknown-unknown"
    ];
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

  enterTest = ''
    which clang
    clang -v 2>&1 | head -n 30
    ls -la ${pkgs.glibc.dev}/lib/Scrt1.o
  '';

  # tasks = {
  #   "cofdtools:tests" = {
  #     after = ["devenv:enterTest"];
  #     exec = "cargo nextest run";
  #   };
  # };

  outputs = {
    # package Rust app using Nix
    cofdtools = config.languages.rust.import ./. { };
  };
  # See full reference at https://devenv.sh/reference/options/
}
