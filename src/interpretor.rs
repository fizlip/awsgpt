use openai_rust;
use openai_rust::futures_util::StreamExt;
use std::io::Write;

pub struct Interpretor {
    client: openai_rust::Client,
    role: String,
    msgs: Vec<openai_rust::chat::Message>,
}

impl Interpretor {
    pub fn new(role: String) -> Interpretor {
        let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
        let msgs: Vec<openai_rust::chat::Message> = vec![];

        Interpretor {client: client, role: role, msgs: msgs}
    }

    pub async fn post(&mut self, message: String) {
        // Prepare message
        self.msgs.push(
            openai_rust::chat::Message {
                role: self.role.to_owned(),
                content: message 
        });

        // Create new message
        let args = openai_rust::chat::ChatArguments::new(
            "gpt-3.5-turbo",
            self.msgs.to_vec(),
        );

        let mut res = self.client.create_chat_stream(args).await.unwrap();
        let mut response = String::from("");

        print!("\x1b[32m[AWS-GPT]\x1b[0m ");

        while let Some(events) = res.next().await {
            for event in events.unwrap() {
                print!("{}", event);
                std::io::stdout().flush().unwrap();
                response.push_str(&event.to_string());

            }
        }

        self.msgs.push(
            openai_rust::chat::Message {
                role: "user".to_owned(),
                content: response
            }
        );
    }
}
