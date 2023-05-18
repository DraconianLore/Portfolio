use dioxus::prelude::*;
pub mod style;
pub mod projects;

pub fn Carousel(cx: Scope) -> Element {
    cx.render(rsx! {
        div { id: "carousel",
            style: style::CAROUSEL,
            div { 
                style: style::CAROUSEL_INNER,
                
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
    })
}

