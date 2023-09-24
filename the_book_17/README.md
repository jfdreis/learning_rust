# Object-Oriented Programming Features of Rust

In [chapter 17 of The Rust Programming Language](https://doc.rust-lang.org/book/ch17-00-oop.html) they what to solve this project:

"First, we’re going to implement the state pattern in a more traditional object-oriented way, then we’ll use an approach that’s a bit more natural in Rust. Let’s dig in to incrementally implementing a blog post workflow using the state pattern.

The final functionality will look like this:

A blog post starts as an empty draft.
When the draft is done, a review of the post is requested.
When the post is approved, it gets published.
Only published blog posts return content to print, so unapproved posts can’t accidentally be published."

They do it in two different ways that can be seen in the folders blog_object_oriented and blog_states_as_types.

## My purpose

I want to build on those two solutions and add the following functionalities:

1. Add a reject method that changes the post’s state from PendingReview back to Draft.
2. Require two calls to approve before the state can be changed to Published.
3. Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.
