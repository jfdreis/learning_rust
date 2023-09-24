pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
//This was the first version of add_text
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
        }
//This is the version that only allows to add_text when the Post is in Draft state

// Allow users to add text content only when a post is in the Draft state.
//Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.


    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state =Some(s.request_review())
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
    fn request_review(self:Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self:Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self : Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self : Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview{}

impl State for PendingReview {
    fn request_review(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self : Box<Self>) -> Box<dyn State> {
        Box::new(FirstApproval {})
    }

    fn reject(self : Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct FirstApproval {}

impl State for FirstApproval {
    fn request_review(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self : Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self : Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self:Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self:Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        &post.content
    }

    fn reject(self : Box<Self>) -> Box<dyn State> {
        self
    }

}