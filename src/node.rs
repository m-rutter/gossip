use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Body {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_msg() {
        let msg = Node {
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
                "msg_id": 10,
                "in_reply_to": 55,
                "type": "echo_ok",
                "echo": "foobar"
            }
        }
            "#;

        let msg: Node = serde_json::from_str(json).unwrap();

        insta::assert_snapshot!(format!("{:?}", msg));
    }
}
