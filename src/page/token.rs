#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_variables)]

use crate::{create_mint_js, Context};
use bs58::decode;
use seed::{prelude::*, *};
use serde::Deserialize;
// ------ ------ ------ ------ ------
//      Import from Root Model
// ------ ------ ------ ------ ------

//filename shoudl be snackcase
mod create_account;
mod edit_token;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!(&url);

    Model {
        errors: Vec::new(),
        base_url: url.to_base_url(),
        page: Page::init(url, orders),

        token: RemoteData::Notasked,
        form: Form::default(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    //fetch loa
    errors: Vec<FetchError>,

    //state
    token: RemoteData<Token>,
    form: Form,

    base_url: Url,
    page: Page,
}

#[derive(Debug, Default, PartialEq)]
struct Form {
    //use null will be over-whelming, check by &str.is_empty() instead
    payer_secret: String,
    mint_authority_address: String,
    freeze_authority_address: String,
    token_decimals: String,
    form_error: FormErrors,
}

impl Form {
    pub fn get_field(&self, key: &String) -> &String {
        if key == "payer_secret" {
            &self.payer_secret
        } else if key == "mint_authority_address" {
            &self.mint_authority_address
        } else if key == "token_decimals" {
            &self.token_decimals
        } else if key == "freeze_authority_address" {
            &self.freeze_authority_address
        } else {
            panic!("invalid field")
        }
    }
    pub fn get_mut_field(&mut self, key: &String) -> &mut String {
        if key == "payer_secret" {
            &mut self.payer_secret
        } else if key == "mint_authority_address" {
            &mut self.mint_authority_address
        } else if key == "token_decimals" {
            &mut self.token_decimals
        } else if key == "freeze_authority_address" {
            &mut self.freeze_authority_address
        } else {
            panic!("invalid field")
        }
    }
}

//default to all be None
#[derive(Default, PartialEq, Debug)]
struct FormErrors {
    payer_secret: Option<String>,
    mint_authority_address: Option<String>,
    freeze_authority_address: Option<String>,
    token_decimals: Option<String>,
}

#[derive(PartialEq, PartialOrd)]
enum RemoteData<T> {
    Notasked,
    Loading,
    Loaded(T),
}

//response from external API
#[derive(Deserialize, Debug, PartialEq)]
struct Token {
    created_mint_address: String,
    decimals: String,
    supply: String,

    token_link: String,
}

// ------ ------ ------
//      Model_Url
// ------ ------ -----
const CREATE_ACCOUNT: &str = "createAccount";
const EDIT_TOKEN: &str = "editToken";

struct_urls!();
impl<'a> Urls<'a> {
    pub fn default(self) -> Url {
        self.create_account()
    }
    pub fn create_account(self) -> Url {
        self.base_url().add_path_part(CREATE_ACCOUNT)
    }
    pub fn edit_token(self) -> Url {
        self.base_url().add_path_part(EDIT_TOKEN)
    }
}

enum Page {
    Home,
    CreateAccount(create_account::Model),
    EditToken(edit_token::Model),
}

impl Page {
    pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        //if threre only one level down sub-path, then we use remaining path
        match url.remaining_path_parts().as_slice().clone() {
            [CREATE_ACCOUNT] => Self::CreateAccount(create_account::init(
                url,
                &mut orders.proxy(Msg::CreateAccount),
            )),
            [EDIT_TOKEN] => {
                Self::EditToken(edit_token::init(url, &mut orders.proxy(Msg::EditToken)))
            }
            _ => Self::Home,
        }
    }
}

// ------ ------
//     Update
// ------ ------

pub enum Msg {
    //Token
    CreateTokenFetch(Result<JsValue, JsValue>),

    //controlled Input& Form
    OnChange(String, String),
    OnSubmit(String, String),

