use bs58::decode;
use seed::{prelude::*, *};
use serde::Deserialize;

use crate::{mint_token_js, Context};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        actions: Action::Mint(MintForm::default()),

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
#[derive(PartialEq)]
enum RemoteData<T> {
    Notasked,
    Loading,
    Loaded(T),
}

//retunr value

pub enum Action {
    Mint(MintForm),
    Transfer,
    Freeze,
    SetOwner,
    SetCloser,
    Burn,
}

//derive the fitler from 'given action name'

// ------ ------ ------
//    Model_Action_Form
// ------ ------ ------

// ------ Mint ------
#[derive(Default, Debug, PartialEq)]
pub struct MintForm {
    fee_payer_seed: String,
    token_mint: String,
    destination_account: String,
    mint_authority_seed: String,
    amount: String,

    form_error: MintFormErrors,
}
#[derive(Default, Debug, PartialEq)]
struct MintFormErrors {
    fee_payer_seed: Option<String>,
    token_mint: Option<String>,
    destination_account: Option<String>,
    mint_authority_seed: Option<String>,
    amount: Option<String>,
}

const MINT_FORM_FEE_PAYER_SEED: &str = "fee_payer_seed";
const MINT_FORM_TOKEN_MINT: &str = "token_mint";
const MINT_FORM_DESTINATION_ACCOUNT: &str = "destination_account";
const MINT_FORM_MINT_AUTHORITY_SEED: &str = "mint_authority_seed";
const MINT_FORM_AMOUNT: &str = "amount";

impl MintForm {
    pub fn get_field(&self, key: &str) -> &String {
        match key {
            MINT_FORM_FEE_PAYER_SEED => &self.fee_payer_seed,
            MINT_FORM_TOKEN_MINT => &self.token_mint,
            MINT_FORM_DESTINATION_ACCOUNT => &self.destination_account,
            MINT_FORM_MINT_AUTHORITY_SEED => &self.mint_authority_seed,
            MINT_FORM_AMOUNT => &self.amount,
            _ => {
                panic!("non-exist field !! ")
            }
        }
    }
    pub fn get_mut_field(&mut self, key: &str) -> &mut String {
        match key {
            MINT_FORM_FEE_PAYER_SEED => &mut self.fee_payer_seed,
            MINT_FORM_TOKEN_MINT => &mut self.token_mint,
            MINT_FORM_DESTINATION_ACCOUNT => &mut self.destination_account,
            MINT_FORM_MINT_AUTHORITY_SEED => &mut self.mint_authority_seed,
            MINT_FORM_AMOUNT => &mut self.amount,
            _ => {
                panic!("non-exist field !! ")
            }
        }
    }

