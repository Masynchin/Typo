mod models;

use models::{Article, ArticleItem};
use warp::Filter;

fn random_article() -> Article {
    Article {
        blocks: vec![
            ArticleItem::Header {
                level: 1,
                text: "Ligue des champions - Bayern Munich - Real Madrid - L'antisèche : Le Bayern, tout juste… et tout à craindre".to_owned(),
            },
            ArticleItem::Paragraph {
                text: "Dans une demi-finale aller qu'il a dominée, le Bayern Munich n'a pourtant pas su faire la différence face à un Real Madrid qui a exploité ses très grosses erreurs individuelles. Pourtant, les Allemands avaient complètement comblé l'écart qui semblait les séparer des Espagnols. Autant Munich peut se féliciter de son match, autant il peut craindre la suite…".to_owned(),
            },
            ArticleItem::Header {
                level: 2,
                text: "Le jeu : Le Real s'en sort bien".to_owned(),
            },
            ArticleItem::Paragraph { text: "Dix-sept minutes. C'est le temps qu'il a fallu au Real Madrid pour imposer une première séquence de possession. C'est dire la domination du Bayern Munich dans un début de match qui lui laissera une tonne de regrets. L'ouverture du score de Vincius, au cœur du temps fort allemand, a complètement renversé le match jusqu'à la pause. Et il a fallu une permutation des ailiers du Bayern et l'arrivée sur l'aile gauche de Musiala pour remettre au supplice les Madrilènes. Un changement tactique qui aurait dû permettre au Bayern de décrocher un succès mérité. Mais tout ne s'explique pas sur un tableau noir et quand un défenseur est coupable de deux erreurs aussi grossières que celles de Kim Min-jae, impossible de s'en sortir indemne. Surtout face au Real.".to_owned() },
            ArticleItem::Header { level: 2, text: "Les joueurs : Le naufrage de Kim, les éclairs de Musiala et Vinicius".to_owned() },
            ArticleItem::Paragraph { text: "Il est rare de voir, à ce niveau, un défenseur autant plomber les chances de qualification de son équipe. Et pourtant, Kim Min-jae a complètement annihilé les efforts de son équipe en commettant deux erreurs impardonnables. La première en laissant un boulevard dans son dos, la seconde en ceinturant grossièrement Rodrygo dans la surface. Et voilà comment le Bayern Munich a concédé un nul alors qu'il méritait plus.".to_owned() },
            ArticleItem::Paragraph { text: "Le génial Jamal Musiala avait pourtant tout fait, en seconde période, pour mettre son équipe dans les meilleures dispositions et en rendant fou un Lucas Vazquez complètement dépassé par la vivacité de l'Allemand. Côté Real, Vinicius reste l'homme fort des Madrilènes en Ligue des champions et son doublé couronne une partie pleine. Même si l'ouverture du score est d'abord un petit chef d'œuvre de Toni Kroos, auteur de la plus belle passe décisive du printemps.".to_owned() },
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
