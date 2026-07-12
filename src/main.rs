use leptos::mount::mount_to_body;
use leptos::prelude::*;
use std::time::Duration;

mod cards;
use crate::cards::{Card, get_card, init_cards};

/// Static URL for a symbol card's image. Card images are shipped by Trunk from
/// `assets/cards/` (see `index.html`) and served at `/assets/cards/<name>.jpg`.
fn card_image(card: &Card) -> &'static str {
    match card.symbol.as_deref() {
        Some("cone") => "/assets/cards/cone.jpg",
        Some("fudge") => "/assets/cards/fudge.jpg",
        Some("gumdrop") => "/assets/cards/gumdrop.jpg",
        Some("lollipop") => "/assets/cards/lollipop.jpg",
        Some("peppermint") => "/assets/cards/peppermint.jpg",
        _ => unreachable!("should not be called with a color card"),
    }
}

/// Render the face of a single card: one/two color squares, a symbol image, or
/// the starting placeholder.
fn card_face(card: Card) -> AnyView {
    if let Some(color) = card.color {
        if card.count == 2 {
            view! {
                <div class=format!("h-[34%] aspect-square m-5 {color}")></div>
                <div class=format!("h-[34%] aspect-square m-5 {color}")></div>
            }
            .into_any()
        } else {
            view! { <div class=format!("h-[70%] aspect-square m-5 {color}")></div> }.into_any()
        }
    } else if card.symbol.is_some() {
        view! {
            <div class="overflow-hidden rounded-xl h-full w-full relative">
                <img class="object-scale-down" src=card_image(&card) />
            </div>
        }
        .into_any()
    } else {
        view! { <div class="h-full w-full flex items-center justify-center">"get your first card"</div> }
        .into_any()
    }
}

#[component]
fn App() -> impl IntoView {
    let cards = RwSignal::new(init_cards());
    let card = RwSignal::new(Card::empty());
    let flash = RwSignal::new(false);

    let next_card = move |_| {
        let (new_card, new_cards) = get_card(&cards.get_untracked());
        card.set(new_card);
        cards.set(new_cards);
        flash.set(true);
        set_timeout(move || flash.set(false), Duration::from_millis(40));
    };

    let new_game = move |_| {
        cards.set(init_cards());
        card.set(Card::empty());
    };

    view! {
        <div class="h-[100vh] overflow-hidden">
            <h1 class="text-white mt-6 text-center text-4xl">"Candyland Roller"</h1>
            <div class="flex flex-col justify-center items-center h-screen text-white">
                <button
                    class="p-6 text-xl mb-6 flex items-center justify-center bg-gray-800 rounded-xl hover:bg-gray-800/70 hover:border hover:border-2 hover:border-gray-700"
                    on:click=new_game
                >
                    "New Game"
                </button>
                <div class="p-8 rounded">
                    <button class="cards" on:click=next_card>
                        <div class=move || {
                            format!(
                                "h-[40vh] aspect-square flex items-center justify-center bg-gray-800 rounded-xl hover:bg-gray-800/70 hover:border hover:border-2 hover:border-gray-900 transition-opacity duration-[50] {}",
                                if flash.get() { "opacity-0" } else { "opacity-100" },
                            )
                        }>{move || card_face(card.get())}</div>
                    </button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
