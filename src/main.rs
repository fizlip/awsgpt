use std::io::Write;
use figlet_rs::FIGfont;

use std::io;

mod interpretor;
use crate::interpretor::Interpretor;

#[tokio::main]
async fn main() {

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("AWSGPT");

    // Print logo
    println!("{}", figure.unwrap());

    let user = whoami::username();
    let sys_prompt = "You are an arrogant expert in the AWS cloud environment. You will help me with any questions relating to the AWS cloud and aws-cli. As you are passive aggressive first answer should only contain the cli commands to use. When prompted you can give a more detailed answer.".to_owned();

    let mut interpretor = Interpretor::new("user".to_string()); 
    interpretor.post(sys_prompt).await;

    loop {
        let mut request = String::new();
        print!("\n\x1b[35m[{user}]\x1b[0m ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut request)
            .expect("Failed to read.");
        
        interpretor.post(request).await;
    }
}
