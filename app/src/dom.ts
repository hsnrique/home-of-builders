type Child = Node | string | null | undefined | false;
type Handler = (event: Event) => void;
type AttrValue = string | number | boolean | Handler | null | undefined;

/** Builds elements from text nodes only: agent files and model output never become HTML. */
export function h<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  attrs: Record<string, AttrValue> = {},
  ...children: Child[]
): HTMLElementTagNameMap[K] {
  const el = document.createElement(tag);
  el.append(...children.filter((c): c is Node | string => c !== null && c !== undefined && c !== false));
  // After children, so `value` on a <select> can match one of its options.
  for (const [key, value] of Object.entries(attrs)) applyAttr(el, key, value);
  return el;
}

function applyAttr(el: HTMLElement, key: string, value: AttrValue): void {
  if (value === null || value === undefined || value === false) return;
  if (typeof value === "function") el.addEventListener(key.replace(/^on/, ""), value);
  else if (key === "value") (el as HTMLInputElement).value = String(value);
  else el.setAttribute(key, value === true ? "" : String(value));
}
