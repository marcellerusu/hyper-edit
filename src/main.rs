#[macro_use]
extern crate rocket;

use std::fs;

use db::*;
use html::{Render, RenderOptions};
use parse_utils::{parse_property, parse_selector};
use rocket::{http::ContentType, response::Redirect, serde::json::Json};

mod db;
mod html;
mod parse_utils;

fn css() -> String {
    fs::read_to_string("src/index.css").unwrap()
}

const JS_FILE_NAMES: [&str; 6] = [
    "insert_property",
    "toggle_property",
    "explore_siblings",
    "highlight_property",
    "update_value",
    "preview_var",
];

fn editor_js() -> String {
    JS_FILE_NAMES
        .iter()
        .map(|name| fs::read_to_string(format!("src/js/{}.js", name)).unwrap())
        .map(|js| format!("<script type=\"module\">{}</script>", js))
        .collect()
}

#[post("/src/<selector>", data = "<property>")]
fn insert(selector: &str, property: &str) {
    let property = parse_property(property).unwrap();
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    db.insert(selector, &path, property);
    fs::write("test.css", db.serialize()).unwrap()
}

#[post("/src/<selector>/<name>/disable")]
fn disable(selector: &str, name: String) {
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    db.set_state(&path, &name, State::Commented);
    fs::write("test.css", db.serialize()).unwrap()
}

#[post("/src/<selector>/<name>/enable")]
fn enable(selector: &str, name: String) {
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    db.set_state(&path, &name, State::Valid);
    fs::write("test.css", db.serialize()).unwrap()
}

#[post("/src/<selector>/<name>/value", data = "<value>")]
fn set_value(selector: &str, name: String, value: String) {
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    let property = parse_property(&format!("{}: {};", name, value)).unwrap();
    println!("{:?}", value);
    db.delete(&path, &name);
    db.insert(selector, &path, property);
    fs::write("test.css", db.serialize()).unwrap()
}

#[delete("/src/<selector>/<name>")]
fn delete(selector: &str, name: String) {
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    db.delete(&path, &name);
    fs::write("test.css", db.serialize()).unwrap()
}

#[get("/src/<selector>/<idx>/siblings")]
fn siblings(selector: &str, idx: usize) -> (ContentType, Json<Vec<Vec<(String, String)>>>) {
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    let sibling_path = &path[0..idx];
    let rest_of_path = &path[idx..];
    let siblings = db
        .siblings_with_subpath(sibling_path, rest_of_path)
        .iter()
        .map(|tree| tree.rule.as_ref().unwrap())
        .map(|rule| {
            rule.selector
                .to_css_db_path()
                .iter()
                .map(|part| {
                    let selector_html = if part == " " {
                        "so sad, what to do here :(".to_owned()
                    } else {
                        parse_selector(&part).render_html(&RenderOptions::default())
                    };
                    (part.to_owned(), selector_html)
                })
                .collect::<Vec<(String, String)>>()
        })
        .collect::<Vec<Vec<(String, String)>>>();

    (ContentType::JSON, Json::from(siblings))
}

#[get("/src/<selector>/<idx>")]
fn index_at(selector: &str, idx: usize) -> Redirect {
    let selector = parse_selector(selector);
    let path = selector.to_css_db_path();
    let sibling_path = &path[0..idx];

    Redirect::to(format!("/src/{}", sibling_path.join("")))
}

#[get("/src/<selector>")]
fn index(selector: String) -> (ContentType, String) {
    let mut db = CSSDB::new();
    db.load("test.css");
    let selector = parse_selector(&selector);
    let path = selector.to_css_db_path();
    let tree = db.get(&path).unwrap();
    let rule = tree.rule.as_ref().unwrap();
    let inherited_properties = db.inherited_properties_for(&path);
    let inherited_vars = db.inherited_vars_for(&path);
    let rule_html = format!(
        "
    <div data-kind=rule>
        <div data-attr=selector>{}</div>
        <div data-attr=properties>{}</div>
        <div data-attr=inherited-properties>{}</div>
    </div>
    ",
        rule.selector.render_html(&RenderOptions::default()),
        rule.properties
            .iter()
            .map(|p| p.render_html(&RenderOptions::default()))
            .collect::<String>(),
        inherited_properties
            .iter()
            .map(|(_, (selector, p))| format!(
                "<a href=\"{}?highlight_property_name={}\">{}</a>",
                selector.trim(),
                p.name(),
                p.render_html(&RenderOptions::default())
            ))
            .collect::<String>()
            + &inherited_vars
                .iter()
                .map(|(_, (selector, p))| format!(
                    "<a href=\"{}?highlight_property_name={}\">{}</a>",
                    selector.trim(),
                    p.name(),
                    p.render_html(&RenderOptions::default())
                ))
                .collect::<String>()
    );

    (
        ContentType::HTML,
        format!(
            "<style>{}</style>
            {}
            <div class=\"--editor\" spellcheck=\"false\">{}<div>",
            css(),
            editor_js(),
            rule_html
        ),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, insert, set_value, delete, enable, disable, siblings, index_at],
    )
}
