use select::{
    document::Document,
    predicate::{Attr, Name, Predicate},
};

pub struct Parse;

pub struct Data {
    pub class: String,
    pub descendant: String,
    pub attr: String,
}

impl Parse {
    pub async fn get_html(html: &str, data: Data) -> Vec<String> {
        let document = Document::from(html);

        let mut results = Vec::new();

        for node in document.find(Attr("class", data.class.as_str()).descendant(Name(data.descendant.as_str()))) {
            if let Some(href) = node.attr(data.attr.as_str()) {
                results.push(href.to_string());
            }
        }

        results
    }

    pub fn get_post() -> Data {
        Data {
            class: "thumbnail-preview".to_string(),
            descendant: "a".to_string(),
            attr: "href".to_string(),
        }
    }

    pub fn get_image() -> Data {
        Data {
            class: "image-container note-container".to_string(),
            descendant: "img".to_string(),
            attr: "src".to_string(),
        }
    }

    pub fn get_video() -> Data {
        Data {
            class: "gelcomVPlayer fit-width".to_string(),
            descendant: "source".to_string(),
            attr: "src".to_string(),
        }
    }
}
