//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod home;
pub use home::HomeSection;

mod about_me;
pub use about_me::AboutMeSection;

mod projects;
pub use projects::ProjectsSection;

mod contacts;
pub use contacts::ContactsSection;pub mod card;
pub mod separator;
pub mod button;
