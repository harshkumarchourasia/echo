use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    msg_id: Option<i32>,
    in_reply_to: Option<i32>,
    echo: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the "type" field
        write!(f, "{{ \"type\": \"{}\"", self.ty)?;

        if let Some(msg_id) = self.msg_id {
            write!(f, ", \"msg_id\": {}", msg_id)?;
        }

        if let Some(in_reply_to) = self.in_reply_to {
            write!(f, ", \"in_reply_to\": {}", in_reply_to)?;
        }

        if let Some(ref echo) = self.echo {
            write!(f, ", \"echo\": \"{}\"", echo)?;
        }

        write!(f, "}}")
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"src\": \"{}\", \"dest\": \"{}\" , \"body\": {:?}}}",
            self.src, self.dest, self.body
        )
    }
}

fn echo(mut inp: Message) -> Message {
    inp.body.ty = "echo_ok".to_string();
    inp.body.in_reply_to = inp.body.msg_id;
    inp
}

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let message: Message = serde_json::from_str(&input).expect("Failed to parse JSON");
        let response = echo(message);

        println!("{:?}", response);
    }
}
