use seed::{prelude::*, *};
use serde::Deserialize;

use crate::{create_token_account_js, Context};
use bs58::decode;

// ------ ------
//     Init
// ------ ------
pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        account: RemoteData::Notasked,
        form: Form::default(),
    }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
    account: RemoteData<Account>,
    form: Form,
}

#[derive(PartialEq)]
enum RemoteData<T> {
    Notasked,
    Loading,
    Loaded(T),
}

#[derive(Default, Debug, PartialEq)]
struct Form {
    fee_payer_seed: String,
    token_mint: String,
    owner: String,

    //controlled form validation
    form_error: FormErrors,
}
#[derive(Debug, PartialEq, Default)]
struct FormErrors {
    fee_payer_seed: Option<String>,
    token_mint: Option<String>,
    owner: Option<String>,
}

//Js fetch result
#[derive(Deserialize, Debug, PartialEq)]
struct Account {
    acconut: String,
    token_link: String,
}

impl Form {
    pub fn get_field(&self, key: &str) -> &String {
        if key == "fee_payer_seed" {
            &self.fee_payer_seed
        } else if key == "token_mint" {
            &self.token_mint
        } else if key == "owner" {
            &self.owner
        } else {
            panic!("non-exist field")
        }
    }
    pub fn get_mut_field(&mut self, key: &str) -> &mut String {
        if key == "fee_payer_seed" {
            &mut self.fee_payer_seed
        } else if key == "token_mint" {
            &mut self.token_mint
        } else if key == "owner" {
            &mut self.owner
        } else {
            panic!("non-exist field")
        }
    }
    pub fn get_error_field(&self, key: &str) -> &Option<String> {
        if key == "fee_payer_seed" {
            &self.form_error.fee_payer_seed
        } else if key == "token_mint" {
            &self.form_error.token_mint
        } else if key == "owner" {
            &self.form_error.owner
        } else {
            panic!("non-exist field")
        }
    }
    pub fn get_mut_error_field(&mut self, key: &str) -> &mut Option<String> {
        if key == "fee_payer_seed" {
            &mut self.form_error.fee_payer_seed
        } else if key == "token_mint" {
            &mut self.form_error.token_mint
        } else if key == "owner" {
            &mut self.form_error.owner
        } else {
            panic!("non-exist field")
        }
    }
    pub fn get_ui<'a>(&'a self, key: &str) -> (&'a str, &'a str, &'a str) {
        let field = self.get_error_field(key);
        let (input_color, icon, err_msg) = match field {
            Some(err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
            None => {
                if self.get_field(key).is_empty() {
                    ("", "", "")
                } else {
                    ("is-success", "fas fa-check", "Available")
                }
            }
        };
        (input_color, icon, err_msg)
    }
    pub fn check_input<'a>(&'a mut self, key: &str) {
        let input = self.get_field(key);
        if !input.is_empty() {
            match key {
                "fee_payer_seed" => {
                    let vec: Vec<&str> = if input.contains(",") {
                        input.split(",").collect()
                    } else {
                        input.split_whitespace().collect()
                    };
                    if vec.len() == 64 {
                        *self.get_mut_error_field(key) = None;
                    } else {
                        *self.get_mut_error_field(key) =
                            Some("Seed shuold have 64 numbers".to_string());
                    }
                }
                "token_mint" | "owner" => {
                    let decoded = decode(self.get_field(key).clone()).into_vec();
                    if let Ok(pubkey) = decoded {
                        if pubkey.len() == 32 {
                            *self.get_mut_error_field(key) = None
                        } else {
                            *self.get_mut_error_field(key) =
                                Some("Length of decode bytes should be 32".to_string());
                        }
                    }
                }

                _ => self.form_error = FormErrors::default(),
            }
        } else {
            self.form_error = FormErrors::default();
        }
    }
}

// ------ ------
//     Update
// ------ ------

//to make simple, we use String instead of &str
pub enum Msg {
    // ------ Controlled Form ------
    OnChange(String, String),
    OnSubmit(String, String),

    // ------ API ------
    CreateAccountFetch(Result<JsValue, JsValue>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::OnChange(name, data) => {
            *model.form.get_mut_field(&name) = data;
            model.form.check_input(&name);
        }
        Msg::OnSubmit(cluster, commitment) => {
            if (model.account != RemoteData::Loading || model.account != RemoteData::Notasked)
                && model.form != Form::default()
            {
                model.account = RemoteData::Loading;

                let cluster = cluster.to_owned();
                let commitment = commitment.to_owned();
                let feepayer_seed = model.form.fee_payer_seed.to_owned();
                let token_mint = model.form.token_mint.to_owned();
                let owner = model.form.owner.to_owned();
                orders.skip().perform_cmd(async {
                    Msg::CreateAccountFetch(unsafe {
                        create_token_account_js(
                            cluster,
                            commitment,
                            feepayer_seed,
                            token_mint,
                            owner,
                        )
                        .await
                    })
                });
            } else {
                log!("Unfinished Form")
            }
        }
        Msg::CreateAccountFetch(Ok(res)) => {
            if !res.is_undefined() {
                match serde_wasm_bindgen::from_value(res) {
                    Ok(account) => {
                        log!(account);
                        model.account = RemoteData::Loaded(account)
                    }
                    Err(err) => error!("result can not be deserialized"),
                }
            } else {
                log!("undefined res catched up from external JS file")
            }
        }
        Msg::CreateAccountFetch(Err(err)) => {
            error!("create token_account_js fail", err);
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    section![
        C!["section hero is-light is-fullheight-with-navbar"],
        attrs![At::Style => "padding-bottom: 10rem"],
        match &model.account {
            RemoteData::Loading => {
                view_loading_message("Loading...")
            }
            RemoteData::Notasked => {
                view_loading_message("Create Token Account")
            }
            RemoteData::Loaded(account) => {
                view_level_nav(account)
            }
        },
        h2![
            C!["title is-2"],
            attrs! {At::Style => "text-align:center"},
            "Create Token Account"
        ],
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

fn view_level_nav(account: &Account) -> Node<Msg> {
    nav![
        C!["level"],
        div![
            C!["level-item has-text-centered"],
            div![p![C!["haeding"], "Term"], p![C!["title"], "SOL"]]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                small![C!["haeding"]],
                p![
                    C!["title is-7"],
                    a![
                        attrs! {At::Href => account.token_link,At::Target => "_blank"},
                        &account.token_link
                    ]
                ]
            ]
        ],
        div![
            C!["level-item has-text-centered"],
            div![p![C!["haeding"], "Decimals"], p![C!["title"], "18"]]
        ],
    ]
}

fn view_fee_payer_seed_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = &model.form.get_ui("fee_payer_seed");
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Fee payer"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => model.form.fee_payer_seed},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "fee_payer_seed".to_owned(),
                    data
                ))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_mint_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = model.form.get_ui("token_mint");
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Mint"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => model.form.token_mint},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "token_mint".to_owned(),
                    data
                ))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_owner_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = model.form.get_ui("owner");
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Owner"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => model.form.owner},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "owner".to_owned(),
                    data
                ))
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
                ev(Ev::Click, move |_| { Msg::OnSubmit(net, commit) }),
                "Create Account"
            ]
        ]
    ]
}

// ------ ------ ------ ------
//     Helper functions
// ------ ------ ------ ------
