use std::{
    collections::HashMap,
    io::{StdoutLock, Write},
};

use anyhow::Context;
use gossip::node::{Body, Message};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

struct BroadcastNode {
    id: usize,
    messages: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum BroadcastPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

impl BroadcastNode {
    pub fn step<'a>(
        &mut self,
        input: Message<BroadcastPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let reply = match input.body.payload {
            BroadcastPayload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: BroadcastPayload::InitOk {},
                    },
                };
                Some(reply)
            }
            BroadcastPayload::InitOk => None,

            BroadcastPayload::Broadcast { message } => {
                self.messages.push(message);

                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: BroadcastPayload::BroadcastOk,
                    },
                };

                Some(reply)
            }
            BroadcastPayload::BroadcastOk => todo!(),

            BroadcastPayload::Read => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: BroadcastPayload::ReadOk {
                            messages: self.messages.clone(),
                        },
                    },
                };

                Some(reply)
            }
            BroadcastPayload::ReadOk { .. } => None,

            BroadcastPayload::Topology { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: BroadcastPayload::TopologyOk,
                    },
                };

                Some(reply)
            }
            BroadcastPayload::TopologyOk => None,
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

    let inputs = Deserializer::from_reader(stdin).into_iter::<Message<BroadcastPayload>>();

    let mut node = BroadcastNode {
        id: 0,
        messages: vec![],
    };

    for input in inputs {
        let input = input.context("cannot deserialize node from maelstrom")?;

        node.step(input, &mut stdout)?;
    }

    Ok(())
}
