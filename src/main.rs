use std::io::StdoutLock;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
    InitOk{ node_id: String , node_ids:Vec<String>},
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn handle(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
        ) -> anyhow::Result<()> {
            match input.body.payload {
                Payload::Echo { echo } => {
                    let reply = Message {
                        src: input.dst,
                        dst: input.src,
                        body: Body {
                            id: Some(self.id),
                            in_reply_to: input.body.id,
                            ty: "reply".to_string(),
                            payload: Payload::EchoOk { echo },
                        },
                    };
                    serde_json::to_writer(&mut *output, &reply).context("Serialize response to init")?;
                    output.write_all(b"\n").context("write trailing newline")?;
                    self.id += 1;
                }
                
                Payload::EchoOk { .. } => {},

                Payload::InitOk{..} => bail!("received init ok message")
        }
        Ok(())
    }
}

fn main() {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    let mut state = EchoNode { id: 0 };
    let mut output = serde_json::Serializer::new(stdout);
    for input in inputs {
        let input = input.context("Maelstrom input from STDIN could not be deserialized");
        state
        .handle(input, &mut output)
        .context("Malestorm output to STDOUT could not be serialized");

    }
    Ok(())
}
