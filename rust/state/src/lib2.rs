pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // self.content.push_str(text);

        if let Some(s) = self.state.take() {
            self.content.push_str(&s.add_text(String::from(text)));
            // self.state = Some(s.return_self())
            self.state = Some(s)
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(&self, text: String) -> String;
    fn content<'a>(&self, post: &'a Post) -> &'a str {""}
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_calls: 0})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // println!("--draft approve--");
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, text: String) -> String {
        return text
    }
}

struct PendingReview {
    approve_calls: i32
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // println!("--{:?}--", self.approve_calls);
        if self.approve_calls == 1 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview{ approve_calls: self.approve_calls + 1})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn add_text(&self, text: String) -> String {
        return String::from("")
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // println!("--pub approve--");
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, text: String) -> String {
        return String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post1() {
        let mut post = Post::new();
        println!("{:?}", post.content);

        post.add_text("text");
        println!("{:?}\n-----------------", post.content);

        post.request_review();
        post.add_text("aboba");
        println!("{:?}", post.content);

        post.approve();
        println!("{:?}", post.content());

        post.approve();
        println!("{:?}", post.content());
    }

    #[test]
    fn post2() {
        let mut post = Post::new();

        post.add_text("aboba");
        post.request_review();

        post.approve();
        println!("{:?}", post.content());

        post.reject();

        post.add_text("text");
        println!("{:?}", post.content);

        post.request_review();

        post.approve();
        post.approve();
        println!("{:?}", post.content());
    }
}