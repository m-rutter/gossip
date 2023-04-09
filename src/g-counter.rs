use std::io::{StdoutLock, Write};

use anyhow::Context;
use gossip::message::{Body, Message};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

struct CounterNode {
    id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum CounterPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

impl CounterNode {
    pub fn step<'a>(
        &mut self,
        input: Message<CounterPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let reply = match input.body.payload {
            CounterPayload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: CounterPayload::InitOk {},
                    },
                };
                Some(reply)
            }
            CounterPayload::InitOk => None,
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

    let inputs = Deserializer::from_reader(stdin).into_iter::<Message<CounterPayload>>();

    let mut node = CounterNode { id: 0 };

    for input in inputs {
        let input = input.context("cannot deserialize node from maelstrom")?;

        node.step(input, &mut stdout)?;
    }

    Ok(())
}
