#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_variables)]

use core::panic;

use crate::{escrow_maker_js, escrow_taker_js, Context};
use bs58::decode;
use seed::{prelude::*, *};
use serde::Deserialize;

// ------ ------
//     Init
// ------ ------

// Idea ~
// mouseOnLeave when entering token -> checking both balance of given account
// kinds of like alias of "swap" functionality

// program_id
const PROGRAM_ID: &str = "2f81caFNCYHQgRepoq9oNLnbjt7rsrU8pfC8fGRtQZzi";

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let res = EscrowResponse {
        escrow_account: "A8bkizaAC3EePjYJjVSzfsUpKqTGREpyb89eT1FJyrzn".to_string(),
        signer: "A8bkizaAC3EePjYJjVSzfsUpKqTGREpyb89eT1FJyrzn".to_string(),
        token_to_send: "A8bkizaAC3EePjYJjVSzfsUpKqTGREpyb89eT1FJyrzn".to_string(),
        token_to_receive: "".to_string(),
        amount_to_send: "100".to_string(),
        amount_to_receive: "90".to_string(),
        token_link: "http://localhost:8000/escrow".to_string(),
    };
    Model {
        form: Form::default(),

        maker: RemoteData::Loaded(res.clone()),
        taker: RemoteData::Notasked,
    }
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug)]

pub struct Model {
    //form
    form: Form,
    // distinguish action by checking token
    //result
    maker: RemoteData<EscrowResponse>,
    taker: RemoteData<EscrowResponse>,
}

// ------ ------ ------
//     Model_Form
// ------ ------ ------

#[derive(Debug, Default, PartialEq)]
struct Form {
    fee_payer_seed: String,
    token_send: String,
    token_receive: String,
    //interchangable
    amount_to_send_or_escrow_account: Action<String>,

    amount_to_receive: String,

    form_error: FormErrors,
}

#[derive(Debug, Default, PartialEq)]
struct FormErrors {
    fee_payer_seed: Option<String>,
    token_send: Option<String>,
    token_receive: Option<String>,
    //interchangable
    amount_to_send_or_escrow_account: Action<Option<String>>,
    // amount_to_send: Option<String>,
    escrow_account: Option<String>,
    amount_to_receive: Option<String>,
}

const FORM_FEE_PAYER_SEED: &str = "fee_payer_seed";
const FORM_TOKEN_SEND: &str = "token_send";
const FORM_TOKEN_RECEIVE: &str = "token_receive";
//interchangable input
const FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT: &str = "amount_to_send_or_escrow_account";
// const FORM_AMOUNT_TO_SEND: &str = "amount_to_send";
// const FORM_ESCROW_ACCOUNT: &str = "escrow_account";

const FORM_AMOUNT_TO_RECEIVE: &str = "amount_to_receive";

