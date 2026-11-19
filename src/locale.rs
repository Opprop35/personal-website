use leptos::prelude::*;

const RESUME_REGIONS: [&str; 3] = ["us", "ca", "au"];

pub fn resume_label() -> &'static str {
    let is_resume_region = window().navigator().language().is_some_and(|language| {
        language
            .to_ascii_lowercase()
            .split('-')
            .skip(1)
            .any(|region| RESUME_REGIONS.contains(&region))
    });

    if is_resume_region {
        "Resume"
    } else {
        "CV"
    }
}