    pub fn get_errors_field(&self, key: &str) -> &Option<String> {
        match key {
            MINT_FORM_FEE_PAYER_SEED => &self.form_error.fee_payer_seed,
            MINT_FORM_TOKEN_MINT => &self.form_error.token_mint,
            MINT_FORM_DESTINATION_ACCOUNT => &self.form_error.destination_account,
            MINT_FORM_MINT_AUTHORITY_SEED => &self.form_error.mint_authority_seed,
            MINT_FORM_AMOUNT => &self.form_error.amount,
            _ => panic!("non-exist field !!"),
        }
    }
    pub fn get_mut_errors_field(&mut self, key: &str) -> &mut Option<String> {
        match key {
            MINT_FORM_FEE_PAYER_SEED => &mut self.form_error.fee_payer_seed,
            MINT_FORM_TOKEN_MINT => &mut self.form_error.token_mint,
            MINT_FORM_DESTINATION_ACCOUNT => &mut self.form_error.destination_account,
            MINT_FORM_MINT_AUTHORITY_SEED => &mut self.form_error.mint_authority_seed,
            MINT_FORM_AMOUNT => &mut self.form_error.amount,
            _ => panic!("non-exist field !!"),
        }
    }
    fn get_fields_const(&self) -> [&str; 5] {
        [
            MINT_FORM_FEE_PAYER_SEED,
            MINT_FORM_TOKEN_MINT,
            MINT_FORM_DESTINATION_ACCOUNT,
            MINT_FORM_MINT_AUTHORITY_SEED,
            MINT_FORM_AMOUNT,
        ]
    }
    pub fn get_field_ui(&self) -> Vec<(&str, &str, &str)> {
        let field_names = self.get_fields_const();
        let res = field_names.iter().map(|field| self.get_ui(field)).collect();

        res
    }
    pub fn get_ui<'a>(&'a self, key: &str) -> (&'a str, &'a str, &'a str) {
        let current_err = self.get_errors_field(key);

        let (status, icon, err_msg) = match current_err {
            &Some(ref err_msg) => ("is-danger", "fas fa-exclamation-triangle", err_msg.as_ref()),
            &None => {
                if self.get_field(key).is_empty() {
                    ("", "", "")
                } else {
                    ("is-success", "fas fa-check", "Available")
                }
            }
        };

        (status, icon, err_msg)
    }
    pub fn check_input<'a>(&'a mut self, key: &str) {
        let input = self.get_field(key);

        if !input.is_empty() {
            match key {
                //Seed
                MINT_FORM_FEE_PAYER_SEED | MINT_FORM_MINT_AUTHORITY_SEED => {
                    let vec: Vec<&str> = if input.contains(",") {
                        input.split(",").collect()
                    } else {
                        input.split_whitespace().collect()
                    };

                    if vec.len() == 64 {
                        *self.get_mut_errors_field(key) = None;
                    } else {
                        *self.get_mut_errors_field(key) =
                            Some("Seed should have 64 numbers".to_string())
                    }
                }
                //Pubkey
                MINT_FORM_DESTINATION_ACCOUNT | MINT_FORM_TOKEN_MINT => {
                    let decoded = decode(self.get_field(key).clone()).into_vec();

                    if let Ok(pubkey) = decoded {
                        if pubkey.len() == 32 {
                            *self.get_mut_errors_field(key) = None;
                        } else {
                            *self.get_mut_errors_field(key) =
                                Some("Length of bytes code should be 32".to_string())
                        }
                    }
                }
                //Amount
                MINT_FORM_AMOUNT => {
                    let parsed = self.get_mut_field(key).trim().parse::<u8>();

                    if let Ok(amount) = parsed {
                        log!(amount);
                        if amount > 0 {
                            *self.get_mut_errors_field(key) = None;
                        } else {
                            *self.get_mut_errors_field(key) =
                                Some("mint amount should be positive".to_owned());
                        }
                    }
                }
                _ => panic!("non-exist input"),
            }
        } else {
            *self.get_mut_errors_field(key) = None;
        }
    }

    pub fn check_all_errors(&self) -> bool {
        //since fields of errors and inputs are identical,
        //we could use same iterator
        let fields = self.get_fields_const();
        let mut err_passed: bool = true;

        for field_key in fields.iter() {
            if self.get_errors_field(field_key).is_none() {
                err_passed = false;
            }
        }

        err_passed
    }
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
#[derive(Deserialize, Debug, PartialEq)]
struct Mint {
    deposited_address: String,
    amount: String,
    decimals: String,

    token_link: String,
}
#[derive(Deserialize, Debug, PartialEq)]
struct Transfer {
    foo: String,
    bar: String,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    MintToken,
    OnSelect,
    OnChange(String, String),
    OnSubmit(String, String), //since context comes from view function, it is not possibly to store the data in Model when executing init function !

