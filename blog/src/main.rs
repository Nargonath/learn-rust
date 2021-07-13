use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let pending = post.request_review();

    let mut post = pending.reject();

    post.add_text("\nI added that!");

    let post = post.request_review();

    let post = post.approve();
    let post = post.approve();
    assert_eq!(
        "I ate a salad for lunch today\nI added that!",
        post.content()
    );
}
