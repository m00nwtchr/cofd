#!/usr/bin/env bash
#
# Build the Foundry system: compile cofd-foundry for wasm32-unknown-unknown,
# generate browser bindings with wasm-bindgen, stage the bindings and static
# system files under dist/cofd/, and zip the staged system.

set -euo pipefail

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$CRATE_DIR/../.." && pwd)"
system_ID="cofd"
TARGET="wasm32-unknown-unknown"
WASM_NAME="cofd_foundry.wasm"
WASM_BINDGEN_NAME="cofd_foundry"
WASM_BINDGEN_VERSION="0.2.126"
DIST_DIR="$CRATE_DIR/dist"
STAGE_DIR="$DIST_DIR/$system_ID"

find_python() {
	if command -v python3 >/dev/null 2>&1; then
		command -v python3
		return 0
	fi
	if command -v python >/dev/null 2>&1; then
		command -v python
		return 0
	fi
	for candidate in /nix/store/*-python3-*/bin/python3; do
		if [ -x "$candidate" ]; then
			printf '%s\n' "$candidate"
			return 0
		fi
	done
	return 1
}

cd "$WORKSPACE_DIR"
if ! command -v wasm-bindgen >/dev/null 2>&1; then
	echo "build-system.sh: wasm-bindgen CLI is required (provided by devenv)" >&2
	exit 1
fi
CLI_VERSION="$(wasm-bindgen --version | awk '{print $2}')"
if [ "$CLI_VERSION" != "$WASM_BINDGEN_VERSION" ]; then
	echo "build-system.sh: wasm-bindgen $WASM_BINDGEN_VERSION is required; found $CLI_VERSION" >&2
	exit 1
fi

cargo build -p "$system_ID" --target "$TARGET" --release --locked

rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR/templates"

wasm-bindgen \
	"target/$TARGET/release/$WASM_NAME" \
	--target web \
	--out-dir "$STAGE_DIR" \
	--out-name "$WASM_BINDGEN_NAME"
cp "$CRATE_DIR/system/system.json" "$STAGE_DIR/system.json"
cp "$CRATE_DIR/system/README.md" "$STAGE_DIR/README.md"
cp "$CRATE_DIR/system/$system_ID.js" "$STAGE_DIR/$system_ID.js"
cp "$CRATE_DIR/system/$system_ID.css" "$STAGE_DIR/$system_ID.css"
cp "$CRATE_DIR/system/templates/actor-sheet.hbs" "$STAGE_DIR/templates/actor-sheet.hbs"

if ! PYTHON="$(find_python)"; then
	echo "build-system.sh: python3 is required to package the system" >&2
	exit 1
fi

"$PYTHON" - "$STAGE_DIR" "$STAGE_DIR/$system_ID.zip" <<'PY'
import pathlib
import sys
import zipfile

stage = pathlib.Path(sys.argv[1]).resolve()
archive = pathlib.Path(sys.argv[2]).resolve()

with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
    for path in sorted(stage.rglob("*")):
        if not path.is_file() or path.resolve() == archive:
            continue
        bundle.write(path, path.relative_to(stage.parent))

print(f"created {archive}")
PY

echo "cofd-foundry: system staged in $STAGE_DIR"
