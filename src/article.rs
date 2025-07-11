use std::fmt::Display;

use rocket_db_pools::{sqlx, Connection};

pub enum ArticleCategory {
	Rust,
	Linux,
	FreeBSD,
	Other,
}

impl Display for ArticleCategory {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ArticleCategory::Rust => write!(f, "Rust"),
			ArticleCategory::Linux => write!(f, "Linux"),
			ArticleCategory::FreeBSD => write!(f, "FreeBSD"),
			ArticleCategory::Other => write!(f, "Other"),
		}
	}
}

impl From<String> for ArticleCategory {
	fn from(value: String) -> Self {
		match value.as_str() {
			"Rust" => ArticleCategory::Rust,
			"Linux" => ArticleCategory::Linux,
			"FreeBSD" => ArticleCategory::FreeBSD,
			_ => ArticleCategory::Other,
		}
	}
}

pub struct Article {
	pub title: String,
	pub content: String,
	pub category: ArticleCategory,
}

impl Article {
	pub async fn read(mut db: Connection<crate::Blog>, id: i64) -> Option<Article> {
		let row: Result<Option<(String, String, String)>, sqlx::Error> =
			sqlx::query_as("select title, content, category from articles where id = ?")
				.bind(id)
				.fetch_optional(&mut **db)
				.await;
		if let Ok(row) = row {
			if let Some((title, content, category)) = row {
				return Some(Article {
					title,
					content,
					category: ArticleCategory::from(category),
				});
			}
		}
		None
	}
}
