{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  cachix.pull = [
    "m00nwtchr"
    "cargo2nix"
  ];

  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
      # PDF parsing native dependencies (was cofd-miner/devenv.nix)
      pkg-config
      openssl
      freetype
      gumbo
      jbig2dec
      openjpeg
      libjpeg
      harfbuzz
      # build tools
      llvmPackages.clangUseLLVM # bindgen
      # GUI stack — meta-editor and cofdtools are eframe/egui binaries
      # that dlopen these at runtime (winit's wayland backend,
      # xkbcommon for keyboard, libglvnd/EGL/GL/GBM for rendering,
      # libX11 as the eframe X11 fallback). With default winit
      # features, all of these must be on LD_LIBRARY_PATH.
      wayland
      libxkbcommon
      libglvnd
      libdrm
      libX11
    ]
    ++ lib.optionals (!config.container.isBuilding) [
      git
      cargo-nextest
      cargo-audit
    ];

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  # GUI binaries built with default winit features dlopen wayland,
  # xkbcommon, EGL/GL/GBM, and X11 at runtime. The binary has no
  # baked-in rpath for these, so we put the lib paths on
  # LD_LIBRARY_PATH via the dynamic-loader wrapper.
  env.LD_LIBRARY_PATH = lib.makeLibraryPath [
    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.libglvnd
    pkgs.libdrm
    pkgs.libX11
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    mold.enable = true;
    channel = "stable";
    components = [
      "rust-src"
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  processes = {
    cofd.exec = "cargo run";
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
    oxfmt.enable = true;
  };

  tasks = {
    "cofd:tests" = {
      exec = "cargo nextest run --workspace";
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
