use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
mod cards;
use crate::cards::{get_card, init_cards, Card};
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn get_card_image(card: &Card) -> Asset {
    match card.symbol.as_deref() {
        Some("cone") => asset!("/assets/cards/cone.jpg"),
        Some("fudge") => asset!("/assets/cards/fudge.jpg"),
        Some("gumdrop") => asset!("/assets/cards/gumdrop.jpg"),
        Some("lollipop") => asset!("/assets/cards/lollipop.jpg"),
        Some("peppermint") => asset!("/assets/cards/peppermint.jpg"),
        _ => unreachable!("should not be called with a color card"),
    }
}
fn main() {
    dioxus::launch(App);
}
#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    pub card: Card,
}
#[component]
fn CardBlock(card: Card) -> Element {
    if let Some(color) = card.color {
        if card.count == 2 {
            rsx! {
                div { class: format!("h-[34%] aspect-square m-5 {color}") }
                div { class: format!("h-[34%] aspect-square m-5 {color}") }
            }
        } else {
            rsx! {
                div { class: format!("h-[70%] aspect-square m-5 {color}") }
            }
        }
    } else if card.symbol.is_some() {
        rsx! {
            div { class: "overflow-hidden rounded-xl h-full w-full relative",
                img { class: "object-scale-down", src: get_card_image(&card) }
            }
        }
    } else {
        rsx! {
            div { class: "h-full w-full flex items-center justify-center", "get your first card" }
        }
    }
}
#[component]
fn App() -> Element {
    let mut cards = use_signal(init_cards);
    let mut card = use_signal(|| Card {
        color: None,
        count: 0,
        symbol: None,
    });
    let mut flash = use_signal(|| false);
    let next_card = move |_| {
        let (new_card, new_cards) = get_card(&cards.read());
        card.set(new_card);
        cards.set(new_cards);
        flash.set(true);
        spawn(async move {
            TimeoutFuture::new(40).await;
            flash.set(false);
        });
    };
    let shuffle_deck = move |_| {
        cards.set(init_cards());
        card.set(Card {
            color: None,
            count: 0,
            symbol: None,
        });
    };
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no",
        }
        div { class: "h-[100vh] overflow-hidden",
            h1 { class: "text-white mt-6 text-center text-4xl", "Candyland Roller" }
            div { class: "flex flex-col justify-center items-center h-screen text-white",
                button {
                    class: "p-6 text-xl mb-6 flex items-center justify-center bg-gray-800 rounded-xl hover:bg-gray-800/70 hover:border hover:border-2 hover:border-gray-700",
                    onclick: shuffle_deck,
                    "New Game"
                }
                div { class: "p-8 rounded",
                    button { class: "cards", onclick: next_card,
                        div { class: format!("h-[40vh] aspect-square flex items-center justify-center bg-gray-800 rounded-xl hover:bg-gray-800/70 hover:border hover:border-2 hover:border-gray-900 transition-opacity duration-[50] {}", if *flash.read() { "opacity-0" } else { "opacity-100" }),
                            CardBlock { card: card.read().clone() }
                        }
                    }
                }
            }
        }
    }
}
