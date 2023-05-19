use dioxus::prelude::*;
pub mod style;
pub mod projects;

pub fn Carousel(cx: Scope) -> Element {
    let mut carousel_page = use_state(cx, || 0);

    cx.render(rsx! {
        div { id: "carousel",
            style: style::CAROUSEL,
            div { 
                style: style::CAROUSEL_INNER,
                class: "slide{carousel_page}",
                
                (0..9).map(|i| {
                    let pro = projects::GET_PROJECT(i);
                    rsx! {                  
                    div {
                        style: style::CAROUSEL_ITEM,
                        div {
                            
                            style: style::CAROUSEL_CONTENT,
                            h1 {
                                pro.name
                            }
                            
                            p {
                                "image to go here"
                            }
                            p {
                                pro.description
                            }
                        }
                    }
                }})
            }
        }
        
        div {
            style: style::BTN_LEFT,
            class: "carousel_button",
            onclick: move |_| {
                if *carousel_page.get() > 0 {
                    carousel_page -= 1;
                } else {
                    carousel_page.set(8)
                }

            },
            img {
                style: style::BTN_ARROW,
                src: "static/images/left.svg"
            }
        }
        div {
            style: style::BTN_RIGHT,
            class: "carousel_button",
            onclick: move |_| {
                if *carousel_page.get() < 8 {
                    carousel_page += 1;
                } else {
                    carousel_page.set(0)
                }

            },
            img {
                style: style::BTN_ARROW,
                src: "static/images/right.svg"
            }
        }
    })
}
