#!/usr/bin/env bash
#
# Build the Foundry system: compile cofd-foundry for wasm32-unknown-unknown,
# generate browser bindings with UniFFI, bundle the TypeScript system loader,
# stage the bindings and static system files under dist/cofd/, and zip the
# staged system.

set -euo pipefail

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$CRATE_DIR/../.." && pwd)"
system_ID="cofd"
PACKAGE_NAME="cofd-foundry"
UBRN_VERSION="0.31.0-6"
DIST_DIR="$CRATE_DIR/dist"
STAGE_DIR="$DIST_DIR/$system_ID"
UBRN="$CRATE_DIR/node_modules/.bin/ubrn"
ESBUILD="$CRATE_DIR/node_modules/.bin/esbuild"

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
if [ ! -x "$UBRN" ]; then
	echo "build-system.sh: uniffi-bindgen-react-native $UBRN_VERSION is required (run npm ci --prefix $CRATE_DIR)" >&2
	exit 1
fi
if [ ! -x "$ESBUILD" ]; then
	echo "build-system.sh: esbuild is required to bundle the TypeScript system loader (run npm ci --prefix $CRATE_DIR)" >&2
	exit 1
fi

if ! PYTHON="$(find_python)"; then
	echo "build-system.sh: python3 is required to package the system" >&2
	exit 1
fi

METADATA_FILE="$(mktemp)"
trap 'rm -f "$METADATA_FILE"' EXIT
cargo metadata --format-version 1 --no-deps --locked > "$METADATA_FILE"

(
	cd "$CRATE_DIR"
	rm -rf "$STAGE_DIR"
	"$UBRN" build wasm2 --config ubrn.config.yaml --release
)

# UBRN emits browser-neutral TypeScript bindings and its WASM player entrypoint.
# Bundle the authored TypeScript system loader with that entrypoint so Foundry
# only has to load one JavaScript module and the adjacent WASM asset.
cp "$CRATE_DIR/system/$system_ID.ts" "$STAGE_DIR/$system_ID-system.ts"
"$ESBUILD" \
	"$STAGE_DIR/$system_ID-system.ts" \
	--bundle \
	--format=esm \
	--platform=browser \
	--target=es2022 \
	--outfile="$STAGE_DIR/$system_ID.js"
rm "$STAGE_DIR/$system_ID-system.ts"
mkdir -p "$STAGE_DIR/templates"
cp "$CRATE_DIR/system/README.md" "$STAGE_DIR/README.md"
cp "$CRATE_DIR/system/$system_ID.css" "$STAGE_DIR/$system_ID.css"
cp "$CRATE_DIR/system/templates/actor-sheet.hbs" "$STAGE_DIR/templates/actor-sheet.hbs"

"$PYTHON" - "$METADATA_FILE" "$STAGE_DIR/system.json" "$PACKAGE_NAME" "$STAGE_DIR" "$system_ID" <<'PY'
import json
import pathlib
import sys

metadata_path = pathlib.Path(sys.argv[1])
manifest_path = pathlib.Path(sys.argv[2])
package_name = sys.argv[3]
stage_path = pathlib.Path(sys.argv[4]).resolve()
expected_id = sys.argv[5]

metadata = json.loads(metadata_path.read_text())
package = next((package for package in metadata["packages"] if package["name"] == package_name), None)
if package is None:
    raise SystemExit(f"package {package_name} not found in cargo metadata")

foundry = (package.get("metadata") or {}).get("foundry")
if not isinstance(foundry, dict):
    raise SystemExit(f"missing [package.metadata.foundry] for {package_name}")

manifest = dict(foundry)
manifest["version"] = package["version"]

required = {
    "id",
    "title",
    "description",
    "url",
    "compatibility",
    "esmodules",
    "styles",
    "languages",
    "packs",
    "media",
    "documentTypes",
    "readme",
}
missing = sorted(required - manifest.keys())
if missing:
    raise SystemExit(f"missing Foundry manifest fields: {', '.join(missing)}")
if manifest["id"] != expected_id:
    raise SystemExit(f"Foundry id {manifest['id']!r} does not match {expected_id!r}")

for field in ("id", "title", "description", "url", "readme", "version"):
    if not isinstance(manifest[field], str):
        raise SystemExit(f"Foundry manifest field {field} must be a string")

compatibility = manifest["compatibility"]
if not isinstance(compatibility, dict) or any(
    not isinstance(compatibility.get(field), str) for field in ("minimum", "verified")
):
    raise SystemExit("Foundry compatibility minimum and verified must be strings")

for field in ("esmodules", "styles", "languages", "packs", "media"):
    if not isinstance(manifest[field], list) or any(
        not isinstance(path, str) for path in manifest[field]
    ):
        raise SystemExit(f"Foundry manifest field {field} must be a list of strings")

if not isinstance(manifest["documentTypes"], dict):
    raise SystemExit("Foundry documentTypes must be an object")

def staged_file(path):
    candidate = (stage_path / path).resolve()
    try:
        candidate.relative_to(stage_path)
    except ValueError:
        raise SystemExit(f"staged Foundry asset escapes output directory: {path}")
    if not candidate.is_file():
        raise SystemExit(f"staged Foundry asset is missing: {path}")

for path in [manifest["readme"], *manifest["esmodules"], *manifest["styles"]]:
    staged_file(path)
staged_file("cofd_foundry.wasm")

def author_name(author):
    if not isinstance(author, str):
        raise SystemExit("Cargo package authors must be strings")
    if "<" in author and author.endswith(">"):
        author = author.rsplit("<", 1)[0].strip()
    return author

manifest["authors"] = [{"name": author_name(author)} for author in package["authors"]]

manifest_path.write_text(json.dumps(manifest, indent="\t") + "\n")
PY

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