impl Form {
    pub fn get_field(&self, key: &str) -> &String {
        match key {
            FORM_FEE_PAYER_SEED => &self.fee_payer_seed,
            FORM_TOKEN_SEND => &self.token_send,
            FORM_TOKEN_RECEIVE => &self.token_receive,
            FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT => match &self.amount_to_send_or_escrow_account {
                Action::Maker(form_maker) => form_maker,
                Action::Taker(form_taker) => form_taker,
            },
            FORM_AMOUNT_TO_RECEIVE => &self.amount_to_receive,
            _ => panic!("non-exist field"),
        }
    }
    pub fn get_mut_field(&mut self, key: &str) -> &mut String {
        match key {
            FORM_FEE_PAYER_SEED => &mut self.fee_payer_seed,
            FORM_TOKEN_SEND => &mut self.token_send,
            FORM_TOKEN_RECEIVE => &mut self.token_receive,
            FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT => {
                match &mut self.amount_to_send_or_escrow_account {
                    Action::Maker(form_maker) => form_maker,
                    Action::Taker(form_taker) => form_taker,
                }
            }
            FORM_AMOUNT_TO_RECEIVE => &mut self.amount_to_receive,
            _ => panic!("non-exist field"),
        }
    }
    pub fn get_error_field(&self, key: &str) -> &Option<String> {
        match key {
            FORM_FEE_PAYER_SEED => &self.form_error.fee_payer_seed,
            FORM_TOKEN_SEND => &self.form_error.token_send,
            FORM_TOKEN_RECEIVE => &self.form_error.token_receive,
            FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT => {
                match &self.form_error.amount_to_send_or_escrow_account {
                    Action::Maker(form_maker) => form_maker,
                    Action::Taker(form_taker) => form_taker,
                }
            }
            FORM_AMOUNT_TO_RECEIVE => &self.form_error.amount_to_receive,
            _ => panic!("non-exist field"),
        }
    }
    pub fn get_mut_error_field(&mut self, key: &str) -> &mut Option<String> {
        match key {
            FORM_FEE_PAYER_SEED => &mut self.form_error.fee_payer_seed,
            FORM_TOKEN_SEND => &mut self.form_error.token_send,
            FORM_TOKEN_RECEIVE => &mut self.form_error.token_receive,
            FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT => {
                match &mut self.form_error.amount_to_send_or_escrow_account {
                    Action::Maker(form_maker) => form_maker,
                    Action::Taker(form_taker) => form_taker,
                }
            }
            FORM_AMOUNT_TO_RECEIVE => &mut self.form_error.amount_to_receive,
            _ => panic!("non-exist field"),
        }
    }
    pub fn get_fields_const(&self) -> [&str; 5] {
        [
            FORM_FEE_PAYER_SEED,
            FORM_TOKEN_SEND,
            FORM_TOKEN_RECEIVE,
            FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT,
            FORM_AMOUNT_TO_RECEIVE,
        ]
    }

    pub fn get_fields_ui(&self) -> Vec<(&str, &str, &str)> {
        let fields_names = self.get_fields_const();

        let res = fields_names
            .iter()
            .map(|field| self.get_ui(field))
            .collect();

        res
    }

    pub fn get_ui<'a>(&'a self, key: &str) -> (&'a str, &'a str, &'a str) {
        let current_err = self.get_error_field(key);

        let (status, icon, err_msg) = match current_err {
            Some(err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
            None => {
                if self.get_field(key).is_empty() {
                    ("", "", "")
                } else {
                    ("is-success", "fas fa-check", "Available")
                }
            }
        };

        (status, icon, err_msg)
    }

    pub fn check_input(&mut self, key: &str) {
        let input = self.get_field(key);

        if !input.is_empty() {
            match key {
                FORM_FEE_PAYER_SEED => {
                    self.check_seed(key);
                }
                FORM_TOKEN_RECEIVE | FORM_TOKEN_SEND => {
                    self.check_pubkey(key);
                }
                FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT => {
                    match self.amount_to_send_or_escrow_account {
                        Action::Maker(_) => {
                            self.check_amount(key);
                        }
                        Action::Taker(_) => {
                            self.check_pubkey(key);
                        }
                    }
                }
                FORM_AMOUNT_TO_RECEIVE => {
                    self.check_amount(key);
                }
                _ => panic!("non-exist field"),
            }
        } else {
            *self.get_mut_error_field(key) = None;
        }
    }

    fn check_seed(&mut self, key: &str) {
        let input = self.get_field(key);
        let vec: Vec<&str> = if input.contains(",") {
            input.split(",").collect()
        } else {
            input.split_whitespace().collect()
        };

        if vec.len() == 64 {
            *self.get_mut_error_field(key) = None;
        } else {
            *self.get_mut_error_field(key) = Some("Seed should have 64 numbers".to_string())
        }
    }

    fn check_pubkey(&mut self, key: &str) {
        let decoded = decode(self.get_field(key).clone()).into_vec();

        if let Ok(pubkey) = decoded {
            if pubkey.len() == 32 {
                *self.get_mut_error_field(key) = None;
            } else {
                *self.get_mut_error_field(key) =
                    Some("Length of bytes code should be 32".to_string())
            }
        }
    }

    fn check_amount(&mut self, key: &str) {
        let parsed = self.get_mut_field(key).trim().parse::<u64>();

        if let Ok(amount) = parsed {
            if amount > 0 {
                *self.get_mut_error_field(key) = None;
            } else {
                *self.get_mut_error_field(key) =
                    Some("exchanged amount should be positive".to_owned());
            }
        } else {
            *self.get_mut_error_field(key) = Some("parsed fail".to_owned());
        }
    }

    pub fn check_all_errors(&self) -> bool {
        let fields = self.get_fields_const();
        let mut err_passed = true;

        for key in fields.iter() {
            if self.get_error_field(key).is_some() {
                err_passed = false;
            }
        }

        err_passed
    }
}

