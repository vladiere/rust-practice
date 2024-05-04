#[derive(Debug)]
pub struct PostBody<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub user_id: i32,
    pub posted_at: &'a str,
}
