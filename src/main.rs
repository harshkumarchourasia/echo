use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{StdoutLock, Write};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
    Init  { node_id: String, node_ids: Vec<String> },
    InitOk
}

struct EchoNode;

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut  StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: input.body.id,
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n").context("writing trailing new line")
            }
            Payload::EchoOk { .. } => Ok(()),
            Payload::InitOk { .. } => Ok(()),
            Payload::Init { .. } => {
                let reply = Message{
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: input.body.id,
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    }
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n").context("writing trailing new line")
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut state = EchoNode;
    for input in inputs {
        let input: Message =
            input.context("Maelstrom input from STDIN can not be deserialized ")?;
        state
            .step(input, &mut stdout)
            .context("Node Step function failed")?;
        stdout.flush()?;
    }

    Ok(())
}
