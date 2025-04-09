use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// State for tracking active tab
#[derive(Clone, Copy, PartialEq)]
enum ActiveTab {
    Generate,
    Encrypt,
    Decrypt,
    Verify,
}

static ACTIVETAB: GlobalSignal<ActiveTab> = Signal::global(|| ActiveTab::Generate);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-container",
            Header {}
            TabNavigation {  }
            TabContent { }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            h1 { "Yet Another PGP Tool" }
        }
    }
}

#[component]
fn TabNavigation() -> Element {
    // let set_active = move |tab: ActiveTab| {
    //     active_tab.set(tab);
    // };
    
    // let tab_generate = move |evt| {};
    // let tabe_encrypt = move |evt| {};
    // let tab_decrypt = move |evt| {};
    // let tab_verify = move |evt| {};
    
    // let active_tab = use_hook(|| ActiveTab::Generate);
    // let active_tab = use_signal(|| ActiveTab::Generate);
    let active_tab = &ACTIVETAB;


    rsx! {
        nav { class: "tab-navigation",
            button {
                class: if *active_tab == ActiveTab::Generate { "tab-button active" } else { "tab-button" },
                onclick: move |_| *ACTIVETAB.write() = ActiveTab::Generate,
                "Generate Keys"
            }
            button {
                class: if *active_tab == ActiveTab::Encrypt { "tab-button active" } else { "tab-button" },
                onclick: move |_| *ACTIVETAB.write() = ActiveTab::Encrypt,
                "Encrypt Message"
            }
            button {
                class: if *active_tab == ActiveTab::Decrypt { "tab-button active" } else { "tab-button" },
                onclick: move |_| *ACTIVETAB.write() = ActiveTab::Decrypt,
                "Decrypt Message"
            }
            button {
                class: if *active_tab == ActiveTab::Verify { "tab-button active" } else { "tab-button" },
                onclick: move |_| *ACTIVETAB.write() = ActiveTab::Verify,
                "Verify Message"
            }
        }
    }
}

#[component]
fn TabContent() -> Element {
    let active_tab = &ACTIVETAB;
    rsx! {
        div { class: "tab-content",
            match *active_tab.read() {
                ActiveTab::Generate => rsx! { GenerateKeysTab {} },
                ActiveTab::Encrypt => rsx! { EncryptMessageTab {} },
                ActiveTab::Decrypt => rsx! { DecryptMessageTab {} },
                ActiveTab::Verify => rsx! { VerifyMessageTab {} },
            }
        }
    }
}

#[component]
fn GenerateKeysTab() -> Element {
    rsx! {
        div { class: "tab-panel",
            h2 { "Generate PGP Keys" }
            p { "Generate your public and private key pair here." }
            // Form elements would go here
        }
    }
}

#[component]
fn EncryptMessageTab() -> Element {
    rsx! {
        div { class: "tab-panel",
            h2 { "Encrypt Message" }
            p { "Encrypt a message using a public key." }
            // Form elements would go here
        }
    }
}

#[component]
fn DecryptMessageTab() -> Element {
    rsx! {
        div { class: "tab-panel",
            h2 { "Decrypt Message" }
            p { "Decrypt a message using your private key." }
            // Form elements would go here
        }
    }
}

#[component]
fn VerifyMessageTab() -> Element {
    rsx! {
        div { class: "tab-panel",
            h2 { "Verify Message" }
            p { "Verify a signed message using a public key." }
            // Form elements would go here
        }
    }
}
