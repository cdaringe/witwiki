use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct ErrorMsg {
    message: String,
    details: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    values: T,
    total: usize,
    errors: Option<Vec<ErrorMsg>>,
}

impl<T> ApiResponse<Vec<T>>
where
    T: Serialize,
{
    pub fn new(values: Vec<T>, total: usize) -> Self {
        Self {
            values,
            total,
            errors: None,
        }
    }
}

impl ApiResponse<Vec<()>> {
    pub fn empty() -> Self {
        let values: Vec<()> = vec![];
        Self {
            values,
            total: 0,
            errors: None,
        }
    }
}
