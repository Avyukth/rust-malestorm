use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize , Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize , Deserialize)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    payload: Payload,
}

#[derive(Debug, Clone, Serialize , Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload{
    Echo { echo: String },
}

fn main() {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    let input: Message = serde_json::from_reader(stdin).unwrap();
}
