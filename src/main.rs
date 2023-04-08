use std::io::StdoutLock;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Body {
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk,
}

struct EchoNode;

impl EchoNode {
    pub fn handle(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {}
            Payload::EchoOk => todo!(),
        }

        let reply = Message {
            src: input.dest,
            dest: input.src,
            body: Body {
                id: None,
                in_reply_to: None,
                payload: Payload::Echo {
                    echo: "echo".to_string(),
                },
            },
        };
        todo!()
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    for input in inputs {
        let input = input.context("cannot deserialize")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_msg() {
        let msg = Message {
            src: "foo".to_string(),
            dest: "bar".to_string(),
            body: Body {
                id: Some(10),
                in_reply_to: Some(55),
                payload: Payload::Echo {
                    echo: "foobar".to_string(),
                },
            },
        };

        insta::assert_json_snapshot!(&msg);
    }

    #[test]
    fn deserialize_msg() {
        let json = r#"
        {
            "src": "foo",
            "dest": "bar",
            "body": {
                "id": 10,
                "in_reply_to": 55,
                "type": "echo",
                "echo": "foobar"
            }
        }
            "#;

        let msg: Message = serde_json::from_str(json).unwrap();

        insta::assert_snapshot!(format!("{:?}", msg));
    }
}