    // ------ FetchJs Actions ------
    MintTokenJSRes(Result<JsValue, JsValue>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::OnSelect => {
            let selection = "token_action".get_element();

            if let Ok(element) = selection {
                log!("select")
            } else {
                panic!("id with 'token_action' not found")
            }
        }

        Msg::OnChange(key, data) => match &mut model.actions {
            Action::Mint(form) => {
                *form.get_mut_field(&key) = data;
                form.check_input(&key);
            }
            _ => log!("no matched actions when onchange"),
        },
        Msg::OnSubmit(cluster, commitment) => match &mut model.actions {
            Action::Mint(form) => {
                if (model.mint != RemoteData::Loading || model.mint != RemoteData::Notasked)
                    && *form != MintForm::default()
                    && form.check_all_errors()
                // && *form.get
                {
                    model.mint = RemoteData::Loading;

                    let cluster = cluster.to_owned();
                    let commitment = commitment.to_owned();
                    let feepayer_seed = form.fee_payer_seed.to_owned();
                    let token_mint = form.token_mint.to_owned();
                    let destination = form.destination_account.to_owned();
                    let mint_authority_seed = form.mint_authority_seed.to_owned();
                    let amount = form.amount.to_owned();

                    //sicnce orders require lifetime be static, use clone beforehand
                    orders.skip().perform_cmd(async {
                        Msg::MintTokenJSRes(unsafe {
                            mint_token_js(
                                cluster,
                                commitment,
                                feepayer_seed,
                                token_mint,
                                destination,
                                mint_authority_seed,
                                amount,
                            )
                            .await
                        })
                    });
                } else {
                    log!("unfinished form")
                }
            }
            _ => log!("submit"),
        },
        _ => log!("oncahgne"),
    }
}
// ------ ------
//     view
// ------ ------

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    section![
        C!["section hero is-light is-fullheight-with-navbar"],
        attrs![At::Style => "padding-bottom: 10rem"],
        // Error7 Loading Message
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
        //Title
        view_title(&model.actions),
        //Result
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
        //Actions Selection
        //Form
        match &model.actions {
            Action::Mint(mint_form) => {
                view_mint_form(mint_form, ctx)
            }
            _ => empty(),
        }
    ]
}
pub fn view_title(action: &Action) -> Node<Msg> {
    let title = match action {
        Action::Mint(form) => "Mint To Account",
        Action::Transfer => "Transfer To Account",
        _ => "",
    };
    h2![
        C!["title is-2"],
        attrs! {At::Style => "text-align:center"},
        title
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

fn view_selection() -> Node<Msg> {
    ul![
        C!["filters"],
        li![a![C!["selected"], " mint",]],
        li![a![" transfer",]],
        li![a![" freeze",]],
        li![a![" burn",]]
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

// ------ ------
//     Forms
// ------ ------
fn view_mint_form(mint_form: &MintForm, ctx: &Context) -> Node<Msg> {
    let ui = mint_form.get_field_ui();
    //  inputs{
    // fee_payer_seed: String,
    // token_mint: String,
    // destination_account: String,
    // mint_authority_seed: String,
    // amount: String,
    //  }

    form![
        ev(Ev::Submit, move |event| {
            event.prevent_default();
        }),
        view_input(
            MINT_FORM_FEE_PAYER_SEED,
            "Fee Payer Seed",
            "text",
            ui[0].0,
            ui[0].1,
            ui[0].2,
            &mint_form.fee_payer_seed
        ),
        view_input(
            MINT_FORM_TOKEN_MINT,
            "Mint Account",
            "text",
            ui[1].0,
            ui[1].1,
            ui[1].2,
            &mint_form.token_mint
        ),
        view_input(
            MINT_FORM_DESTINATION_ACCOUNT,
            "Destination Account",
            "text",
            ui[2].0,
            ui[2].1,
            ui[2].2,
            &mint_form.destination_account
        ),
        view_input(
            MINT_FORM_MINT_AUTHORITY_SEED,
            "Mint Authority Seed",
            "text",
            ui[3].0,
            ui[3].1,
            ui[3].2,
            &mint_form.mint_authority_seed
        ),
        view_input(
            MINT_FORM_AMOUNT,
            "Mint Amount",
            "number",
            ui[4].0,
            ui[4].1,
            ui[4].2,
            &mint_form.amount
        ),
        view_button(ctx),
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
                "Mint"
            ]
        ]
    ]
}
// ------ ------
//     helper
// ------ ------
