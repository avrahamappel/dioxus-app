use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        DogApp { breed: "mutt" }
    }
}

#[component]
fn DogApp(breed: String) -> Element {
    rsx! { "Breed: {breed}" }
}
