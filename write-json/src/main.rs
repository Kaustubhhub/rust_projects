use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
fn main() {
    let article: Article = Article {
        article: String::from("writing json data in rust"),
        author: String::from("Kaustubh Kumbhare"),
        paragraph: vec![
            Paragraph {
                name: String::from("Heading of the paragraph"),
            },
            Paragraph {
                name: String::from("Body of the paragraph"),
            },
            Paragraph {
                name: String::from("Footer of the paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("The json data is : {}", json);
}
