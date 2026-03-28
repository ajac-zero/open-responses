use open_responses::{
    client::Client, InputContentPart, InputItem, InputTextContentParam, UserMessageItemParam,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = Client::from_env()?
        .create_response()
        .model("gpt-4.1-mini")
        .input_item(InputItem::UserMessage(UserMessageItemParam {
            type_: "message".into(),
            role: "user".into(),
            content: Some(vec![InputContentPart::InputText(InputTextContentParam {
                type_: "input_text".into(),
                text: "Write a haiku about Rust.".into(),
            })]),
        }))
        .send()?;

    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
