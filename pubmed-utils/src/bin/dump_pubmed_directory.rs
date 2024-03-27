extern crate pubmed;

use rsnltk::tokenize_sentence;
use tokio::{io, sync::mpsc};



#[tokio::main]
async fn main() -> io::Result<()> {
    let lang="en";
    let path = String::from("/Users/sdoronin/Downloads/baseline");
    let (tx, mut rx) = mpsc::channel(10);
    let producer_handle = tokio::spawn(pubmed::directory_articles(path, tx));
    while let Some(article) = rx.recv().await {
        // let title = article.title();
        let parts = article.abstact_parts();
        for part in &parts {
            let text = part.text();
            let sentences=tokenize_sentence(text.as_str(),lang);
            for sentence in sentences {
                println!("{}",sentence);
            }
            
        }
        // println!("{}",title.unwrap_or(String::new()));
    }
    producer_handle.await.expect("Producer task panicked")
}