// ------ ------ ------
//      Form_Actions
// ------ ------ ------
#[derive(Debug, PartialEq)]
enum Action<T> {
    Maker(T),

    Taker(T),
}

impl<T> Default for Action<T>
where
    T: Default,
{
    fn default() -> Self {
        let value = T::default();
        Self::Maker(value)
    }
}

#[derive(PartialEq, Debug)]
enum RemoteData<T> {
    Notasked,
    Loading,
    Loaded(T),
}

// ------ ------ ------
//     Model_Response
// ------ ------ ------

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
struct EscrowResponse {
    escrow_account: String,
    signer: String,
    token_to_send: String,
    token_to_receive: String,
    amount_to_send: String,
    amount_to_receive: String,
    token_link: String,
}

// ------ ------
//     Update
// ------ ------

pub enum Msg {
    OnChange(String, String),
    MakerOnSubmit(String, String),
    TakerOnSubmit(String, String),

    //external API
    MakerJSRes(Result<JsValue, JsValue>),
    TakerJSRes(Result<JsValue, JsValue>),

    //Toggle Actions
    ToggleAction,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::OnChange(key, data) => {
            *model.form.get_mut_field(&key) = data;
            model.form.check_input(&key);
        }
        Msg::MakerOnSubmit(cluster, commitment) => {
            log!(&model);

            if model.maker != RemoteData::Loading
                && model.form.check_all_errors()
                && model.form != Form::default()
            {
                let cluster = cluster.to_owned();
                let commitment = commitment.to_owned();
                let fee_payer_seed = model.form.fee_payer_seed.to_owned();
                let token_to_send = model.form.token_send.to_owned();
                let token_to_receive = model.form.token_receive.to_owned();
                let amount_to_send =
                    if let Action::Maker(input) = &model.form.amount_to_send_or_escrow_account {
                        input.to_owned()
                    } else {
                        panic!("maker input should not fail")
                    };
                let amount_to_receive = model.form.amount_to_receive.to_owned();

                orders.skip().perform_cmd(async {
                    Msg::MakerJSRes(unsafe {
                        escrow_maker_js(
                            cluster,
                            commitment,
                            fee_payer_seed,
                            token_to_send,
                            token_to_receive,
                            amount_to_send, //here stands for 'amount to send'
                            amount_to_receive,
                            PROGRAM_ID.to_string(),
                        )
                        .await
                    })
                });
            }
        }
        Msg::TakerOnSubmit(cluster, commitment) => {
            log!(&model.form);
            if model.taker != RemoteData::Loading
                && model.form.check_all_errors()
                && model.form != Form::default()
            {
                let cluster = cluster.to_owned();
                let commitment = commitment.to_owned();
                let fee_payer_seed = model.form.fee_payer_seed.to_owned();
                let token_to_send = model.form.token_send.to_owned();
                let token_to_receive = model.form.token_receive.to_owned();
                let escrow_account =
                    if let Action::Taker(input) = &model.form.amount_to_send_or_escrow_account {
                        input.to_owned()
                    } else {
                        panic!("input should not be null")
                    };
                let amount_to_receive = model.form.amount_to_receive.to_owned();

                orders.skip().perform_cmd(async {
                    Msg::TakerJSRes(unsafe {
                        escrow_taker_js(
                            cluster,
                            commitment,
                            fee_payer_seed,
                            token_to_send,
                            token_to_receive,
                            escrow_account, //here becomes 'escrow account'
                            amount_to_receive,
                            PROGRAM_ID.to_string(),
                        )
                        .await
                    })
                });
            }
        }

