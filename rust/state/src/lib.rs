use::std::rc::Rc;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approve_calls: Rc<i32>
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approve_calls: Rc::new(0)
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // self.content.push_str(text);

        if let Some(s) = self.state.take() {
            self.content.push_str(&s.add_text(String::from(text)))
        }
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        let a = *self.approve_calls;
        if a == 2 {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
            self.approve_calls = Rc::new(0)
        } else {
            self.approve_calls = Rc::new(1+*self.approve_calls)
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
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, text: String) -> String {
        return text
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
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
    fn text() {
        let mut post = Post::new();
        println!("{:?}", post.content);

        post.add_text("text");
        println!("{:?}", post.content);

        post.approve();
        post.add_text("aboba");
        println!("{:?}", post.content);
        println!("{:?}", *post.approve_calls);

        post.approve();
        println!("{:?}", *post.approve_calls);

        post.approve();
        println!("{:?}", *post.approve_calls);


    }
}