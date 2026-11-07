const MODULE_ID = "cofd-foundry";
const WASM_URL = new URL("../cofd_foundry.wasm", import.meta.url);

const api = {
  id: MODULE_ID,
  version: null,
  templateCount: 0,
  mortalCharacterSize: 0,
  ready: false,
  error: null,
  instance: null,
};

globalThis.cofdFoundry = api;

function decodeString(memory, pointer, length) {
  const bytes = new Uint8Array(memory.buffer, pointer, length);
  return new TextDecoder("utf-8").decode(bytes);
}

async function loadWasm() {
  const response = await fetch(WASM_URL);
  if (!response.ok) {
    throw new Error(`${MODULE_ID}: failed to fetch ${WASM_URL.href} (${response.status})`);
  }
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return instance;
}

const ready = loadWasm()
  .then((instance) => {
    const exports = instance.exports;
    api.instance = instance;
    api.version = decodeString(
      exports.memory,
      exports.cofd_foundry_version_ptr(),
      exports.cofd_foundry_version_len(),
    );
    api.templateCount = exports.cofd_foundry_template_count();
    api.mortalCharacterSize = exports.cofd_foundry_mortal_character_size();
    api.ready = true;
    return api;
  })
  .catch((error) => {
    api.error = error;
    throw error;
  });

globalThis.cofdFoundryWasmReady = ready;
ready.catch((error) => {
  console.error(`${MODULE_ID} | wasm failed to load`, error);
});

if (typeof Hooks !== "undefined") {
  Hooks.once("init", () => {
    ready
      .then(({ version, templateCount, mortalCharacterSize }) => {
        console.log(
          `${MODULE_ID} | CofD engine ready (wasm ${version}, ${templateCount} templates, mortal character ${mortalCharacterSize} bytes)`,
        );
      })
      .catch(() => {});
  });
}
