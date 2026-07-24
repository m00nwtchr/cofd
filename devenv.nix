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
    ]
    ++ lib.optionals (!config.container.isBuilding) [
      git
      cargo-nextest
      cargo-audit
    ];

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    mold.enable = true;
    # Force nightly: cofd-miner pulls oxidize-pdf and other crates
    # that require nightly-only features. The cofd-rs sub-project has
    # been migrated to stable (rust-toolchain.toml); the override
    # here applies only when the nightly toolchain is needed.
    channel = "stable";
    components = [ "rust-src" ];
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
  };

  tasks = {
    "cofd:tests" = {
      exec = "cargo nextest run --workspace";
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}