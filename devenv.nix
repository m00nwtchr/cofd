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
      podman # Foundry VTT development container
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
    targets = [
      "x86_64-unknown-linux-gnu"
      "wasm32-unknown-unknown"
    ];

    components = [
      "rust-src"
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  dotenv.enable = true;

  processes = {
    cofd.exec = "cargo run";

    foundry = {
      after = [ "cofd-foundry:build" ];
      exec = ''
        set -euo pipefail

        if [ -z "''${FOUNDRY_LICENSE_KEY:-}" ] \
          && { [ -z "''${FOUNDRY_USERNAME:-}" ] || [ -z "''${FOUNDRY_PASSWORD:-}" ]; }; then
          echo "Set FOUNDRY_LICENSE_KEY or both FOUNDRY_USERNAME and FOUNDRY_PASSWORD" >&2
          exit 1
        fi
        if [ -z "''${FOUNDRY_ADMIN_KEY:-}" ]; then
          echo "FOUNDRY_ADMIN_KEY must be set to start Foundry VTT" >&2
          exit 1
        fi
        if [ -z "''${FOUNDRY_RELEASE_URL:-}" ] \
          && { [ -z "''${FOUNDRY_USERNAME:-}" ] || [ -z "''${FOUNDRY_PASSWORD:-}" ]; }; then
          echo "Set FOUNDRY_RELEASE_URL or both FOUNDRY_USERNAME and FOUNDRY_PASSWORD" >&2
          exit 1
        fi

        mkdir -p .devenv/foundry-data

        exec ${pkgs.podman}/bin/podman run --rm --replace \
          --name cofd-foundry \
          --hostname cofd-foundry.local \
          --userns=keep-id \
          --publish 127.0.0.1:30000:30000/tcp \
          --volume "$PWD/crates/cofd-foundry/dist/cofd-foundry:/data/Data/modules/cofd-foundry:ro" \
          --volume "$PWD/.devenv/foundry-data:/data" \
          --env FOUNDRY_USERNAME \
          --env FOUNDRY_PASSWORD \
          --env FOUNDRY_RELEASE_URL \
          --env FOUNDRY_LICENSE_KEY \
          --env FOUNDRY_ADMIN_KEY \
          --env FOUNDRY_TELEMETRY=false \
          --env FOUNDRY_HOT_RELOAD=true \
          --env FOUNDRY_MINIFY_STATIC_FILES=false \
          ghcr.io/felddy/foundryvtt:14
      '';
      ready.http.get = {
        host = "127.0.0.1";
        port = 30000;
        path = "/";
      };
      ready.initial_delay = 30;
      ready.probe_timeout = 5;
      ready.failure_threshold = 60;
    };
  };

  tasks."cofd-foundry:build".exec = "crates/cofd-foundry/build-module.sh";

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
