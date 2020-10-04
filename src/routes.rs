/// home route
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

/// time line infor
#[get("/archive")]
pub fn archive() -> &'static str {
    "archive"
}

/// different kinds article route
#[get("/category?<kind>")]
pub fn category(kind: Option<String>) -> String {
    match kind {
        None => {
            String::from("all categories")
        }
        Some(kind) => {
            format!("all article in the category: {}", kind)
        }
    }
}

/// different tag article route
#[get("/tag?<tag>")]
pub fn tag(tag: Option<String>) -> String {
    match tag {
        None => {
            String::from("all tags")
        }
        Some(tag) => {
            format!("all article in the tags: {}", tag)
        }
    }
}

#[get("/articles?<tag>&<category>")]
pub fn articles(tag: Option<String>, category: Option<String>) -> String {
    format!("tag={},category={}", tag.unwrap(), category.unwrap())
}