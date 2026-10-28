import { h } from "./dom";

// Widths echo a Markdown file: frontmatter, a title, then paragraphs of varying length.
const WIDTHS = [30, 92, 48, 30, 0, 55, 0, 96, 88, 70, 0, 40, 94, 82, 90, 64];

/** Placeholder in the shape of an agent file while it loads or generates. */
export function skeleton(): HTMLElement {
  return h(
    "div",
    { class: "skeleton", "aria-hidden": "true" },
    ...WIDTHS.map((width) => (width ? h("span", { style: `width: ${width}%` }) : h("i"))),
  );
}
