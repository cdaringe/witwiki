use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    values: T,
    total: usize,
}

impl<T> ApiResponse<Vec<T>>
where
    T: Serialize,
{
    pub fn new(values: Vec<T>, total: usize) -> Self {
        Self { values, total }
    }
}

impl ApiResponse<Vec<()>> {
    pub fn empty() -> Self {
        let values: Vec<()> = vec![];
        Self { values, total: 0 }
    }
}
