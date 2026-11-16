pub mod layout;
pub mod links;
pub mod menus;
pub mod nav;
pub mod text_sized_image;
pub mod toast;

/// Base style for interactive background surfaces (nav items, menu items,
/// buttons): the default rounded corner, with colors smoothly inverting to
/// the hover background/ink on hover.
pub const BACKGROUND_BUTTON: &str =
    "rounded-default transition-colors duration-150 hover:bg-background-hover hover:text-background-hover-ink";

pub use layout::Layout;
pub use links::{Link, IMAGE_LINK};
pub use menus::projects_menu::ProjectsMenu;
pub use nav::{NavButton, NAV_ITEM};

pub use text_sized_image::TextSizedImage;
pub use toast::Toasts;
