use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
    section![
        C!["hero is-link is-fullheight-with-navbar"],
        div![
            C!["hero-body"],
            figure![
                p![C!["title"], "Welcome to Crypto World"],
                small!["Jarek Lin"]
            ]
        ]
    ]
}
