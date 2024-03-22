
#[cfg(test)]
mod tests {
    use std::env;

    use pubmed::{articles, directory_articles, PubmedArticle};
    use tokio::sync::mpsc;

    use super::*; // import the module being tested

    #[tokio::test]
    async fn test_articles() {
        let path = String::from("/Users/sdoronin/workspace/test-data/articleset.xml.gz");
        let (tx, mut rx) = mpsc::channel(10);
        let producer_handle = tokio::spawn(articles(path, tx));
        let mut articles: Vec<PubmedArticle> = Vec::new();
        while let Some(article) = rx.recv().await {
            articles.push(article);
        }
        let _ = producer_handle.await;
        let a = articles.get(0).unwrap();
        assert!(articles.len() == 4);
        assert!(a.pmid().unwrap() == 28895354);
        assert!(
            a.title().unwrap()
                == String::from(
                    "[Fermentations of xylose and arabinose by Kluyveromyces marxianus]."
                )
        )
    }

    /// Retrieves the current directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    ///
    /// // Retrieve the current directory
    /// if let Ok(current_dir) = env::current_dir() {
    ///     println!("Current directory: {}", current_dir.display());
    /// } else {
    ///     eprintln!("Failed to retrieve current directory");
    /// }
    /// ```
    #[tokio::test]
    async fn test_directory_articles() {
        let path = String::from("../");
        let (tx, mut rx) = mpsc::channel(10);
        let producer_handle = tokio::spawn(directory_articles(path, tx));
        let mut articles: Vec<PubmedArticle> = Vec::new();
        while let Some(article) = rx.recv().await {
            articles.push(article);
        }
        let _ = producer_handle.await;
        // let  a = articles.get(0).unwrap();
        assert!(articles.len() == 4);
        // assert!(a.pmid().unwrap() == 28895354);
        // assert!(a.title().unwrap() == String::from("[Fermentations of xylose and arabinose by Kluyveromyces marxianus]."))
    }
}
