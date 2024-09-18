use std::fmt::{Display, Formatter, Result};

use markdown::{
	mdast::{Node, Root, Text},
	to_html,
};

pub enum ArticleCategory {
	Rust,
	Linux,
	Unselected,
}

impl Display for ArticleCategory {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			ArticleCategory::Rust => write!(f, "Rust"),
			ArticleCategory::Linux => write!(f, "Linux"),
			ArticleCategory::Unselected => write!(f, ""),
		}
	}
}

pub struct Article {
	title: String,
	content: String,
	category: ArticleCategory,
}

impl Display for Article {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.content)
	}
}

impl From<Node> for Article {
	fn from(value: Node) -> Self {
		let default = Node::Text(Text {
			value: "".to_string(),
			position: None,
		});
		let default_vector = vec![Node::Root(Root {
			children: vec![default.clone()],
			position: None,
		})];
		let content = to_html(&value.to_string());
		let title = value
			.children()
			.unwrap_or(&default_vector)
			.first()
			.unwrap_or(&default);
		let title = title.to_string();
		Article {
			title,
			content,
			category: ArticleCategory::Unselected,
		}
	}
}

impl Article {
	pub fn from_idx(id: usize) -> Article {
		unimplemented!()
	}

	pub fn new() -> Article {
		Article {
			title: "".to_string(),
			content: "".to_string(),
			category: ArticleCategory::Unselected,
		}
	}

	pub fn save(&mut self, category: ArticleCategory) {
		unimplemented!()
	}
}
