import fs from "fs/promises";
import { parse_rule, parse_query } from "./parser/css-parser.js";
import { run } from "./parser/parse-utils.js";
import { AssertionError } from "assert";
import query_to_file_path from "./query_to_file_path.js";

function assert(cond, message) {
  if (!cond) {
    throw new AssertionError({ message });
  }
}

function to_html(node) {
  assert(typeof node !== "number", "do not yet support numbers");
  if (typeof node === "string") {
    let attrs = "";
    if (!Number.isNaN(Number(node))) {
      attrs += `data-numeric="true"`;
    }
    return `<div ${attrs} data-value="${node}" contenteditable>${node}</div>`;
  } else if (Array.isArray(node)) {
    return `<div data-array="true">`;
  }

  console.log(node);
  assert(node.kind, "expected ast node");

  let attr_only_keys = [];
  let attrs = ``;
  for (let key in node) {
    if (key === "kind") continue;
    if (node[key].attr_only) {
      attr_only_keys.push(node[key].value);
    } else if (Array.isArray(node[key])) {
      attrs += `<div data-attr="${key}" data-array="true">${node[key].map(to_html).join("")}</div>`;
    } else {
      attrs += `<div data-attr="${key}">${to_html(node[key])}</div>`;
    }
  }

  return `
    <div
      data-kind="${node.kind}"
      data-attr-only-keys="${attr_only_keys.join(",")}"
    >
      ${attrs}
    </div>
  `;
}

let css = (await fs.readFile("index.css")).toString();

export async function render(req, next, res, query) {
  try {
    let url = decodeURI(query);
    console.log({ url });
    let parsed_query = parse_query[run](url, /^$/)[0];
    console.log({ parsed_query });

    let path = query_to_file_path(parsed_query).join("/");
    console.log(path);
    let file = (await fs.readFile(`./css/${path}/index.css`)).toString();
    let ast = parse_rule[run](file)[0];
    let html = to_html(ast);
    res.end(`
  <html>
    <head>
      <style type="text/css">${css}</style>
    </head>
    <body>
      <div class="--css-edit-editor" spellcheck="false">
        ${html}
      </div>  
    </body>
  </div>
  `);
  } catch (e) {
    console.error(e);
    console.log(":(");
    res.end(`no file found`);
  }
}
