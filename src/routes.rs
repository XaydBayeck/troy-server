use crate::article::{ArticleCard, Search, ArticleList};
use rocket::response::content::Json;
use std::path::{PathBuf, Path};
use rocket::response::NamedFile;

/// home route
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

type ArticleCards = Json<ArticleList>;

/// different kinds article route
#[get("/category/<category>")]
pub fn in_category(category: String) -> ArticleCards {
    articles(None, Some(category))
}

/// different tag article route
#[get("/tag/<tag>")]
pub fn has_tag(tag: String) -> ArticleCards {
    articles(Some(tag), None)
}

#[get("/articles?<category>&<tag>")]
pub fn articles(tag: Option<String>, category: Option<String>) -> ArticleCards {
    //TODO: 将json文件读取改成数据库读取
    let articles = ArticleCard::from("src/resource/json/articleCards.json");

    let article_list = match (tag, category) {
        (None, None) => vec![],
        (None, Some(category)) => {
            let (articles, _) = articles.in_category(&category);

            articles
        }
        (Some(tag), None) => {
            let (articles, _) = articles.has_tag(&tag);
            articles
        }
        (Some(tag), Some(category)) => {
            let (articles, _) = articles.search_all(&category, &tag);
            articles
        }
    };

    Json(ArticleList {
        0: article_list
    })
}

/// static file route
#[get("/<file..>", rank = 2)]
pub fn files(file:PathBuf)->Option<NamedFile>{
    NamedFile::open(Path::new("static/").join(file)).ok()
}

