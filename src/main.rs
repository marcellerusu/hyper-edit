#[macro_use]
extern crate rocket;

use std::fs;

use biome_css_parser::CssParserOptions;
use db::*;
use html::Render;
use parse_utils::parse_selector;
use rocket::{
    http::ContentType,
    serde::{json::Json, Deserialize},
};

mod db;
mod html;
mod parse_utils;

fn css() -> String {
    fs::read_to_string("src/index.css").unwrap()
}

fn insert_property_js() -> String {
    fs::read_to_string("src/js/insert_property.js").unwrap()
}

fn delete_property_js() -> String {
    fs::read_to_string("src/js/delete_property.js").unwrap()
}

#[post("/src/<selector>/<name>", data = "<value>")]
fn insert(selector: String, name: String, value: Json<String>) {
    let mut db = DBTree::new();
    db.load("test.css");
    let selector = parse_selector(&selector);
    let parts = selector.to_path_parts();
    db.insert_mut(selector, &parts, &name, &value);
    fs::write("test.css", db.serialize()).unwrap()
}

#[delete("/src/<selector>/<name>")]
fn delete(selector: String, name: String) {
    let mut db = DBTree::new();
    db.load("test.css");
    let selector = parse_selector(&selector);
    let path = selector.to_path_parts();
    db.delete_mut(&path, &name);
    fs::write("test.css", db.serialize()).unwrap()
}

#[get("/src/<selector>")]
fn index(selector: String) -> (ContentType, String) {
    let mut db = DBTree::new();
    db.load("test.css");
    let selector = parse_selector(&selector);
    let path = selector.to_path_parts();
    let tree = db.get(&path).unwrap();
    let rule = tree.rule.as_ref().unwrap();
    let inherited_properties = db.inherited_properties_for(&path);

    let rule_html = format!(
        "
    <div data-kind=rule>
        <div data-attr=selector>{}</div>
        <div data-attr=properties>{}</div>
        <div data-attr=inherited_properties>{}</div>
    </div>
    ",
        rule.selector.render_html(),
        rule.properties
            .iter()
            .map(|p| p.render_html())
            .collect::<String>(),
        inherited_properties
            .iter()
            .map(|(_, p)| p.render_html())
            .collect::<String>()
    );

    (
        ContentType::HTML,
        format!(
            "<style>{}</style>
            <script>{}{}</script>
            <div class=\"--editor\" spellcheck=\"false\">{}<div>",
            css(),
            insert_property_js(),
            delete_property_js(),
            rule_html
        ),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, insert, delete])
}