use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reload {
	pub items: Option<Vec<ReloadItem>>,
}

#[derive(Deserialize)]
pub struct ReloadItem {
	pub hook: String,
	pub args: Vec<String>,
}
