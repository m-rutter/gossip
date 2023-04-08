use std::io::{StdoutLock, Write};

use anyhow::Context;
use gossip::node::{Body, Node, Payload};
use serde_json::Deserializer;

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step<'a>(&mut self, input: Node, output: &mut StdoutLock) -> anyhow::Result<()> {
        let reply = match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Node {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo: echo.clone() },
                    },
                };

                Some(reply)
            }
            Payload::EchoOk { .. } => None,
            Payload::Init { .. } => {
                let reply = Node {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk {},
                    },
                };
                Some(reply)
            }
            Payload::InitOk => None,
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

    let inputs = Deserializer::from_reader(stdin).into_iter::<Node>();

    let mut node = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("cannot deserialize node from maelstrom")?;

        node.step(input, &mut stdout)?;
    }

    Ok(())
}
