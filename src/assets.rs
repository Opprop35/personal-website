pub const IMAGE_DIR: &str = "/images";

pub fn image(name: &str) -> String {
    format!("{IMAGE_DIR}/{name}")
}
