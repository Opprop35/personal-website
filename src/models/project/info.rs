use crate::models::project::end_date::ProjectEndDate;
use crate::models::project::genre::Genre;
use crate::models::project::image_link::ImageLink;
use crate::models::project::project_type::ProjectType;
use crate::models::skill::Skill;

pub struct Info {
    pub role: &'static str,
    pub genres: Option<&'static [Genre]>,
    pub project_type: ProjectType,
    pub platforms: &'static str,
    pub team_size: &'static str,
    pub duration: &'static str,
    pub end_date: ProjectEndDate,
    pub skills: &'static [Skill],
    pub socials: &'static [ImageLink],
}
