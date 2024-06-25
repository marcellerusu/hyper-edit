let undo_stack = {
  MAX_SIZE: 1000,
  _stack: [],
  is_empty() {
    return this._stack.length === 0;
  },
  push(editor) {
    if (editor.length === this.MAX_SIZE) this._stack.shift();
    this._stack.push({
      selector: editor
        .querySelector(
          '[data-attr="selector"] > [data-kind][data-string-value]',
        )
        .dataset.stringValue.trim(),
      html: editor.innerHTML,
    });
  },
  pop() {
    return this._stack.pop();
  },
};

function init(editor) {
  editor.addEventListener("reload", (e) => {
    if (e.detail?.src === "undo") return;
    undo_stack.push(editor);
  });
}

function get_properties(html) {
  let elem = document.createElement("div");
  elem.innerHTML = html;
  let properties = [];
  for (let property of elem.querySelectorAll(
    '[data-attr="properties"] > [data-kind="property"]',
  )) {
    let is_commented = property.dataset.commented === "true";
    let name = property.querySelector('[data-attr="name"] > [data-value]')
      .dataset.value;
    let value = property.querySelector(
      '[data-attr="value"] > [data-kind][data-string-value]',
    ).dataset.stringValue;
    properties.push({ is_commented, name, value });
  }
  return properties;
}

window.addEventListener("keydown", async (e) => {
  if (e.key === "z" && e.metaKey) {
    if (undo_stack.is_empty()) return;
    let { selector, html } = undo_stack.pop();
    let editor = document.querySelector(`.--editor[data-url*='${selector}']`);
    let properties = get_properties(html);
    await fetch(
      `http://localhost:8000/src/${selector}/replace_all_properties`,
      {
        method: "POST",
        body: JSON.stringify(properties),
      },
    );
    editor.dispatchEvent(
      new CustomEvent("reload", { detail: { src: "undo" } }),
    );
  }
});

document.addEventListener("DOMContentLoaded", (_) => {
  let canvas = document.querySelector(".canvas");
  canvas.addEventListener("new-editor", (_) => {
    let editor = document.querySelector(".--editor:last-child");
    init(editor);
  });
});