    //Page
    CreateAccount(create_account::Msg),
    EditToken(edit_token::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        //Token
        Msg::CreateTokenFetch(Ok(res)) => {
            if !res.is_undefined() {
                match serde_wasm_bindgen::from_value(res) {
                    Ok(token_res) => {
                        log!(token_res);
                        model.token = RemoteData::Loaded(token_res)
                    }
                    Err(err) => error!("result unable to deserialized"),
                }
            } else {
                log!("undefined returned value from JS file");
            }
        }
        Msg::CreateTokenFetch(Err(err)) => {
            //when external API fail
            //creating our own customized Err message

            error!("create token fail !", err);
        }

        //Form controlled
        Msg::OnChange(name, data) => {
            //update controlled state
            *model.form.get_mut_field(&name) = data;

            check_input_format(&name, &mut model.form);
        }
        Msg::OnSubmit(cluster, commitment) => {
            //check all input is not empty
            if (model.token != RemoteData::Loading || model.token != RemoteData::Notasked)
                && model.form != Form::default()
            {
                model.token = RemoteData::Loading;
                // to pass the data to external API, clone the string
                let payer_secret = model.form.payer_secret.to_owned();
                let mint_authority_address = model.form.mint_authority_address.to_owned();
                let freeze_authority_address = model.form.freeze_authority_address.to_owned();
                let token_decimals = model.form.token_decimals.to_owned();

                orders.skip().perform_cmd(async {
                    Msg::CreateTokenFetch(unsafe {
                        create_mint_js(
                            cluster,
                            commitment,
                            payer_secret,
                            mint_authority_address,
                            freeze_authority_address,
                            token_decimals,
                        )
                        .await
                    })
                    //external API
                });
            } else {
                log!("unfinished form !!")
            }
        }
        Msg::CreateAccount(msg) => {
            if let Page::CreateAccount(model) = &mut model.page {
                create_account::update(msg, model, &mut orders.proxy(Msg::CreateAccount))
            }
        }
        Msg::EditToken(msg) => {
            if let Page::EditToken(model) = &mut model.page {
                edit_token::update(msg, model, &mut orders.proxy(Msg::EditToken))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

//to lifting up the "state of root model"
// state from root Model -> view function (children) -> Msg::Desired_function
pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    match &model.page {
        Page::Home => view_content(model, ctx),
        Page::CreateAccount(create_account_model) => {
            create_account::view(create_account_model, ctx).map_msg(Msg::CreateAccount)
            //saying that the view function only accept "this type of message"
        }
        Page::EditToken(edit_token_model) => {
            edit_token::view(edit_token_model, ctx).map_msg(Msg::EditToken)
        }
    }
}
fn view_content(model: &Model, ctx: &Context) -> Node<Msg> {
    section![
        C!["section hero is-light is-fullheight-with-navbar"],
        attrs![At::Style => "padding-bottom:  10rem"],
        match &model.token {
            //Loading message
            RemoteData::Loading => {
                view_loading_message("Loading...")
            }
            RemoteData::Notasked => {
                view_loading_message("Create Your Token")
            }
            //Successful Loaded
            RemoteData::Loaded(token) => {
                view_level_nav(token)
            }
        },
        h2![
            C!["title is-2"],
            attrs! {At::Style => "text-align:center"},
            "Token Creator"
        ],
        if let RemoteData::Loaded(token) = &model.token {
            div![
                C!["container is-fullhd"],
                attrs! {At::Style => "text-align:center"},
                div![
                    span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
                    span![C!["message"], &token.created_mint_address]
                ]
            ]
        } else {
            empty()
        },
        form![
            ev(Ev::Submit, move |event| {
                event.prevent_default();
            }),
            view_seed_input(model),
            view_mint_authority_input(model),
            view_freeze_authoruty_input(model),
            view_decimals_input(model),
            view_button(ctx)
        ]
    ]
}
pub fn view_loading_message(msg: &str) -> Node<Msg> {
    div![div![C!["message"], msg]]
}
fn view_level_nav(token: &Token) -> Node<Msg> {
    nav![
        C!["level"],
        div![
            C!["level-item has-text-centered"],
            div![p![C!["haeding"], "Supply"], p![C!["title"], &token.supply]]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                small![C!["haeding"]],
                p![
                    C!["title is-7"],
                    a![
                        attrs! {At::Href => token.token_link,At::Target => "_blank"},
                        &token.token_link
                    ]
                ]
            ]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                p![C!["haeding"], "Decimals"],
                p![C!["title"], &token.decimals]
            ]
        ],
    ]
}
fn view_seed_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = match &model.form.form_error.payer_secret {
        Some(err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
        None => {
            if model.form.payer_secret.is_empty() {
                ("", "", "")
            } else {
                ("is-success", "fas fa-check", "Available Seed")
            }
        }
    };

    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Seed"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => model.form.payer_secret},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "payer_secret".to_owned(),
                    data
                ))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_mint_authority_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = match &model.form.form_error.mint_authority_address {
        Some(err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
        None => {
            if model.form.mint_authority_address.is_empty() {
                ("", "", "")
            } else {
                ("is-success", "fas fa-check", "Available Pubkey")
            }
        }
    };
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Mint authority"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => model.form.mint_authority_address},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "mint_authority_address".to_owned(),
                    data
                ))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-user"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_freeze_authoruty_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = match &model.form.form_error.freeze_authority_address {
        Some(err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
        None => {
            if model.form.freeze_authority_address.is_empty() {
                ("", "", "")
            } else {
                ("is-success", "fas fa-check", "Available Pubkey")
            }
        }
    };

    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Freeze authority"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "text",At::Placeholder => "Text input",At::Value => model.form.freeze_authority_address},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "freeze_authority_address".to_owned(),
                    data
                ))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-user"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}
