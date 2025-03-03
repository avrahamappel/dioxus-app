use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        Router::<Route> {}
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::DogView,
                h1 { "🌭 HotDog!" }
            }
            Link { to: Route::Favorites, id: "heart", "♥️" }
        }
        Outlet::<Route> {}
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let mut dog_img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let skip = move |_| {
        dog_img_src.restart();
    };
    let save = move |_| async move {
        let image = dog_img_src.cloned().unwrap();
        dog_img_src.restart();
        save_dog(image).await;
    };

    rsx! {
        div { id: "dogview",
            img { src: dog_img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}

#[component]
fn Favorites() -> Element {
    let dogs = use_resource(get_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (idx, dog) in dogs().unwrap() {
                    img { key: idx, class: "favorite-dog", src: "{dog}" }
                }
            }
        }
    }
}

// Server functions

#[server]
async fn get_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let file = std::fs::read_to_string("dogs.txt").unwrap();
    let dogs = file.lines().map(|s| s.to_string()).enumerate().collect();
    Ok(dogs)
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}
