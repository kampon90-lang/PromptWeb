use dioxus::{html::{div, g::primitive_units, label}, prelude::*};
use dioxus_free_icons::icons::fa_solid_icons::{FaQrcode, FaBahtSign, FaIdCard, FaLock, FaBolt};
use dioxus_free_icons::Icon;

 mod promptpay;
 mod qrcode_generate;
 use promptpay::promptpay_payload;
use crate::qrcode_generate::{generate_qr_base64_png, generate_qr_base64_svg};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Hero{},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    // Create signal
    let mut error_msg = use_signal(|| String::new());
    let mut pp_id = use_signal(|| "".to_string());
    let mut amount = use_signal::<Option<f64>>(|| None);
    let mut qrcode = use_signal(|| String::new());
    
    // Read signal
    let pp_id_display = pp_id.peek().clone();
    let qrcode_display = qrcode.peek().clone();
    let amount_display = (*amount.peek()).map(|v| v.to_string()).unwrap_or_default();
    let has_qr = !qrcode_display.is_empty();
        
    rsx! {
        div {
            id: "hero",
            class: "flex items-center justify-center p-4 min-h-screen bg-[#2d264f]",
            div { class: "max-w-md w-full",
                // Glass Card
                div { class: "glass-card rounded-[2.5rem] shadow-2xl p-8 space-y-6 bg-[#1e1b2e]/90 backdrop-blur-xl border border-white/10",

                    // Header Section
                    div { class: "text-center space-y-3",
                        div { class: "inline-flex items-center justify-center w-16 h-16 bg-cyan-500 rounded-2xl mb-2 shadow-lg shadow-cyan-500/20",
                            i { class: "fa-solid fa-qrcode text-3xl text-white",
                                Icon {
                                    icon: FaQrcode,
                                    width: 30,
                                    height: 30,
                                    fill: "white",
                                }
                            
                            }
                        }
                        //p { class: "text-white text-xs", "len = {qrcode.read().len()}" }
                        h1 { class: "text-2xl font-bold text-white tracking-tight",
                            "PromptPay QR Code Generator"
                        }
                        p { class: "text-slate-400 text-sm", "Create receiving money QR here" }
                    }

                    // QR Display Box (adjust to fit)
                    div { class: "relative bg-white/5 rounded-3xl p-6 border border-white/10 flex flex-col items-center justify-center min-h-[200px]",
                        if has_qr {
                            img {
                                class: "w-44 h-44 bg-white p-2 rounded-xl",
                                alt: "QR Code",
                                src: "{qrcode_display}",
                            }
                        } else {
                            div {
                                id: "qrcode-container",
                                class: "flex flex-col items-center justify-center space-y-3",
                                // Placeholder icon
                                Icon {
                                    icon: FaQrcode,
                                    width: 40,
                                    height: 40,
                                    fill: "#334155",
                                }
                                // Placeholder text
                                span { class: "text-slate-500 text-xs", "QR code will be here" }
                            }
                        }
                    
                    }

                    // Form Fields
                    div { class: "space-y-5",
                        // Receiver ID
                        div { class: "space-y-2",
                            label { class: "text-xs font-semibold text-cyan-400 uppercase tracking-wider ml-1",
                                "Receiver information"
                            }
                            div { class: "relative group",
                                div { class: "absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none",
                                    i { class: "fa-solid fa-id-card text-slate-500 group-focus-within:text-cyan-500 transition-colors",
                                        Icon {
                                            icon: FaIdCard,
                                            width: 15,
                                            height: 15,
                                            fill: "white",
                                        }
                                    }
                                }

                                input {
                                    class: "w-full bg-slate-900/50 border border-slate-700 text-white text-sm rounded-xl py-4 pl-11 pr-4 focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all placeholder:text-slate-600",
                                    r#type: "number",
                                    value: "{pp_id_display}",
                                    oninput: move |e| {
                                        let val = e.value();
                                        spawn(async move {
                                            pp_id.set(val);
                                        });
                                    },
                                    placeholder: "PromptPay ID (Phone, ID, or E-wallet)",
                                }
                            }
                        }

                        // Amount
                        div { class: "space-y-2",
                            label { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider ml-1",
                                "Amount <optional>"
                            }
                            div { class: "relative group",
                                div { class: "absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none",
                                    i { class: "fa-solid fa-baht-sign text-slate-500 group-focus-within:text-cyan-400",
                                        Icon {
                                            icon: FaBahtSign,
                                            width: 15,
                                            height: 15,
                                            fill: "white",
                                        }
                                    }
                                }
                                input {
                                    class: "w-full bg-slate-900/50 border border-slate-700 text-white text-sm rounded-xl py-4 pl-11 pr-4 focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all placeholder:text-slate-600",
                                    r#type: "number",
                                    // Convert Option<f64> to String
                                    value: {amount_display},
                                    oninput: move |e| {
                                        let val = e.value();
                                        spawn(async move {
                                            if val.trim().is_empty() {
                                                amount.set(None);
                                            } else if let Ok(n) = val.parse::<f64>() {
                                                amount.set(Some(n));
                                            }
                                        });
                                    },
                                    placeholder: "0.00",
                                }
                            }
                        }
                    }

                    // Submit Button
                    button {
                        id: "btnSubmit",
                        class: "curosor-pointer transform-gpu relative z-50 w-full bg-cyan-500 hover:bg-cyan-400 text-[#1e1b2e] font-bold py-4 rounded-xl shadow-lg shadow-cyan-500/20 active:scale-[0.98] transition-all flex items-center justify-center space-x-2",
                        "style": "touch-action: manipulation; -webkit-tap-highlight-color: transparent;",
                        //r#type: "button",
                        onclick: move |evt| {
                            evt.stop_propagation();

                            // use peek
                            let id_val = pp_id.peek().clone();
                            let amt_val = *amount.peek();

                            spawn(async move {
                                // macth result
                                match promptpay_payload(&id_val, amt_val) {
                                    Ok(payload) => {
                                        let svg = generate_qr_base64_svg(&payload);
                                        qrcode.set(svg);

                                        pp_id.set("".to_string());
                                        amount.set(None);
                                        error_msg.set("".to_string());

                                    }
                                    Err(e) => {
                                        error_msg.set(e);
                                        qrcode.set("".to_string());
                                    }
                                }
                            });
                        },
                        span { "Create/Update QR" }
                        i { class: "fa-solid fa-bolt",
                            Icon {
                                icon: FaBolt,
                                width: 15,
                                height: 15,
                                fill: "#1e1b2e",
                            }
                        }
                    }

                    if !error_msg.read().is_empty() {
                        p { class: "text-red-400 text-xs text-center mt-2", "{error_msg}" }
                    }

                    // Footer
                    // --- Footer ---
                    div { class: "flex items-center justify-center space-x-2 mt-6",
                        Icon {
                            icon: FaLock,
                            width: 12,
                            height: 12,
                            fill: "#64748b",
                        }
                        p { class: "text-slate-500 text-[11px]",
                            "Secure and support all PromptPay Banks"
                        }
                    }
                }
            }
        }
    }
}