fn view_decimals_input(model: &Model) -> Node<Msg> {
    let (input_color, icon, err_msg) = match &model.form.form_error.token_decimals {
        Some(err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
        None => {
            if model.form.token_decimals.is_empty() {
                ("", "", "")
            } else {
                ("is-success", "fas fa-check", "Available decimals")
            }
        }
    };

    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], "Decimals"],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => "number",At::Placeholder => "Number input",At::Value => model.form.token_decimals},
                input_ev(Ev::Input, move |data| Msg::OnChange(
                    "token_decimals".to_owned(),
                    data
                ))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-coins"]]],
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
        commit = "".to_string();
    }

    div![
        C!["field"],
        attrs! {At::Style => "text-align:center"},
        div![
            C!["control"],
            button![
                C!["button is-link"],
                ev(Ev::Click, move |_| { Msg::OnSubmit(net, commit) }),
                "Create new token"
            ]
        ]
    ]
}

fn check_input_format(name: &String, form: &mut Form) {
    if name == "payer_secret" {
        //desired format: [u8;64]

        let error = &mut form.form_error.payer_secret;
        let input = form.payer_secret.trim();
        let mut vec: Vec<&str> = Vec::new();

        if input.contains(",") {
            vec = input.split(",").collect();
        } else if input.contains(" ") {
            vec = input.split_whitespace().collect();
        }

        if form.payer_secret.is_empty() {
            *error = None;
        } else if vec.len() == 64 {
            *error = None;
        } else {
            *error = Some("Seed shuold have 64 numbers".to_string());
        }
    } else if name == "freeze_authority_address" {
        //decode bytes should be 58

        let error = &mut form.form_error.freeze_authority_address;
        let decoded = decode(form.freeze_authority_address.to_owned()).into_vec();

        if form.freeze_authority_address.is_empty() {
            *error = None;
        } else {
            if let Ok(pubkey) = decoded {
                if pubkey.len() == 32 {
                    *error = None;
                } else {
                    *error = Some("length of decode bytes is not 32".to_string());
                }
            }
        }
    } else if name == "mint_authority_address" {
        //decode bytes should be 58

        let error = &mut form.form_error.mint_authority_address;
        let decoded = bs58::decode(form.mint_authority_address.to_owned()).into_vec();

        if form.mint_authority_address.is_empty() {
            *error = None;
        } else {
            if let Ok(pubkey) = decoded {
                if pubkey.len() == 32 {
                    *error = None;
                } else {
                    *error = Some("length of decode bytes is not 32".to_string());
                }
            }
        }
    } else if name == "token_decimals" {
        let input = form.token_decimals.trim().parse::<u8>();
        let error = &mut form.form_error.token_decimals;

        if form.token_decimals.is_empty() {
            *error = None;
        } else {
            if let Ok(num) = input {
                match num {
                    4..=10 => *error = None,
                    _ => *error = Some("decimals place should between 4 and 10".to_string()),
                }
            } else {
                *error = Some("number only".to_owned())
            }
        }
    } else {
        form.form_error = FormErrors::default()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let input = "0".to_owned();
        let foo: u8;
        let res = input.parse::<u8>().expect("parse error");
        match res {
            0..=10 => foo = 10,
            _ => foo = 0,
        }
        assert_eq!(foo, 0);
    }
}
