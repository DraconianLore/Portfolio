use dioxus::prelude::*;
pub mod style;
pub mod projects;


pub fn Carousel(cx: Scope) -> Element {
    let mut carousel_page = use_state(cx, || 0);
    let number_of_projects = 7; //MUST BE UPDATED WHEN ADDING NEW PROJECTS
    cx.render(rsx! {
        div { id: "carousel",
            style: style::CAROUSEL,
            div { 
                style: style::CAROUSEL_INNER,
                class: "slide{carousel_page}",
                
                (0..number_of_projects).map(|i| {
                    let pro = projects::GET_PROJECT(i);
                    // Video checking
                    let filename = pro.image.to_owned();
                    let fileType = filename[filename.len() -4..].to_string();
                    let live = pro.live.to_owned();
                    let has_live = live.len() > 0;

                    rsx! {                  
                    div {
                        style: style::CAROUSEL_ITEM,
                        div {
                            style: style::CAROUSEL_CONTENT,
                            div {
                                style: style::CAROUSEL_HEADER,
                                div {
                                    style: "width: 110px;",
                                    a {
                                    href: "{pro.github.to_owned()}",
                                    style: style::GITHUB,
                                    target: "_blank",
                                    title: "See on Github",
                                    img {
                                        style: "width: 48px;",
                                        src: "static/images/github-mark.svg"
                                    }
                                }
                            }
                            h1 {
                                style: style::PROJECT_TITLE,
                                "{pro.name.to_owned()}"
                            }
                            if has_live {
                                rsx!{div{
                                    style: "width: 104px; padding-right: 5px;",
                                    a {
                                        href: "{pro.live.to_owned()}",
                                        style: style::LIVE,
                                        target: "_blank",
                                        title: "See this project live.",
                                        img {
                                            src: "static/images/live-demo.png"
                                        }
                                    }
                                }}
                                } else {
                                    rsx!{div {
                                        style: "width: 104px;",
                                    }}
                                }
                            }
                            div {

                                id: "project_{carousel_page}_details",
                            }
                            if fileType == "webm" {
                                rsx!{
                                    video {
                                        style: style::PROJECT_IMG,
                                        autoplay: true,
                                        "loop": true,
                                        src: "static/images/projects/{pro.image.to_owned()}"
                                    }
                                }
                            } else {
                                rsx!{
                                    a {
                                        href: "static/images/projects/{pro.image.to_owned()}",
                                        target: "_blank",
                                        title: "Click to see full size image",
                                        style: style::IMG_CONTAINER,
                                        img {
                                            style: style::PROJECT_IMG,
                                            src: "static/images/projects/{pro.image.to_owned()}"
                                        }
                                    }
                                }
                            }
                            p {
                               style: style::PROJECT_DESCRIPTION,
                               "See this project on " 
                               a {
                                href: "{pro.github.to_owned()}",
                                target: "_blank",
                                "Github"
                               }
                               if has_live {
                                    rsx!{span {
                                        " and check out the live version running "
                                        a {
                                            href: "{live}",
                                            target: "_blank",
                                            "HERE."
                                        }
                                    }}
                               }
                            }
                            pre {
                                style: style::PROJECT_DESCRIPTION,
                                "{pro.description.to_owned()}"
                            }
                            p {
                                style: style::TECH_STACK,
                                b {
                                    "Tech Stack: "
                                }
                                "{pro.tech_stack.to_owned()}"
                            }
                            div {
                                style: style::LOGO_CONTAINER,
                                for logo in pro.logos {
                                    rsx!{a {
                                        href: "{logo.link.to_owned()}",
                                        target: "_blank",
                                        title: "{logo.title.to_owned()}",
                                        img {
                                            style: style::LOGO,
                                            src: "static/images/{logo.image.to_owned()}",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }})
                // Other Projects page
                div {
                    style: style::CAROUSEL_ITEM,
                    div {
                        style: style::CAROUSEL_CONTENT,
                        h1 {
                            "Other Projects"
                        }
                        projects::minor_projects::minro_projects().iter().map(|mp| {
                            rsx!{

                                p {
                                    style: style::PROJECT_DESCRIPTION,
                                    a {
                                        style: "font-weight: 700;",
                                        href: "{mp.github.to_owned()}",
                                        target: "_blank",
                                        "{mp.name.to_owned()}: "
                                    }
                                    "{mp.description.to_owned()}"
                                }
                            }
                        })
                        hr {
                            style: "width: 80%;"
                        }
                        h2 {
                            "Open Source Contributions"
                        }
                        p {
                            style: style::PROJECT_DESCRIPTION,
                            a {
                                style: "font-weight: 700;",
                                target: "_blank",
                                href: "https://mapknitter.org",
                                "Mapknitter: "
                            }
                            "An open source image(aerial) to map joining application."
                        }
                    }
                }
            }
        }
        
        div {
            style: style::BTN_LEFT,
            class: "carousel_button",
            onclick: move |_| {
                if *carousel_page.get() > 0 {
                    carousel_page -= 1;
                } else {
                    carousel_page.set(number_of_projects)
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
                if *carousel_page.get() < number_of_projects {
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
