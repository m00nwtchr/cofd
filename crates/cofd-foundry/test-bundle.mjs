import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";

const wasm = new URL("./dist/cofd/cofd_foundry.wasm", import.meta.url);
const server = createServer(async (request, response) => {
  if (request.url !== "/cofd_foundry.wasm") {
    response.writeHead(404).end();
    return;
  }
  response.writeHead(200, { "content-type": "application/wasm" });
  response.end(await readFile(wasm));
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const { port } = server.address();
const fetch = globalThis.fetch;
globalThis.fetch = (input, init) => {
  const source = input instanceof URL ? input.href : String(input);
  return source.startsWith("file:")
    ? fetch(`http://127.0.0.1:${port}/cofd_foundry.wasm`, init)
    : fetch(input, init);
};

try {
  await import(new URL("./dist/cofd/cofd.js", import.meta.url));
  const api = await globalThis.cofdFoundryWasmReady;
  assert.equal(api.ready, true);
  assert.equal(api.version, "0.1.0");
  assert.equal(api.schemaEnums.Template.Mortal, 0);

  const character = new api.CharacterHandle("Mortal");
  try {
    character.setTrait("Stamina", 3);
    character.setTrait("Size", 5);
    assert.equal(character.getTrait("Health"), 8);
    assert.equal(character.getTrait("Willpower"), 2);
  } finally {
    character.uniffiDestroy();
  }
} finally {
  server.close();
}

console.log("cofd Foundry bundle smoke test passed");
