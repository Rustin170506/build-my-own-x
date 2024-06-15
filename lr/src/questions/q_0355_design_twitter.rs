use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct Post {
    tweet_id: i32,
    date: i32,
}

impl std::cmp::PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Post {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

struct User {
    user_id: i32,
    posts: BinaryHeap<Post>,
    following: HashSet<i32>,
}

struct Twitter {
    users: HashMap<i32, User>,
    current_date: i32,
}

impl Twitter {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            current_date: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let user = self.users.entry(user_id).or_insert(User {
            user_id,
            posts: BinaryHeap::new(),
            following: HashSet::new(),
        });
        user.posts.push(Post {
            tweet_id,
            date: self.current_date,
        });
        self.current_date += 1;
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut posts = BinaryHeap::new();
        if !self.users.contains_key(&user_id) {
            return Vec::new();
        }

        let user = self.users.get(&user_id).unwrap();
        for post in &user.posts {
            posts.push(post);
        }

        for followee_id in user.following.iter() {
            if let Some(user) = self.users.get(followee_id) {
                for post in &user.posts {
                    posts.push(post);
                }
            }
        }

        let mut result = Vec::new();
        for _ in 0..10 {
            if let Some(post) = posts.pop() {
                result.push(post.tweet_id);
            } else {
                break;
            }
        }

        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        match self.users.get_mut(&follower_id) {
            Some(user) => {
                user.following.insert(followee_id);
            }
            None => {
                let mut user = User {
                    user_id: follower_id,
                    posts: BinaryHeap::new(),
                    following: HashSet::new(),
                };
                user.following.insert(followee_id);
                self.users.insert(follower_id, user);
            }
        };
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        match self.users.get_mut(&follower_id) {
            Some(user) => {
                user.following.remove(&followee_id);
            }
            None => {
                let user = User {
                    user_id: follower_id,
                    posts: BinaryHeap::new(),
                    following: HashSet::new(),
                };
                self.users.insert(follower_id, user);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_355() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
    }
}
