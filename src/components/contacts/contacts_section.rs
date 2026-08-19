use dioxus::prelude::*;

use super::{form::MessageForm, linkedin_logo::LinkedinLogo, github_logo::GithubLogo, discord_logo::DiscordLogo, socials_adialog::SocialsAlertDialog};

#[component]
pub fn ContactsSection() -> Element {
    rsx! {
        section { id: "contacts", class: "max-md:min-h-fit md:h-screen container-x",
            h3 { class: "text-center font-bold mb-16 text-3xl md:text-5xl", "CONTACTS" }

            div { class: "grid gap-y-10 md:grid-cols-2",
                // Socials section
                div {
                    h4 { class: "text-lg text-blue-500 mb-8 md:text-2xl", "SOCIALS" }
                    div {
                        img {
                            class: "h-75 mx-auto sm:w-95",
                            src: asset!("/assets/static/socials.svg"),
                            alt: "socials logo",
                            loading: "lazy",
                        }
                    }
                    div { class: "flex justify-between sm:justify-center sm:gap-x-27",
                        SocialsAlertDialog {
                            social_site: "Linkedin",
                            link: "https://www.linkedin.com/in/kmfiguerrez",
                            LinkedinLogo {}
                        }
                        SocialsAlertDialog {
                            social_site: "Github",
                            link: "https://github.com/kmfiguerrez",
                            GithubLogo {}
                        }
                        SocialsAlertDialog {
                            social_site: "Discord",
                            link: "https://discord.com/channels/@me",
                            DiscordLogo {}
                        }
                    }
                }
                // Message section
                div {
                    h4 { class: "text-lg text-blue-500 mb-8 md:text-2xl", "SEND ME A MESSAGE" }
                    MessageForm {}
                }
            }
            a { href: "#home", class: "block w-fit mx-auto sm:hidden",
                img {
                    class: "h-10 w-10 animate-bounce",
                    src: asset!("/assets/static/up-arrow-white.svg"),
                    alt: "white up arrow icon",
                    loading: "lazy",
                }
            }
        }
    }
}