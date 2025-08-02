use prompts::{Prompt, text::TextPrompt};
mod ai_client;

#[tokio::main]
async fn main() {
    loop {
        let mut prompt_text = TextPrompt::new("PLEASE ENTER YOUR QUERY.. ");
        match prompt_text.run().await {
            Ok(Some(input)) => {
                if input == "exit" {
                    break;
                }
                println!("{input}");
                match ai_client::ask_gpt(&input).await {
                    Ok(response) => println!("AI Response : {response}"),
                    Err(e) => println!("AI Response Error : {e}"),
                }
            }
            Err(error) => {
                println!("Error : {error}")
            }
            _ => {}
        }
    }
}
