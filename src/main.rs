use dioxus::prelude::*;
use pgp::{decrypt, encrypt, gen_key_pair, native::ser::Serialize};

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
    let mut private_key = use_signal(String::new);
    let mut public_key = use_signal(String::new);
    
    let generate_keys = move |_| async move {
        // In a real implementation, you would generate actual PGP keys here
        // For now, we'll just set some placeholder text
        // private_key.set("-----BEGIN PGP PRIVATE KEY BLOCK-----\n[Private key would appear here]\n-----END PGP PRIVATE KEY BLOCK-----".to_string());
        // public_key.set("-----BEGIN PGP PUBLIC KEY BLOCK-----\n[Public key would appear here]\n-----END PGP PUBLIC KEY BLOCK-----".to_string());
        let (priv_key, pub_key) = gen_key_pair("","").await.unwrap();
        private_key.set(priv_key.to_armored_string(None).unwrap());
        public_key.set(pub_key.to_armored_string(None).unwrap());
    };
    
    rsx! {
        div { class: "tab-panel",
            h2 { "Generate PGP Keys" }
            
            div { class: "form-group",
                button {
                    class: "generate-button",
                    onclick: generate_keys,
                    "Generate Keys"
                }
            }
            
            div { class: "keys-container",
                div { class: "key-section",
                    label { "Private Key:" }
                    textarea {
                        class: "key-textarea",
                        readonly: true,
                        value: private_key.read().clone(),
                        rows: 10,
                        cols: 50
                    }
                }
                
                div { class: "key-section",
                    label { "Public Key:" }
                    textarea {
                        class: "key-textarea",
                        readonly: true,
                        value: public_key.read().clone(),
                        rows: 10,
                        cols: 50
                    }
                }
            }
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
