use std::fmt::Display;

pub enum ArticleCategory {
	Rust,
	Linux,
	FreeBSD,
}

impl Display for ArticleCategory {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ArticleCategory::Rust => write!(f, "Rust"),
			ArticleCategory::Linux => write!(f, "Linux"),
			ArticleCategory::FreeBSD => write!(f, "FreeBSD"),
		}
	}
}

pub struct Article {
	id: u32,
	title: String,
	content: String,
	category: ArticleCategory,
}
