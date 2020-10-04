use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleCard {
    title: String,
    time: String,
    category: String,
    tags: Vec<String>,
    description: String,
}

impl ArticleCard {
    pub fn from(file: &str) -> Vec<Self> {
        let mut article_cards = String::new();
        let mut file = File::open(file).unwrap();
        file.read_to_string(&mut article_cards).unwrap();
        serde_json::from_str(&article_cards).unwrap()
    }

    pub fn is_in_category(&self, category: &str) -> bool {
        if self.category == category {
            true
        } else { false }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        let mut result = false;

        for stag in &self.tags {
            if (stag as &str) == tag {
                result = true;
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::article::ArticleCard;

    #[test]
    fn base() {
        let article_card = ArticleCard {
            title: String::from("测试文章"),
            time: String::from("2020-10-04"),
            category: String::from("test"),
            tags: vec![String::from("test"), String::from("rust"), String::from("json")],
            description: String::from("这是用来测试的文章。"),
        };

        let serialized = serde_json::to_string(&article_card).unwrap();
        let assert_serialized = r#"{"title":"测试文章","time":"2020-10-04","category":"test","tags":["test","rust","json"],"description":"这是用来测试的文章。"}"#;
        assert_eq!(serialized, assert_serialized.to_string());

        let deserialized: ArticleCard = serde_json::from_str(&serialized).unwrap();
        let deserialized = format!("{:?}", deserialized);
        let assert_deserialized = r#"ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是用来测试的文章。" }"#;
        assert_eq!(deserialized, assert_deserialized.to_string());
    }

    #[test]
    fn file() {
        let path = "src/resource/json/articleCards.json";
        let cards = ArticleCard::from(path);
        let assert_cards = r#"[ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], description: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }]"#;
        assert_eq!(format!("{:?}", cards), assert_cards);
    }

    #[test]
    fn assert() {
        let path = "src/resource/json/articleCards.json";
        let cards = ArticleCard::from(path);

        let tag = "rust";
        let category = "test";

        assert_eq!(cards[0].has_tag(tag), true);
        assert_eq!(cards[0].is_in_category(category), true);

        let tag = "kotlin";
        let category = "lalilulelo";

        assert_eq!(cards[0].has_tag(tag), false);
        assert_eq!(cards[0].is_in_category(category), false)
    }
}