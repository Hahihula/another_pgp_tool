use dioxus::prelude::*;
use pgp::{decrypt, encrypt, gen_key_pair, utils};

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
#[derive(Clone, PartialEq)]
struct Notification {
    message: String,
    notification_type: NotificationType,
    id: u32,
}

#[derive(Clone, PartialEq)]
enum NotificationType {
    Error,
    Success,
    Info,
}

static ACTIVETAB: GlobalSignal<ActiveTab> = Signal::global(|| ActiveTab::Generate);
static NOTIFICATIONS: GlobalSignal<Vec<Notification>> = Signal::global(Vec::new);
static NEXT_ID: GlobalSignal<u32> = Signal::global(|| 0);

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
            TabNavigation {}
            TabContent {}
            NotificationContainer {}
        }
    }
}

#[component]
fn NotificationContainer() -> Element {
    let notifications = NOTIFICATIONS.read();

    rsx! {
        div { class: "notification-container",
            for notification in notifications.iter() {
                Notification_item {
                    key: "{notification.id}",
                    message: notification.message.clone(),
                    notification_type: notification.notification_type.clone(),
                    id: notification.id
                }
            }
        }
    }
}

#[component]
fn Notification_item(message: String, notification_type: NotificationType, id: u32) -> Element {
    let notification_class = match notification_type {
        NotificationType::Error => "notification error",
        NotificationType::Success => "notification success",
        NotificationType::Info => "notification info",
    };

    let dismiss = move |_| {
        let mut notifications = NOTIFICATIONS.write();
        notifications.retain(|n| n.id != id);
    };

    rsx! {
        div { class: "{notification_class}",
            span { class: "notification-message", "{message}" }
            button {
                class: "notification-dismiss",
                onclick: dismiss,
                "×"
            }
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
        let (priv_key, pub_key) = match gen_key_pair("", "").await {
            Ok(res) => res,
            Err(e) => {
                show_message(
                    format!("Error generating keys: {}", e),
                    Some(NotificationType::Error),
                );
                return;
            }
        };
        private_key.set(match priv_key.to_armored_string(None) {
            Ok(s) => s,
            Err(e) => {
                show_message(
                    format!("Private key not valid string: {}", e),
                    Some(NotificationType::Error),
                );
                return;
            }
        });
        public_key.set(match pub_key.to_armored_string(None) {
            Ok(s) => s,
            Err(e) => {
                show_message(
                    format!("Public key not valid string: {}", e),
                    Some(NotificationType::Error),
                );
                return;
            }
        });
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
    let encrypted_message = use_signal(String::new);

    let encrypt_message = move |_| {
        to_owned![plain_message, recipient_public_key, encrypted_message];
        async move {
            let msg = plain_message.read().clone().as_bytes().to_vec();
            let skey = match utils::read_pkey_from_string(recipient_public_key.read().clone()).await
            {
                Ok(s) => s,
                Err(e) => {
                    show_message(
                        format!("Error reading public key: {}", e),
                        Some(NotificationType::Error),
                    );
                    return;
                }
            };
            let encrypted_msg = match encrypt(vec![skey], msg).await {
                Ok(s) => s,
                Err(e) => {
                    show_message(
                        format!("Error encrypting message: {}", e),
                        Some(NotificationType::Error),
                    );
                    return;
                }
            };

            encrypted_message.set(match String::from_utf8(encrypted_msg) {
                Ok(s) => s,
                Err(e) => {
                    show_message(
                        format!("Error converting encrypted message to string: {}", e),
                        Some(NotificationType::Error),
                    );
                    return;
                }
            });
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
    let mut private_key = use_signal(String::new);
    let mut encrypted_message = use_signal(String::new);
    let decrypted_message = use_signal(String::new);

    let decrypt_message = move |_| {
        to_owned![private_key, encrypted_message, decrypted_message];
        async move {
            let encrypted_data = encrypted_message.read().clone().as_bytes().to_vec();
            let skey = match utils::read_skey_from_string(private_key.read().clone()).await {
                Ok(s) => s,
                Err(e) => {
                    show_message(
                        format!("Error reading private key: {}", e),
                        Some(NotificationType::Error),
                    );
                    return;
                }
            };
            
            let decrypted_msg = match decrypt(skey, "", encrypted_data).await {
                Ok(s) => s,
                Err(e) => {
                    show_message(
                        format!("Error decrypting message: {}", e),
                        Some(NotificationType::Error),
                    );
                    return;
                }
            };

            decrypted_message.set(match String::from_utf8(decrypted_msg) {
                Ok(s) => s,
                Err(e) => {
                    show_message(
                        format!("Error converting decrypted message to string: {}", e),
                        Some(NotificationType::Error),
                    );
                    return;
                }
            });
        }
    };

    rsx! {
        div { class: "tab-panel",
            h2 { "Decrypt Message" }

            div { class: "form-group",
                label { "Your Private Key:" }
                textarea {
                    class: "key-textarea",
                    value: private_key.read().clone(),
                    oninput: move |evt| private_key.set(evt.value().clone()),
                    rows: 8,
                    cols: 50,
                    placeholder: "Paste your private key here..."
                }
            }

            div { class: "form-group",
                label { "Encrypted Message:" }
                textarea {
                    class: "message-textarea",
                    value: encrypted_message.read().clone(),
                    oninput: move |evt| encrypted_message.set(evt.value().clone()),
                    rows: 5,
                    cols: 50,
                    placeholder: "Paste the encrypted message here..."
                }
            }

            div { class: "form-group",
                button {
                    class: "decrypt-button",
                    onclick: decrypt_message,
                    "Decrypt Message"
                }
            }

            div { class: "form-group",
                label { "Decrypted Message:" }
                textarea {
                    class: "decrypted-textarea",
                    readonly: true,
                    value: decrypted_message.read().clone(),
                    rows: 8,
                    cols: 50
                }
            }
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

// utils:
fn show_message(message: String, message_type: Option<NotificationType>) {
    let mut notifications = NOTIFICATIONS.write();
    let mut id = NEXT_ID.write();
    let notification_type = match message_type {
        Some(t) => t,
        None => NotificationType::Info,
    };

    notifications.push(Notification {
        message,
        notification_type,
        id: *id,
    });

    *id += 1;
}
