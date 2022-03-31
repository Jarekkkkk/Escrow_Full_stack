use seed::{future::Remote, prelude::*, *};

use crate::Context;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        actions: Action::Burn,

        mint: RemoteData::Notasked,
        transfer: RemoteData::Notasked,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    //controlled form for each actions
    actions: Action,

    //individual respond corresponding to 'actions'
    mint: RemoteData<Mint>,
    transfer: RemoteData<Transfer>,
}

enum RemoteData<T> {
    Notasked,
    Loading,
    Loaded(T),
}

//retunr value

enum Action {
    Mint(MintForm),
    Transfer,
    Freeze,
    SetOwner,
    SetCloser,
    Burn,
    Foo,
}

//derive the fitler from 'given action name'

// ------ ------ ------
//    Model_Action_Form
// ------ ------ ------

// ------ Mint ------
struct MintForm {
    destination_account: String,
    amount: String,

    form_error: MintFormErrors,
}
struct MintFormErrors {
    destination_account: Option<String>,
    amount: Option<String>,
}

// ------ Transfer ------
struct TransferForm {
    source_account: String,
    destination_account: String,
    amount: String,

    form_error: TransferFormErrors,
}

struct TransferFormErrors {
    source_account: Option<String>,
    destination_account: Option<String>,
    amount: Option<String>,
}

// ------ ------ ------
//    Model_Response
// ------ ------ ------
struct Mint {
    deposited_address: String,
    amount: String,
    decimals: String,

    token_link: String,
}
struct Transfer {
    foo: String,
    bar: String,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    MintToken,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}
// ------ ------
//     view
// ------ ------

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    section![
        C!["section hero is-light is-fullheight-with-navbar"],
        attrs![At::Style => "padding-bottom: 10rem"],
        match &model.mint {
            RemoteData::Loading => {
                view_loading_message("Loading...")
            }
            RemoteData::Notasked => {
                view_loading_message("Mint To Account")
            }
            RemoteData::Loaded(mint) => {
                view_level_nav(mint)
            }
        },
        h2![
            C!["title is-2"],
            attrs! {At::Style => "text-align:center"},
            "Create Token Account"
        ],
        if let RemoteData::Loaded(mint) = &model.mint {
            div![
                C!["container is-fullhd"],
                attrs! {At::Style => "text-align:center"},
                div![
                    span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
                    span![C!["message"], &mint.deposited_address]
                ]
            ]
        } else {
            empty()
        },
        form![
            ev(Ev::Submit, move |event| {
                event.prevent_default();
            }),
            view_fee_payer_seed_input(model),
            view_mint_input(model),
            view_owner_input(model),
            view_button(ctx),
        ]
    ]
}
pub fn view_loading_message(msg: &str) -> Node<Msg> {
    div![div![C!["message"], msg]]
}
fn view_level_nav(mint: &Mint) -> Node<Msg> {
    nav![
        C!["level"],
        div![
            C!["level-item has-text-centered"],
            div![p![C!["haeding"], "Amount"], p![C!["title"], &mint.amount]]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                small![C!["haeding"]],
                p![
                    C!["title is-7"],
                    a![
                        attrs! {At::Href => mint.token_link,At::Target => "_blank"},
                        &mint.token_link
                    ]
                ]
            ]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                p![C!["haeding"], "Decimals"],
                p![C!["title"], &mint.decimals]
            ]
        ],
    ]
}

fn view_fee_payer_seed_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = ("", "", "");
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Fee payer"],
        div![
            C!["control has-icons-left has-icons-right"],
            //controlled form should be re-design
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value =>""},
                input_ev(Ev::Input, move |data| log!(data))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_mint_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = ("", "", "");
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Mint"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => ""},
                input_ev(Ev::Input, move |data| log!(data))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_owner_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = ("", "", "");
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Owner"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => ""},
                input_ev(Ev::Input, move |data| log!(data))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_button(ctx: &Context) -> Node<Msg> {
    let net: String;
    let commit: String;
    if let Some(cluster) = ctx.cluster.as_ref() {
        net = cluster.to_owned();
    } else {
        net = "".to_string();
    }
    if let Some(commitment) = ctx.commitment.as_ref() {
        commit = commitment.to_owned();
    } else {
        commit = "".to_string()
    }

    div![
        C!["field"],
        attrs! {At::Style => "text-align:center"},
        div![
            C!["control"],
            button![
                C!["button is-link"],
                ev(Ev::Click, move |_| { log!("submit !") }),
                "Create Account"
            ]
        ]
    ]
}
// ------ ------
//     helper
// ------ ------
