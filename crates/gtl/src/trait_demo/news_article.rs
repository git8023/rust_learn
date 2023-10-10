use super::traits::Summarizable;

// 实现Trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

///
/// 实现Summarizable trait
///
impl Summarizable for NewsArticle {

    ///
    /// summary签名来自 Summarizable
    ///
    fn summary(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}