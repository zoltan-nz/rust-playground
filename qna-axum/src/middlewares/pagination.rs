use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
        }
    }
}

impl Pagination {
    pub fn get_values(&self) -> (usize, usize) {
        (self.page.unwrap_or(1), self.per_page.unwrap_or(10))
    }
}
