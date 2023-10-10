use super::traits::Summarizable;

#[derive(Debug)]
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
