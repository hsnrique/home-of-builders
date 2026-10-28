import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

const src = fileURLToPath(new URL("../src/", import.meta.url));
const locale = (lang) => JSON.parse(readFileSync(join(src, "locales", `${lang}.json`), "utf8"));
const en = locale("en");
const pt = locale("pt");

function sourceFiles(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) return sourceFiles(path);
    return entry.name.endsWith(".ts") ? [path] : [];
  });
}

test("both locales define the same keys", () => {
  assert.deepEqual(Object.keys(pt).sort(), Object.keys(en).sort());
});

test("no translation is empty", () => {
  for (const [lang, messages] of Object.entries({ en, pt })) {
    for (const [key, text] of Object.entries(messages)) assert.ok(text.trim(), `${lang}:${key} is empty`);
  }
});

test("placeholders match across locales", () => {
  const vars = (text) => (text.match(/\{\w+\}/g) ?? []).sort();
  for (const key of Object.keys(en)) assert.deepEqual(vars(pt[key]), vars(en[key]), key);
});

test("every key the code references exists", () => {
  const prefixes = new Set(Object.keys(en).map((key) => key.split(".")[0]));
  const literal = /["']([a-z]+\.[A-Za-z.]+)["']/g;
  for (const file of sourceFiles(src)) {
    for (const [, key] of readFileSync(file, "utf8").matchAll(literal)) {
      if (prefixes.has(key.split(".")[0])) assert.ok(key in en, `${file}: unknown key ${key}`);
    }
  }
});

test("backend error codes are translated", () => {
  const rust = fileURLToPath(new URL("../src-tauri/src/", import.meta.url));
  const rustFiles = readdirSync(rust, { recursive: true }).filter((f) => String(f).endsWith(".rs"));
  for (const file of rustFiles) {
    for (const [, key] of readFileSync(join(rust, String(file)), "utf8").matchAll(/"(error\.[A-Za-z]+)"/g)) {
      assert.ok(key in en, `${file}: untranslated backend error ${key}`);
    }
  }
});
