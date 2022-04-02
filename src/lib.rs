#![allow(clippy::wildcard_imports)]
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};
use serde::Deserialize;

pub mod page {
    pub mod escrow;
    pub mod home;
    pub mod notfound;
    pub mod token;
}

// ------ ------
//     Init
// ------ ------

//being triggered when url changed as well
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // 1st async: perfrom_cmd expect "future argument"
    // 2nd async: MSg expect "Fetch::Result"
    // 3rd async: for executing the json file, right after fetch() method
    orders
        .subscribe(Msg::UrlChanged)
        //listen all the click on the window to close down the menu
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu))
        //orders.""perform_cmd"" expect "Future" as argument, then executing by converting to "Promise" in JS
        .perform_cmd(async {
            Msg::FetchJson(
                async {
                    fetch("/assets/config.json")
                        .await?
                        .check_status()?
                        .json()
                        .await
                }
                .await,
            ) //wrap the message in "fetch::Result"
        });

    Model {
        ctx: Context {
            cluster: None,
            commitment: None,
            user: None,
        },

        base_url: url.to_base_url(),
        page: Page::init(url, orders),
        menu_visible: false,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,

    menu_visible: bool,
}

pub struct Context {
    pub cluster: Option<String>,
    pub commitment: Option<String>,

    pub user: Option<User>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pubkey: String,
    lamports: String,
}

// ------ ------ ------
//      Model_Url
// ------ ------ ------

struct_urls!(); //useful for directly accessing the "route path"
                //Ex: filter button to modify Url to trigger the event::UrlChanged -> Msg
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }

    fn escrow(self) -> Url {
        self.base_url().add_path_part(ESCROW)
    }
    fn token(self) -> Url {
        self.base_url().add_path_part(TOKEN)
    }

    fn token_pages(self) -> page::token::Urls<'a> {
        page::token::Urls::new(self.base_url().add_path_part(TOKEN))
    }
}

// ------ ------ ------
//      Model_Page
// ------ ------ ------

const ESCROW: &str = "escrow";
const TOKEN: &str = "token";

//enum page simply is combinations of init function
enum Page {
    Home,
    //type annotation => saying that this Escrow only accpet the Model correpsonding to the Page
    Escrow(page::escrow::Model),
    Token(page::token::Model),
    NotFound,
}

// ------ ------ ------ ------
//      Model_Page_Router
// ------ ------ ------ ------

//Been called and init the model, when "UrlChanged" event occurs !
//therefore, this pattern will not store page's state
impl Page {
    //  "redirect" the Msg to the associated pages (to their update function)
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        //use next_path_part for creating over 2 level routing path
        match url.next_path_part() {
            None => Self::Home,
            //What we did here:
            //  1. to map 'Orders<ChildMs> to Orders<Msg>',
            //  2. execute init -> build up the Model
            //Beuase Msg<Root> not matched Msg<Child>
            Some(ESCROW) => Self::Escrow(page::escrow::init(url, &mut orders.proxy(Msg::Escrow))),

            Some(TOKEN) => Self::Token(page::token::init(url, &mut orders.proxy(Msg::Token))),

            Some(_) => Self::NotFound,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    domain: String,
    commitment: String,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    //fetch::Result<T,FetchError>
    //Currently we use located file, further option like connnecting to wallet will be handled somewhen
    FetchJson(fetch::Result<Connection>),
    UserInitialized(Result<JsValue, JsValue>),

    //subscribe
    UrlChanged(subs::UrlChanged),

    //Menu UI
    ToggleMenu,
    HideMenu,

    // ------ pages ------
    //type annotation: Each page should only accept corresponding Msg
    Escrow(page::escrow::Msg),
    Token(page::token::Msg),
}

// `update` describes how to handle each `Msg`
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // ------ init ------
        Msg::FetchJson(Ok(connection)) => {
            log!(connection);
            //read keypair file and update the model
            let cluster = connection.domain.clone();
            let commitment = connection.commitment.clone();

            model.ctx.cluster = Some(connection.domain);
            model.ctx.commitment = Some(connection.commitment);

            orders.skip().perform_cmd(async {
                Msg::UserInitialized(unsafe { get_account_js(cluster, commitment).await })
                //external API
            });
        }
        Msg::FetchJson(Err(fetch_error)) => {
            error!("Keypair.json fetch failed!", fetch_error)
        }
        Msg::UserInitialized(Ok(user)) => {
            //fetching our customized API in index.js
            if not(user.is_undefined()) {
                //transfer JS object to Rust Object
                match serde_wasm_bindgen::from_value(user) {
                    Ok(user) => {
                        log!(user);
                        model.ctx.user = Some(user);
                    }
                    Err(error) => error!("Block deserialization failed!", error),
                }
            }

            let search = model.base_url.search_mut();
            if search.remove("code").is_some() && search.remove("state").is_some() {
                model.base_url.go_and_replace();
            }
        }
        Msg::UserInitialized(Err(error)) => {
            error!("User initialization failed!", error);
        }

        // ------ URL ------
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url, orders);
        }

        // ------ UI ------
        Msg::ToggleMenu => {
            model.menu_visible = not(model.menu_visible);
        }
        Msg::HideMenu => {
            if model.menu_visible {
                model.menu_visible = false;
            } else {
                orders.skip(); //no need of re-rendering
            }
        }

        // ------ Page ------

        // --- msg ---, "update function", since each update function represent Msg
        // --- model ---, our current state (Root Model) locating in the root model, acts like borrowing
        // --- &mut impl Orders<Msg> ---, borrowing "orders" to the child element
        Msg::Escrow(msg) => {
            if let Page::Escrow(model) = &mut model.page {
                //executing update function in corresponding page
                //;but Msg::Token should be wrapped with 'orders.proxy' which we had defined when calling init functino
                page::escrow::update(msg, model, &mut orders.proxy(Msg::Escrow))
            }
        }
        Msg::Token(msg) => {
            if let Page::Token(model) = &mut model.page {
                page::token::update(msg, model, &mut orders.proxy(Msg::Token))
            }
        }
    }
}
// ------ ------
//     Wasm
// ------ ------

