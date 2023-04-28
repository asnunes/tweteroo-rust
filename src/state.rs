use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct TweterooState {
    users: Mutex<Vec<User>>,
    tweets: Mutex<Vec<Tweet>>,
}

impl TweterooState {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(vec![]),
            tweets: Mutex::new(vec![]),
        }
    }

    pub fn add_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.push(user);
    }

    pub fn add_tweet(&self, tweet: Tweet) {
        let mut tweets = self.tweets.lock().unwrap();
        tweets.push(tweet);
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        let user = users.iter().find(|u| u.username == username);

        match user {
            Some(user) => Some(user.clone()),
            None => None,
        }
    }

    pub fn get_tweets(&self) -> Vec<TweetWithUser> {
        let tweets = self.tweets.lock().unwrap();
        let users = self.users.lock().unwrap();

        tweets
            .iter()
            .rev()
            .take(10)
            .rev()
            .map(|tweet| {
                let user = users.iter().find(|u| u.id == tweet.user_id).unwrap();
                TweetWithUser {
                    id: tweet.id.clone(),
                    user: user.clone(),
                    tweet: tweet.tweet.clone(),
                }
            })
            .collect()
    }

    pub fn get_tweets_by_username(&self, username: &str) -> Vec<TweetWithUser> {
        let tweets = self.tweets.lock().unwrap();
        let users = self.users.lock().unwrap();

        let user_opt = users.iter().find(|u| u.username == username);
        if user_opt.is_none() {
            return vec![];
        }

        let user = user_opt.unwrap();

        tweets
            .iter()
            .rev()
            .filter(|tweet| tweet.user_id == user.id)
            .rev()
            .take(10)
            .map(|tweet| TweetWithUser {
                id: tweet.id.clone(),
                user: user.clone(),
                tweet: tweet.tweet.clone(),
            })
            .collect()
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

pub struct TweetWithUser {
    pub id: String,
    pub user: User,
    pub tweet: String,
}
