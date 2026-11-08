import * as generated from "./index";
import * as schemaEnums from "./cofd_schema";

const SYSTEM_ID = "cofd";
const ACTOR_SHEET_TEMPLATE = "systems/cofd/templates/actor-sheet.hbs";

type AnyRecord = Record<string, any>;

declare global {
  var Hooks: any;
  var foundry: any;
  var CONFIG: any;
  var cofdFoundry: AnyRecord;
  var cofdFoundryWasmReady: Promise<AnyRecord>;
}

const api: AnyRecord = {
  id: SYSTEM_ID,
  CharacterHandle: null,
  listAttributes: null,
  listDerivedTraits: null,
  listSkills: null,
  listSupportedTemplates: null,
  listTemplates: null,
  schemaEnums,
  version: null,
  ready: false,
  error: null,
  instance: null,
};

globalThis.cofdFoundry = api;

const ATTRIBUTE_GROUPS = [
  {
    key: "mental",
    label: "Mental",
    attributes: [
      { key: "intelligence", label: "Intelligence" },
      { key: "wits", label: "Wits" },
      { key: "resolve", label: "Resolve" },
    ],
  },
  {
    key: "physical",
    label: "Physical",
    attributes: [
      { key: "strength", label: "Strength" },
      { key: "dexterity", label: "Dexterity" },
      { key: "stamina", label: "Stamina" },
    ],
  },
  {
    key: "social",
    label: "Social",
    attributes: [
      { key: "presence", label: "Presence" },
      { key: "manipulation", label: "Manipulation" },
      { key: "composure", label: "Composure" },
    ],
  },
];

const SKILLS = [
  { key: "academics", label: "Academics", category: "mental" },
  { key: "computer", label: "Computer", category: "mental" },
  { key: "crafts", label: "Crafts", category: "mental" },
  { key: "investigation", label: "Investigation", category: "mental" },
  { key: "medicine", label: "Medicine", category: "mental" },
  { key: "occult", label: "Occult", category: "mental" },
  { key: "politics", label: "Politics", category: "mental" },
  { key: "science", label: "Science", category: "mental" },
  { key: "athletics", label: "Athletics", category: "physical" },
  { key: "brawl", label: "Brawl", category: "physical" },
  { key: "drive", label: "Drive", category: "physical" },
  { key: "firearms", label: "Firearms", category: "physical" },
  { key: "larceny", label: "Larceny", category: "physical" },
  { key: "stealth", label: "Stealth", category: "physical" },
  { key: "survival", label: "Survival", category: "physical" },
  { key: "weaponry", label: "Weaponry", category: "physical" },
  { key: "animalKen", label: "Animal Ken", category: "social" },
  { key: "empathy", label: "Empathy", category: "social" },
  { key: "expression", label: "Expression", category: "social" },
  { key: "intimidation", label: "Intimidation", category: "social" },
  { key: "persuasion", label: "Persuasion", category: "social" },
  { key: "socialize", label: "Socialize", category: "social" },
  { key: "streetwise", label: "Streetwise", category: "social" },
  { key: "subterfuge", label: "Subterfuge", category: "social" },
];

const MORTAL_DEFAULTS = {
  template: "Mortal",
  concept: "",
  virtue: "",
  vice: "",
  chronicle: "",
  size: 5,
  attributes: Object.fromEntries(
    ATTRIBUTE_GROUPS.flatMap((group) => group.attributes.map(({ key }) => [key, { value: 1 }])),
  ),
  skills: Object.fromEntries(SKILLS.map(({ key }) => [key, { value: 0, specialties: "" }])),
  integrity: { label: "Integrity", value: 7, max: 10 },
  health: { value: 0, max: 6 },
  willpower: { value: 0, max: 2 },
  notes: "",
};

function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value));
}

