import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const read = (path) => readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8");
const html = read("../index.html");
const pt = JSON.parse(read("../src/locales/pt.json"));
const keys = new Set([...html.matchAll(/data-i18n="([^"]+)"/g)].map(([, key]) => key));

test("every translatable element has a Portuguese string", () => {
  for (const key of keys) assert.ok(pt[key]?.trim(), `missing pt: ${key}`);
});

test("no Portuguese string is orphaned", () => {
  for (const key of Object.keys(pt)) assert.ok(keys.has(key), `unused pt: ${key}`);
});

test("translatable elements hold text only", () => {
  const nested = /<(\w+)[^>]*data-i18n="[^"]+"[^>]*>[^<]*<(?!\/\1)/g;
  assert.equal(html.match(nested), null, "a data-i18n element contains markup that would be lost");
});