//make sure all the function "siganature and argument" is identical, otherwise it can not be catched up !

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn get_account_js(domain: String, commitment: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn create_mint_js(
        domain: String,
        commitment: String,
        payer_secret: String,
        mint_authority_address: String,
        freeze_authority_address: String,
        token_decimals: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn create_token_account_js(
        cluster: String,
        commitment: String,
        feepayer_seed: String,
        token_mint: String,
        owner: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn mint_token_js(
        cluster: String,
        commitment: String,
        feepayer_seed: String,
        token_mint: String,
        destination: String,
        mint_authority_seed: String,
        amount: String,
    ) -> Result<JsValue, JsValue>;

}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_navbar(model.menu_visible, &model.base_url, model.ctx.user.as_ref()),
        view_content(&model),
        // view_content(&model.page), //Be automatically rendered, since we just change the mutable state,
    ]
}

// ----- view_navbar ------
fn view_navbar(menu_visible: bool, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    nav![
        C!["navbar"],
        div![
            C!["container"],
            view_navbrand(menu_visible, base_url),
            view_nav_menu(user, menu_visible, base_url)
        ]
    ]
}

fn view_navbrand(menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-brand"],
        // ------ Logo ------
        a![
            C!["navbar-item "],
            attrs! [At::Href => Urls::new(base_url).home()],
            img![
                attrs! {At::Src =>"/static/solana-sol-logo-horizontal.svg",At::Width => "112",At::Height => "28"}
            ]
        ],
        // ------ Hamburger ------
        a![
            C!["navbar-burger", IF!(menu_visible => "is-active")],
            attrs![At::from("role")=>"button",At::AriaLabel=>"menu",At::AriaExpanded => menu_visible,At::from("data-target") => "navbarBasicExample"],
            //event
            ev(Ev::Click, |event| {
                event.stop_propagation();
                Msg::ToggleMenu
            }),
            span![attrs! {At::AriaHidden => "true"}],
            span![attrs! {At::AriaHidden => "true"}],
            span![attrs! {At::AriaHidden => "true"}]
        ]
    ]
}
fn view_nav_menu(user: Option<&User>, menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-menu", IF!(menu_visible => "is-active")],
        attrs![At::Id => "navMenu"],
        view_navbar_start(base_url),
        view_navbar_end(user),
    ]
}

// ------ Route-Link ------
fn view_navbar_start(base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-start"],
        // ------ Route_Home ------
        a![
            C!["navbar-item"],
            attrs! {At::Href => Urls::new(base_url).home()},
            "Home"
        ],
        // ------ Route_Escrow ------
        a![
            C!["navbar-item"],
            attrs! {At::Href => Urls::new(base_url).escrow()},
            "Escrow"
        ],
        div![
            C!["navbar-item has-dropdown is-hoverable"],
            a![C!["navbar-link"], TOKEN],
            div![
                C!["navbar-dropdown"],
                // ------ Route_Token ------
                a![
                    C!["navbar-item"],
                    attrs! {
                        //initialize base_url of children page
                        At::Href => Urls::new(base_url).token();
                    },
                    "Create Token"
                ],
                a![
                    C!["navbar-item"],
                    attrs! {
                        At::Href => Urls::new(base_url).token_pages().edit_token();
                    },
                    "Edit Token"
                ],
                hr![C!["navbar-divider"]],
                a![
                    C!["navbar-item"],
                    attrs! {
                        At::Href => Urls::new(base_url).token_pages().create_account();
                    },
                    "Create Account"
                ],
                a![C!["navbar-item"], "Edit Account"],
            ]
        ]
    ]
}

fn view_navbar_end(user: Option<&User>) -> Node<Msg> {
    div![
        C!["navbar-end"],
        div![
            C!["navbar-item"],
            div![
                C!["buttons"],
                a![
                    C!["button is-light"],
                    "Github",
                    img![attrs! {At::Src => "/static/Github-Mark-32px.png",
                        At::Width => "20",
                        At::Height => "20",
                        At::Style => "margin-left:10px"
                    }]
                ],
                a![
                    C!["button is-black"],
                    attrs! {At::Href => ""},
                    if let Some(user) = user {
                        &user.pubkey
                    } else {
                        "Connect"
                    }
                ]
            ]
        ]
    ]
}

// ------ ------ ------
//     View_Pages
// ------ ------ ------

//page display, lifting our page component up
//passing 'root model' is disallowed while you could send useful variables ex: 'base_url '
fn view_content(model: &Model) -> Node<Msg> {
    div![match &model.page {
        //checking current "page enum" type
        Page::Home => page::home::view(),
        //when lifting the children's view up to the Root view function, we had to mapping the Msg type from
        // "Msg::Escrow" to "Msg::Root"
        //Simply saying: Node<Msg<Child>> ->  Node<Msg>
        Page::Escrow(escrow_model) => page::escrow::view(escrow_model)
            .map_msg(Msg::Escrow /*this trigger update(root ) above */),
        Page::Token(token_model) => page::token::view(token_model, &model.ctx).map_msg(Msg::Token),
        Page::NotFound => page::notfound::view(),
    }]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

// ------ ------ ------
//     External API
// ------ ------ ------
