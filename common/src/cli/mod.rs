use std::io::{self, Write};
use tokio::task;

pub async fn get_user_input(prompt: &str) -> io::Result<String> {
    let prompt = prompt.to_owned();
    task::spawn_blocking(move || {
        let mut buffer = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");
        Ok(buffer.trim().to_string())
    })
    .await
    .unwrap()
}