function mergeDefaults(target: AnyRecord, defaults: AnyRecord): AnyRecord {
  for (const [key, value] of Object.entries(defaults)) {
    if (target[key] === undefined) {
      target[key] = clone(value);
    } else if (
      value !== null &&
      typeof value === "object" &&
      !Array.isArray(value) &&
      typeof target[key] === "object" &&
      target[key] !== null &&
      !Array.isArray(target[key])
    ) {
      mergeDefaults(target[key], value);
    }
  }
  return target;
}

function mortalSystemData(existing: AnyRecord | undefined): AnyRecord {
  return mergeDefaults(clone(existing ?? {}), MORTAL_DEFAULTS);
}

function numberValue(value: unknown, fallback = 0): number {
  const result = Number(value);
  return Number.isFinite(result) ? result : fallback;
}

function prepareAttributes(system: AnyRecord): AnyRecord[] {
  return ATTRIBUTE_GROUPS.map((group) => ({
    ...group,
    attributes: group.attributes.map((attribute) => ({
      ...attribute,
      value: numberValue(system.attributes?.[attribute.key]?.value, 0),
    })),
  }));
}

function prepareSkills(system: AnyRecord): AnyRecord[] {
  return SKILLS.map((skill) => ({
    ...skill,
    value: numberValue(system.skills?.[skill.key]?.value, 0),
    specialties: system.skills?.[skill.key]?.specialties ?? "",
  }));
}

function prepareTrack(
  system: AnyRecord,
  key: string,
  label: string,
  fallbackValue: number,
  fallbackMax: number,
  derivedMax: number | null = null,
): AnyRecord {
  const track = system[key] ?? {};
  return {
    key,
    label: track.label ?? label,
    value: numberValue(track.value, fallbackValue),
    max: derivedMax ?? numberValue(track.max, fallbackMax),
    derived: derivedMax !== null,
  };
}

function derivedMortalValues(system: AnyRecord): { health: number | null; willpower: number | null } {
  if (!api.ready) return { health: null, willpower: null };

  try {
    const character = new api.CharacterHandle(system.template ?? "Mortal");
    try {
      for (const group of ATTRIBUTE_GROUPS) {
        for (const attribute of group.attributes) {
          character.setTrait(
            attribute.key,
            numberValue(system.attributes?.[attribute.key]?.value, 1),
          );
        }
      }
      character.setTrait("Size", numberValue(system.size, 5));

      return {
        health: character.getTrait("Health") ?? 0,
        willpower: character.getTrait("Willpower") ?? 0,
      };
    } finally {
      character.uniffiDestroy();
    }
  } catch (error) {
    console.error(`${SYSTEM_ID} | failed to evaluate character`, error);
    return { health: null, willpower: null };
  }
}

function wasmContext(): AnyRecord {
  return {
    ready: api.ready,
    loading: !api.ready && !api.error,
    status: api.ready ? "Ready" : api.error ? "Unavailable" : "Loading",
    version: api.version,
    error: api.error?.message ?? null,
  };
}

function textField(fields: AnyRecord, initial = "") {
  return new fields.StringField({ required: false, blank: true, initial });
}

function numberField(fields: AnyRecord, initial: number, max = 99) {
  return new fields.NumberField({ integer: true, initial, min: 0, max });
}

function attributeSchema(fields: AnyRecord) {
  return new fields.SchemaField(
    Object.fromEntries(
      ATTRIBUTE_GROUPS.flatMap((group) =>
        group.attributes.map(({ key }) => [
          key,
          new fields.SchemaField({ value: numberField(fields, 1, 5) }),
        ]),
      ),
    ),
  );
}

function skillSchema(fields: AnyRecord) {
  return new fields.SchemaField(
    Object.fromEntries(
      SKILLS.map(({ key }) => [
        key,
        new fields.SchemaField({
          value: numberField(fields, 0, 5),
          specialties: textField(fields),
        }),
      ]),
    ),
  );
}

function trackSchema(fields: AnyRecord, value: number, max: number, label?: string) {
  const schema = {
    value: numberField(fields, value),
    max: numberField(fields, max),
  };
  if (label) schema.label = textField(fields, label);
  return new fields.SchemaField(schema);
}

