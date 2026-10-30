#[derive(Clone, Copy, Debug)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

impl Pagination {
    pub fn new(limit: i64, offset: i64) -> Self {
        Self { limit, offset }
    }
}
