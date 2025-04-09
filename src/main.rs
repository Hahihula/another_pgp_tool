
use dioxus::prelude::*;
use pgp::{encrypt, utils, gen_key_pair};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Copy, PartialEq)]
enum ActiveTab {
    Generate,
    Encrypt,
    Decrypt,
    Sign,
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
            button {
                class: if *active_tab == ActiveTab::Sign { "tab-button active" } else { "tab-button" },
                onclick: move |_| *ACTIVETAB.write() = ActiveTab::Sign,
                "Sign Message"
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
                ActiveTab::Sign => rsx! { SignMessageTab {} },
            }
        }
    }
}

#[component]
fn GenerateKeysTab() -> Element {
    let mut private_key = use_signal(String::new);
    let mut public_key = use_signal(String::new);
    
    let generate_keys = move |_| async move {
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
    let mut recipient_public_key = use_signal(String::new);
    let mut plain_message = use_signal(String::new);
    let mut encrypted_message = use_signal(String::new);
    
    let encrypt_message = move |_| {
        to_owned![plain_message, recipient_public_key, encrypted_message];
        async move {
        let msg = plain_message.read().clone().as_bytes().to_vec();
        let skey = utils::read_pkey_from_string(recipient_public_key.read().clone()).await.unwrap();
        let encrypted_msg = encrypt(vec![skey], msg)
            .await
            .unwrap();
        encrypted_message.set(String::from_utf8(encrypted_msg).unwrap());
        
        }
    };
    
    rsx! {
        div { class: "tab-panel",
            h2 { "Encrypt Message" }
            
            div { class: "form-group",
                label { "Recipient's Public Key:" }
                textarea {
                    class: "key-textarea",
                    value: recipient_public_key.read().clone(),
                    oninput: move |evt| recipient_public_key.set(evt.value().clone()),
                    rows: 8,
                    cols: 50,
                    placeholder: "Paste recipient's public key here..."
                }
            }
            
            div { class: "form-group",
                label { "Message to Encrypt:" }
                textarea {
                    class: "message-textarea",
                    value: plain_message.read().clone(),
                    oninput: move |evt| plain_message.set(evt.value().clone()),
                    rows: 5,
                    cols: 50,
                    placeholder: "Type your message here..."
                }
            }
            
            div { class: "form-group",
                button {
                    class: "encrypt-button",
                    onclick: encrypt_message,
                    "Encrypt Message"
                }
            }
            
            div { class: "form-group",
                label { "Encrypted Message:" }
                textarea {
                    class: "encrypted-textarea",
                    readonly: true,
                    value: encrypted_message.read().clone(),
                    rows: 8,
                    cols: 50
                }
            }
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

#[component]
fn SignMessageTab() -> Element {
    rsx! {
        div { class: "tab-panel",
            h2 { "Sign Message" }
            p { "Sign a message using your private key." }
            // Form elements would go here
        }
    }
}