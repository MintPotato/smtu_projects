pub struct Post {
    content: String
}

pub struct Draft {
    content: String
}

pub struct PendingReview {
    content: String
}

pub struct PendingReview1 {
    content: String
}

impl Post {
    pub fn new() -> Draft {
        Draft { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
} 

impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview { content: self.content }
    }
}

impl PendingReview {
    pub fn approve(self) -> PendingReview1{
        PendingReview1 { content: self.content }
    }

    pub fn reject(self) -> Draft {
        Draft { content: self.content }
    }
}

impl PendingReview1 {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }

    pub fn reject(self) -> Draft {
        Draft { content: self.content }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut post = Post::new();

        post.add_text("text");

        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();

        println!("{:?}", post.content())
    }

    #[test]
    fn test1() {
        let mut post = Post::new();

        let post = post.request_review();
        let post = post.approve();

        let post = post.reject();

        let post = post.request_review();
        let post = post.reject(); 
    }
}