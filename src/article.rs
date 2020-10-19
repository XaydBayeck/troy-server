use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::{response, Request, Response};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use chrono::{NaiveDate, ParseResult};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct ArticleCard {
    pub(crate) title: String,
    pub time: String,
    pub(crate) category: String,
    pub(crate) tags: Vec<String>,
    pub(crate) part: String,
}

impl ArticleCard {
    pub fn from(file: PathBuf) -> Vec<Self> {
        let mut article_cards = String::new();
        let mut file = File::open(file).unwrap();
        file.read_to_string(&mut article_cards).unwrap();
        serde_json::from_str(&article_cards).unwrap()
    }

    pub fn is_in_category(&self, category: &str) -> bool {
        if self.category == category {
            true
        } else {
            false
        }
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

    #[warn(dead_code)]
    pub fn to_sql(&self) -> String {
        let tags = self
            .tags
            .iter()
            .fold(String::new(), |tags, tag| tags + tag + ",");
        format!(
            "insert into article_card \
        values (\"{}\",\"{}\",\"{}\",\"{:}\",\"{}\");",
            &self.title, &self.time, &self.category, tags, &self.part
        )
    }

    pub fn get_time(&self) -> ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(&*self.time, "%Y-%m-%d")
    }

    pub fn set_time(&mut self, date: NaiveDate) {
        self.time = date.format("%Y-%m-%d").to_string();
    }
}

pub(crate) trait Search: Sized {
    fn in_category(&self, category: &str) -> (Self, bool);
    fn has_tag(&self, tag: &str) -> (Self, bool);
    fn search_all(&self, category: &str, tag: &str) -> (Self, bool);
}

impl Search for Vec<ArticleCard> {
    fn in_category(&self, category: &str) -> (Self, bool) {
        let articles = self
            .clone()
            .into_iter()
            .filter(|article| article.is_in_category(category))
            .collect::<Vec<_>>();

        if articles.len() > 0 {
            (articles, true)
        } else {
            (articles, false)
        }
    }

    fn has_tag(&self, tag: &str) -> (Self, bool) {
        let articles = self
            .clone()
            .into_iter()
            .filter(|article| article.has_tag(tag))
            .collect::<Vec<_>>();

        if articles.len() > 0 {
            (articles, true)
        } else {
            (articles, false)
        }
    }

    fn search_all(&self, category: &str, tag: &str) -> (Self, bool) {
        let (articles, in_category) = self.in_category(category);
        if in_category {
            let (articles, has_tag) = articles.has_tag(tag);
            if has_tag {
                (articles, true)
            } else {
                (articles, false)
            }
        } else {
            (articles, false)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleList(pub(crate) Vec<ArticleCard>);

impl<'r> Responder<'r> for ArticleList {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .header(ContentType::JSON)
            .ok()
    }
}

#[cfg(test)]
mod test {
    use crate::article::ArticleCard;
    use crate::article::Search;
    use std::path::PathBuf;

    #[test]
    fn base() {
        let article_card = ArticleCard {
            title: String::from("测试文章"),
            time: String::from("2020-10-04"),
            category: String::from("test"),
            tags: vec![
                String::from("test"),
                String::from("rust"),
                String::from("json"),
            ],
            part: String::from("这是用来测试的文章。"),
        };

        let serialized = serde_json::to_string(&article_card).unwrap();
        let assert_serialized = r#"{"title":"测试文章","time":"2020-10-04","category":"test","tags":["test","rust","json"],"part":"这是用来测试的文章。"}"#;
        assert_eq!(serialized, assert_serialized.to_string());

        let deserialized: ArticleCard = serde_json::from_str(&serialized).unwrap();
        let deserialized = format!("{:?}", deserialized);
        let assert_deserialized = r#"ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是用来测试的文章。" }"#;
        assert_eq!(deserialized, assert_deserialized.to_string());
    }

    #[test]
    fn file() {
        let path = PathBuf::from("static/json/articleCards.json");
        let cards = ArticleCard::from(path);
        let assert_cards = r#"[ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }, ArticleCard { title: "测试文章", time: "2020-10-04", category: "test", tags: ["test", "rust", "json"], part: "这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，这是测试文章，" }]"#;
        assert_eq!(format!("{:?}", cards), assert_cards);
    }

    #[test]
    fn assert() {
        let path = PathBuf::from("static/json/articleCards.json");
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

    #[test]
    fn vec_category_assert() {
        let path = PathBuf::from("static/json/articleCards.json");
        let cards = ArticleCard::from(path);

        let category = "test";
        let result = cards.in_category(category);
        println!("{:?}", result.0);
        assert_eq!(result.1, true);

        let category = "lalilulelo";
        let result = cards.in_category(category);
        println!("{:?}", result.0);
        assert_eq!(result.1, false);
    }

    #[test]
    fn vec_tag_assert() {
        let path = PathBuf::from("static/json/articleCards.json");
        let cards = ArticleCard::from(path);

        let tag = "rust";
        let result = cards.has_tag(tag);
        println!("{:?}", result.0);
        assert_eq!(result.1, true);

        let tag = "kotlin";
        let result = cards.has_tag(tag);
        println!("{:?}", result.0);
        assert_eq!(result.1, false);
    }

    #[test]
    fn vec_both_assert() {
        let path = PathBuf::from("static/json/articleCards.json");
        let cards = ArticleCard::from(path);

        let category = "test";
        let tag = "rust";
        let result = cards.search_all(category, tag);
        println!("{:?}", result.0);
        assert_eq!(result.1, true);

        let category = "lalilulelo";
        let tag = "kotlin";
        let result = cards.search_all(category, tag);
        println!("{:?}", result.0);
        assert_eq!(result.1, false);
    }
}
