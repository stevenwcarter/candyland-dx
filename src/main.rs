use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod cards;
use crate::cards::{Card, get_card, init_cards};

/// How many recent cards to keep in the bottom history trail.
const TRAIL_LEN: usize = 6;

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

/// CSS class that restarts the per-draw animations. The keyframes live in two
/// identically-defined `-a`/`-b` variants; alternating the class name on each
/// draw changes the `animation-name`, which is what forces the browser to replay
/// the animation even when the drawn card is identical to the previous one.
/// Sequence 0 (before the first draw) yields no animation.
fn anim_class(seq: u32) -> &'static str {
    match seq {
        0 => "",
        n if n % 2 == 1 => "a",
        _ => "b",
    }
}

/// Prepend the just-drawn card to the recent-history trail, capped at `TRAIL_LEN`
/// (newest first).
fn push_recent(hist: &mut Vec<(u32, Card)>, id: u32, card: Card) {
    hist.insert(0, (id, card));
    hist.truncate(TRAIL_LEN);
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

/// Render a miniature card face for the history trail.
fn mini_face(card: Card) -> AnyView {
    if let Some(color) = card.color {
        if card.count == 2 {
            view! {
                <div class=format!("msq small {color}")></div>
                <div class=format!("msq small {color}")></div>
            }
            .into_any()
        } else {
            view! { <div class=format!("msq {color}")></div> }.into_any()
        }
    } else {
        view! { <img class="mmini" src=card_image(&card) /> }.into_any()
    }
}

#[component]
fn App() -> impl IntoView {
    let deck = RwSignal::new(init_cards());
    let card = RwSignal::new(Card::empty());
    let recent = RwSignal::new(Vec::<(u32, Card)>::new());
    let seq = RwSignal::new(0u32);

    let next_card = move |_| {
        let (new_card, new_deck) = get_card(&deck.get_untracked());
        deck.set(new_deck);
        card.set(new_card.clone());
        seq.update(|s| *s += 1);
        let id = seq.get_untracked();
        recent.update(|h| push_recent(h, id, new_card));
    };

    let new_game = move |_| {
        deck.set(init_cards());
        card.set(Card::empty());
        recent.set(Vec::new());
    };

    view! {
        <div class="h-[100vh] overflow-hidden flex flex-col text-white">
            <h1 class="mt-6 text-center text-4xl">"Candyland Roller"</h1>
            <div class="flex-1 flex flex-col justify-center items-center">
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
                                "h-[40vh] aspect-square flex items-center justify-center bg-gray-800 rounded-xl hover:bg-gray-800/70 hover:border hover:border-2 hover:border-gray-900 card-flip {}",
                                anim_class(seq.get()),
                            )
                        }>{move || card_face(card.get())}</div>
                    </button>
                </div>
            </div>
            <div class="trail">
                <span class="trail-label">"recent"</span>
                <div class=move || format!("minis {}", anim_class(seq.get()))>
                    <For each=move || recent.get() key=|(id, _)| *id let:item>
                        <div class="mini">{mini_face(item.1)}</div>
                    </For>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anim_class_is_empty_before_first_draw_then_alternates() {
        assert_eq!(anim_class(0), "");
        assert_eq!(anim_class(1), "a");
        assert_eq!(anim_class(2), "b");
        assert_eq!(anim_class(3), "a");
    }

    #[test]
    fn push_recent_prepends_newest_and_caps_length() {
        let mut hist: Vec<(u32, Card)> = Vec::new();
        for id in 1..=(TRAIL_LEN as u32 + 2) {
            push_recent(&mut hist, id, Card::empty());
        }
        assert_eq!(hist.len(), TRAIL_LEN);
        assert_eq!(hist.first().unwrap().0, TRAIL_LEN as u32 + 2, "newest is first");
        assert_eq!(hist.last().unwrap().0, 3, "oldest kept is id 3");
    }
}
