mod models;

use models::{Article, ArticleItem};
use warp::Filter;

fn random_article() -> Article {
    Article {
        blocks: vec![
            ArticleItem::Header {
                level: 1,
                text: "Header".to_owned(),
            },
            ArticleItem::Paragraph {
                text: "Paragraph".to_owned(),
            },
        ],
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors().allow_any_origin();
    let random = warp::path!("api" / "random")
        .map(|| warp::reply::json(&random_article()))
        .with(cors);

    warp::serve(random).run(([127, 0, 0, 1], 3000)).await
}