        Msg::MakerJSRes(Ok(maker_res)) => {
            if !maker_res.is_undefined() {
                match serde_wasm_bindgen::from_value(maker_res) {
                    Ok(res) => {
                        log!(res);
                        model.maker = RemoteData::Loaded(res)
                    }
                    Err(_) => error!("result can not be deserizlied"),
                }
            } else {
                log!("escrow_maker_js response undefined")
            }
        }
        Msg::MakerJSRes(Err(_)) => {
            error!("escrow_maker_js fail")
        }
        Msg::TakerJSRes(Ok(taker_res)) => {
            if !taker_res.is_undefined() {
                match serde_wasm_bindgen::from_value(taker_res) {
                    Ok(res) => {
                        log!(res);
                        model.taker = RemoteData::Loaded(res)
                    }
                    Err(_) => error!("result can not be deserizlied"),
                }
            } else {
                log!("escrow_taker_js response undefined")
            }
        }
        Msg::TakerJSRes(Err(_)) => {
            error!("escrow_taker_js fail")
        }
        Msg::ToggleAction => {
            model.form.amount_to_send_or_escrow_account =
                match &mut model.form.amount_to_send_or_escrow_account {
                    Action::Maker(input) => {
                        *input = "".to_string();
                        Action::Taker("".to_string())
                    }
                    Action::Taker(input) => {
                        *input = "".to_string();
                        Action::Maker("".to_string())
                    }
                }
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    section![
        C!["section hero is-light is-fullheight-with-navbar"],
        attrs![At::Style => "padding: 0.5rem 1.5rem"],
        // Error7 Loading Message
        match &model.maker {
            RemoteData::Loading => {
                view_loading_message("Loading...")
            }
            RemoteData::Notasked => {
                view_loading_message("Escrow Maker")
            }
            RemoteData::Loaded(res) => {
                view_level_nav(res)
            }
        },
        //Title
        match model.form.amount_to_send_or_escrow_account {
            Action::Maker(_) => {
                view_title("Escrow Maker")
            }
            Action::Taker(_) => {
                view_title("Escrow Taker")
            }
            _ => empty(),
        },
        //Result
        if let RemoteData::Loaded(maker_res) = &model.maker {
            view_result(&maker_res.amount_to_receive)
        } else {
            empty()
        },
        //Form
        view_form(&model.form, ctx),
    ]
}

fn view_level_nav(res: &EscrowResponse) -> Node<Msg> {
    // ------ returned instance ------
    // deposited_address: String,
    // amount: String,
    // decimals: String,
    // token_link: String,
    nav![
        C!["level"],
        div![
            C!["level-item has-text-centered"],
            div![
                p![C!["haeding"], "Sent Amount"],
                p![C!["title"], &res.amount_to_send]
            ]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                small![C!["haeding"]],
                p![
                    C!["title is-7"],
                    a![
                        attrs! {At::Href => res.token_link,At::Target => "_blank"},
                        &res.token_link
                    ]
                ]
            ]
        ],
        div![
            C!["level-item has-text-centered"],
            div![
                p![C!["haeding"], "Received Amount"],
                p![C!["title"], &res.amount_to_receive]
            ]
        ],
    ]
}
pub fn view_loading_message(msg: &str) -> Node<Msg> {
    div![div![C!["message"], msg]]
}
pub fn view_title(title: &str) -> Node<Msg> {
    h2![
        C!["title is-2"],
        attrs! {At::Style => "text-align:center"},
        title
    ]
}