async function loadWasm() {
  await generated.uniffiInitAsync(new URL("./cofd_foundry.wasm", import.meta.url));
  return generated;
}

const ready = loadWasm()
  .then((module) => {
    const exports = module as AnyRecord;
    api.instance = exports;
    api.CharacterHandle = exports.CharacterHandle;
    for (const name of [
      "listAttributes",
      "listDerivedTraits",
      "listSkills",
      "listSupportedTemplates",
      "listTemplates",
    ]) {
      api[name] = exports[name];
    }
    api.version = exports.version();
    api.ready = true;
    return api;
  })
  .catch((error) => {
    api.error = error;
    throw error;
  });

globalThis.cofdFoundryWasmReady = ready;
ready.catch((error) => {
  console.error(`${SYSTEM_ID} | wasm failed to load`, error);
});

if (typeof Hooks !== "undefined") {
  Hooks.once("init", () => {
    const { HandlebarsApplicationMixin } = foundry.applications.api;
    const { ActorSheetV2 } = foundry.applications.sheets;
    const { DocumentSheetConfig } = foundry.applications.apps;

    class CofDActorSheet extends HandlebarsApplicationMixin(ActorSheetV2) {
      static DEFAULT_OPTIONS = {
        classes: ["cofd", "actor-sheet"],
        position: { width: 900, height: 760 },
        tag: "form",
        form: { submitOnChange: true },
      };

      static PARTS = {
        body: { template: ACTOR_SHEET_TEMPLATE },
      };

      async _prepareContext(options: AnyRecord) {
        const context = await super._prepareContext(options);
        const actor = this.document;
        const system = actor.system ?? {};
        const derived = derivedMortalValues(system);

        return {
          ...context,
          actor,
          system,
          editable: this.isEditable,
          attributes: prepareAttributes(system),
          skills: prepareSkills(system),
          derived,
          tracks: [
            prepareTrack(system, "integrity", "Integrity", 7, 10),
            prepareTrack(system, "health", "Health", 0, 6, derived.health),
            prepareTrack(system, "willpower", "Willpower", 0, 2, derived.willpower),
          ],
          wasm: wasmContext(),
        };
      }
    }

    class CofDCharacterData extends foundry.abstract.TypeDataModel {
      static defineSchema() {
        const fields = foundry.data.fields;
        return {
          template: new fields.StringField({ initial: "Mortal" }),
          concept: textField(fields),
          virtue: textField(fields),
          vice: textField(fields),
          chronicle: textField(fields),
          size: new fields.NumberField({ integer: true, initial: 5, min: 1, max: 20 }),
          attributes: attributeSchema(fields),
          skills: skillSchema(fields),
          integrity: trackSchema(fields, 7, 10, "Integrity"),
          health: trackSchema(fields, 0, 6),
          willpower: trackSchema(fields, 0, 2),
          notes: textField(fields),
        };
      }
    }

    Object.assign(CONFIG.Actor.dataModels, { character: CofDCharacterData });
    DocumentSheetConfig.registerSheet(foundry.documents.Actor, SYSTEM_ID, CofDActorSheet, {
      makeDefault: true,
      types: ["character"],
    });

    Hooks.on("preCreateActor", (actor: AnyRecord, data: AnyRecord) => {
      if (actor.type !== "character") return;
      actor.updateSource({ system: mortalSystemData(data?.system ?? actor.system) });
    });

    globalThis.cofdFoundry.actorSheet = CofDActorSheet;
    console.log(`${SYSTEM_ID} | character actor sheet registered`);

    const refreshActorSheets = () => {
      for (const application of globalThis.foundry?.applications?.instances?.values?.() ?? []) {
        if (application instanceof CofDActorSheet && application.rendered) {
          application.render();
        }
      }
    };

    Hooks.once("ready", () => {
      ready.then(refreshActorSheets, refreshActorSheets);
    });
  });

  Hooks.once("init", () => {
    ready
      .then(({ version }) => {
        console.log(`${SYSTEM_ID} | CofD engine ready (wasm ${version})`);
      })
      .catch(() => {});
  });
}
