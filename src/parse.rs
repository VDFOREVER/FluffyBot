use reqwest;
use select::document::Document;
use select::predicate::Attr;
use select::predicate::Predicate;
use select::predicate::Name;
use std::error::Error;
use select::predicate::Class;

pub async fn parse_html(url: &str, class_name: &str, descendant: &str, attr: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    let document = Document::from(body.as_str());

    let mut results = Vec::new();

    for node in document.find(Attr("class", class_name).descendant(Name(descendant))) {
        if let Some(href) = node.attr(attr) {
            results.push(href.to_string());
        }
    }

    Ok(results)
}

pub async fn parse_text(url: &str, class_name: &str, descendant: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?; // Получение содержимого страницы
    let body = res.text().await?; // Извлечение текстового содержимого

    let mut results = Vec::new();

    let document = Document::from(body.as_str()); // Создание DOM из текстового содержимого

    for node in document.find(Class(class_name).descendant(Name(descendant))) {
        let mut author_name = String::new();
        let mut found = false;

        // Ищем тег "a" внутри узла
        if let Some(a_tag) = node.find(Name("a")).next() {
            let text = a_tag.text(); // Получаем текст из тега "a"
            if let Some(name) = text.trim().split('?').next() {
                author_name = name.trim().to_string(); // Удаляем пробелы и сохраняем имя
                found = true;
            }
        }

        // Если имя автора не найдено в теге "a", проверяем текст узла
        if !found {
            let text = node.text();
            let cleaned_text = text.trim().to_string();

            // Проверяем на пустую строку и символ "?"
            if !cleaned_text.is_empty() && cleaned_text != "?" {
                author_name = cleaned_text;
            }
        }

        // Если имя автора найдено, добавляем его в результаты
        if !author_name.is_empty() {
            results.push(author_name);
        }
    }

    Ok(results)
}
