use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct TweterooState {
    users: Vec<User>,
    tweets: Vec<Tweet>,
}

impl TweterooState {
    pub fn new() -> Self {
        Self {
            users: vec![],
            tweets: vec![],
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_tweet(&mut self, tweet: Tweet) {
        self.tweets.push(tweet);
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    pub fn get_tweet(&self, id: &str) -> Option<&Tweet> {
        self.tweets.iter().find(|t| t.id == id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: String,
}

impl User {
    pub fn new(username: &str, avatar: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            avatar: avatar.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub user_id: String,
    pub tweet: String,
}

impl Tweet {
    pub fn new(user_id: &str, tweet: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            tweet: tweet.to_string(),
        }
    }
}
