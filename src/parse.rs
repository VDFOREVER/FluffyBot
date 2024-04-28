use select::{
    document::Document,
    predicate::{Attr, Name, Predicate},
};

pub struct Parse;

pub struct Data<'a> {
    pub class: &'a str,
    pub descendant: &'a str,
    pub attr: &'a str,
}

impl Parse {
    pub async fn get_html(html: &str, data: Data<'_>) -> Vec<String> {
        let document = Document::from(html);

        let mut results = Vec::new();

        for node in document.find(Attr("class", data.class).descendant(Name(data.descendant))) {
            if let Some(href) = node.attr(data.attr) {
                results.push(href.to_string());
            }
        }

        results
    }

    pub fn get_post() -> Data<'static> {
        Data {
            class: "thumbnail-preview",
            descendant: "a",
            attr: "href",
        }
    }

    pub fn get_image() -> Data<'static> {
        Data {
            class: "image-container note-container",
            descendant: "img",
            attr: "src",
        }
    }

    pub fn get_video() -> Data<'static> {
        Data {
            class: "gelcomVPlayer fit-width",
            descendant: "source",
            attr: "src",
        }
    }
}
