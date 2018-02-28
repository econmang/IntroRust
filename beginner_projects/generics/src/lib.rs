//Traits are analogous to interfaces in other languages
//

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
    //the above example implements a default implementation

    //in order to be Summarizable
    //must implement a summary method
}

//if you had created a type "Article" and wanted to implement it with default
//implmentation of summary provided:
//
//imple Summarizable for Article {} will do so

pub fn test() {
    println!("Test");
}
