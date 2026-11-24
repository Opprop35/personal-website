use leptos::prelude::*;

const RESPONSIVE: &str = "width=device-width, initial-scale=1";
const A4: &str = "width=794";

pub fn apply_viewport(pathname: &str) {
    let content = viewport_for(pathname);
    let doc = document();

    let meta = match doc.query_selector("meta[name=viewport]").ok().flatten() {
        Some(meta) => meta,
        None => ensure_meta(&doc),
    };

    if meta.get_attribute("content").as_deref() != Some(content) {
        meta.set_attribute("content", content)
            .expect("could not update the viewport meta");
    }
}

fn viewport_for(pathname: &str) -> &'static str {
    if pathname == "/resume" || pathname == "/resume/" {
        A4
    } else {
        RESPONSIVE
    }
}

fn ensure_meta(doc: &web_sys::Document) -> web_sys::Element {
    let head = doc
        .query_selector("head")
        .expect("could not query <head>")
        .expect("no <head> element");

    let meta = doc
        .create_element("meta")
        .expect("could not create the viewport <meta> element");
    meta.set_attribute("name", "viewport")
        .expect("could not set name=viewport");

    head.insert_adjacent_element("afterbegin", &meta)
        .expect("could not insert <meta> into <head>")
        .expect("inserted <meta> element missing");

    meta
}