fn view_result(res: &str) -> Node<Msg> {
    div![
        C!["container is-fullhd"],
        attrs! {At::Style => "text-align:center"},
        div![
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["message"], res]
        ]
    ]
}
fn view_selection() -> Node<Msg> {
    ul![
        C!["filters"],
        li![a![C!["selected"], " mint",]],
        li![a![" transfer",]],
        li![a![" freeze",]],
        li![a![" burn",]]
    ]
}

// ------ ------
//     Forms
// ------ ------
fn view_form(form: &Form, ctx: &Context) -> Node<Msg> {
    let ui = form.get_fields_ui();
    //differnetiate action's value
    let (title, input_type, value) = match &form.amount_to_send_or_escrow_account {
        Action::Maker(maker_input) => ("Amount To Send", "number", maker_input.as_str()),
        Action::Taker(taker_input) => ("Escrow Account", "text", taker_input.as_str()),
    };

    // fee_payer_seed: String,
    // token_send: String,
    // token_receive: String,
    // //interchangable
    // amount_to_send_or_escrow_account: Action<String>,

    // amount_to_receive: String

    form![
        ev(Ev::Submit, move |event| {
            event.prevent_default();
        }),
        view_input(
            FORM_FEE_PAYER_SEED,
            "Fee Payer Seed",
            "text",
            ui[0].0,
            ui[0].1,
            ui[0].2,
            &form.fee_payer_seed
        ),
        view_input(
            FORM_TOKEN_SEND,
            "Token To Send",
            "text",
            ui[1].0,
            ui[1].1,
            ui[1].2,
            &form.token_send
        ),
        //interchangable inputs
        view_input(
            FORM_TOKEN_RECEIVE,
            "Token To Receive",
            "text",
            ui[2].0,
            ui[2].1,
            ui[2].2,
            &form.token_receive
        ),
        view_input(
            FORM_AMOUNT_TO_SEND_OR_ESCROW_ACCOUNT,
            title,
            input_type,
            ui[3].0,
            ui[3].1,
            ui[3].2,
            value
        ),
        view_input(
            FORM_AMOUNT_TO_RECEIVE,
            "Amount To Receive",
            "number",
            ui[4].0,
            ui[4].1,
            ui[4].2,
            &form.amount_to_receive
        ),
        div![
            C!["field is-grouped"],
            attrs! {At::Style => "text-align:center"},
            div![
                C!["control"],
                button![
                    C!["button is-link"],
                    ev(Ev::Click, move |_| { Msg::ToggleAction }),
                    "Change Action "
                ]
            ],
            match form.amount_to_send_or_escrow_account {
                Action::Maker(_) => {
                    view_maker_button(ctx)
                }
                Action::Taker(_) => {
                    view_taker_button(ctx)
                }
            }
        ],
    ]
}
// match action -> match form field -> input
fn view_input(
    key: &'static str,
    label: &str,
    input_type: &str,
    input_color: &str,
    icon: &str,
    err_msg: &str,
    value: &str,
) -> Node<Msg> {
    div![
        C!["field"],
        attrs![At::Style => "padding: 0 12rem"],
        label![C!["label"], label],
        div![
            C!["control has-icons-left has-icons-right"],
            input![
                C![format!("input {}", input_color)],
                attrs! {At::Type => input_type,At::Placeholder => "Text input",At::Value => value},
                input_ev(Ev::Input, move |data| Msg::OnChange(key.to_owned(), data))
            ],
            span![C!["icon is-small is-left"], i![C!["fas fa-key"]]],
            span![C!["icon is-small is-right"], i![C![icon]]]
        ],
        p![C![format!("help {}", input_color)], err_msg]
    ]
}

fn view_maker_button(ctx: &Context) -> Node<Msg> {
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
                ev(Ev::Click, move |_| { Msg::MakerOnSubmit(net, commit) }),
                "Make the Escrow"
            ]
        ]
    ]
}
fn view_taker_button(ctx: &Context) -> Node<Msg> {
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
        C!["control"],
        button![
            C!["button is-link"],
            ev(Ev::Click, move |_| { Msg::TakerOnSubmit(net, commit) }),
            "Take the Escrow "
        ]
    ]
}
