use std::io::{StdoutLock, Write};

use anyhow::Context;
use gossip::message::{Body, Message};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

struct EchoNode {
    id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

impl EchoNode {
    pub fn step<'a>(
        &mut self,
        input: Message<EchoPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let reply = match input.body.payload {
            EchoPayload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: EchoPayload::InitOk {},
                    },
                };
                Some(reply)
            }
            EchoPayload::InitOk => None,

            EchoPayload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: EchoPayload::EchoOk { echo: echo.clone() },
                    },
                };

                Some(reply)
            }
            EchoPayload::EchoOk { .. } => None,
        };

        if let Some(reply) = reply {
            serde_json::to_writer(&mut *output, &reply).context("failed to serialize node")?;
            output.write_all(b"\n").context("write trialing newline")?;
            self.id += 1
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let inputs = Deserializer::from_reader(stdin).into_iter::<Message<EchoPayload>>();

    let mut node = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("cannot deserialize node from maelstrom")?;

        node.step(input, &mut stdout)?;
    }

    Ok(())
}
