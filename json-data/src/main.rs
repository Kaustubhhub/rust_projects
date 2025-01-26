use ::serde::{Deserialize, Serialize};

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
    let json = r#"
    {
        "article":"Writing json data in rust",
        "author":"Kaustubh Kumbhare",
        "paragraph": [
            {
                "name":"Heading of the paragraph"
            },
            {
                "name":"Body of the paragraph"
            },
            {
                "name":"Ending of the paragraph"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\nThe name of the paragraph is {}", parsed.paragraph[0].name);
    println!("The body of the paragraph is {}", parsed.paragraph[1].name);
    println!(
        "The ending of the paragraph is {}",
        parsed.paragraph[2].name
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}
