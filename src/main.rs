use std::error::Error;
use scraper::{Html, Selector};
use regex::Regex;


use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

// exepect OPENAI_API_KEY env var to be set

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        return Err("Missing url argument".into());
    }
    let url = &argv[1];

    let body = reqwest::get(url).await?.text().await?;
    let doc = Html::parse_document(&body);

    let selector = Selector::parse("body").unwrap();
    let text = doc.select(&selector).next().unwrap().text().collect::<Vec<_>>();

    let regex_multispace = Regex::new(r" +").unwrap();
    let regex_multiline = Regex::new(r"(\r?\n|\r)+").unwrap();

    let text = text
    .iter()
    .map(|s| {

        regex_multiline.replace_all(
            &regex_multispace.replace_all(s.trim(), " ").into_owned(),
            "\n"
        ).into_owned()
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let client = Client::new();
    let prompt = format!("Resume next text \n\n{}", text);

    println!("{}\n\nLength: {}\n\nResume:\n", text.len(), prompt);

    let chat = client.chat();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(prompt)
                .build()?,
        ])
        .build()?;

    let response = chat.create(request).await?;

    for choice in response.choices {
        if let Some(message) = choice.message.content {
            println!("{}",  message);
        }
    }

    Ok(())
}
