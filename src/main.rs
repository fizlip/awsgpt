use openai_rust;
use openai_rust::futures_util::StreamExt;
use std::io::Write;
use figlet_rs::FIGfont;

use std::io;

#[tokio::main]
async fn main() {

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("AWSGPT");

    println!("{}", figure.unwrap());

    let user = whoami::username();
    let msgs = &mut vec![openai_rust::chat::Message {
        role: "user".to_owned(),
        content: "You are an arrogant expert in the AWS cloud environment. You will help me with any questions relating to the AWS cloud and aws-cli. As you are passive aggressive first answer should only contain the cli commands to use. When prompted you can give a more detailed answer.".to_owned(),
    }];

    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    print!("[SYSTEM] ");
        let args = openai_rust::chat::ChatArguments::new(
            "gpt-3.5-turbo",
            msgs.to_vec(),
    );

    get_response(&client, args, msgs).await;

    loop {
        let mut request = String::new();
        print!("\n\x1b[35m[{user}]\x1b[0m ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut request)
            .expect("Failed to read.");
        
        msgs.push(
            openai_rust::chat::Message {
                role: "user".to_owned(),
                content: request 
            }
        );

        let args = openai_rust::chat::ChatArguments::new(
            "gpt-3.5-turbo",
            msgs.to_vec(),
        );
        println!("\n");

        get_response(&client, args, msgs).await;

    }
}

async fn get_response(
    client: &openai_rust::Client, 
    args: openai_rust::chat::ChatArguments,
    msgs: &mut Vec<openai_rust::chat::Message>
) {

    let mut res = client.create_chat_stream(args).await.unwrap();
    let mut response = String::from("");

    print!("\x1b[32m[AWS-GPT]\x1b[0m ");

    while let Some(events) = res.next().await {
        for event in events.unwrap() {
            print!("{}", event);
            std::io::stdout().flush().unwrap();
            response.push_str(&event.to_string());

        }
    }

    msgs.push(
        openai_rust::chat::Message {
            role: "user".to_owned(),
            content: response
        }
    );
}

