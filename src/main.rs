#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod hackernews {
    #![allow(warnings)]
    include!("../hackernews/src/main.rs");
}
use hackernews::Hackernews;

fn main() {
    launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList {},
                #[route("/post/:name")]
                BlogPost { name: String },
            #[end_layout]
        #[end_nest]
        #[route("/hackernews")]
        Hackernews,
    #[end_layout]
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar",
            // img { src: "profile_pic.png", alt: "", width: "50px" }
            ul { class: "navlist",
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::Hackernews {}, "Hackernews" }
                }
                li {
                    Link { to: Route::BlogList {}, "Blog" }
                }   
                li {
                    Link { to: Route::Home {}, "Resume" }
                }
                li {
                    Link { to: "https://github.com/JerryAchrich/", i { class: "fa fa-github" } }
                }
                li {
                    Link { to: "https://www.linkedin.com/in/jerry-achrich/", i { class: "fa fa-linkedin-square" } }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { h1 { text_align: "center", "Jerry Achrich" } }
}

#[component]
fn Blog() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[component]
fn BlogList() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[component]
fn BlogPost(name: String) -> Element {
    rsx! { h2 { "Blog Post: {name}" } }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}