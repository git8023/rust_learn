/// 声明子模块
pub mod news_article;
pub mod traits;
pub mod tweet;
pub mod tweet_new;

use std::fmt::Display;

use traits::Summarizable;
use tweet_new::MyDisplay;

pub fn test() {
    let tweet = tweet::Tweet {
        username: "horse_ebook".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    println!(
        "Summarizable:: 1 new tweet: {}",
        Summarizable::summary(&tweet)
    );
    println!("MyDisplay:: 1 new tweet: {}", MyDisplay::summary(&tweet));

    notify(tweet);

    println!("3.str_len(): {}", "3f64".str_len());
    
}

fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

trait StringLength {
    fn str_len(&self) -> usize;
}

impl<T: Display> StringLength for T {
    fn str_len(&self) -> usize {
        let s = self.to_string();
        s.len()
    }
}
