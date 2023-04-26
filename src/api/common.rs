use serde::Deserialize;

#[derive(Deserialize)]
pub struct Page {
    pub page: u64,
    pub size: u64,
}

impl Default for Page {
  fn default() -> Self {
    Page {
      page: 0,
      size: 20
    }
  }
}