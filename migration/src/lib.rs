#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;
pub mod util;

mod m20260511_203252_courses;
mod m20260511_204100_enrollments;
mod m20260511_210021_add_description_to_courses;
mod m20260511_212010_add_settings_to_course;
mod m20260511_212423_assignments;
mod m20260511_212652_course_pages;
mod m20260511_214101_submissions;
mod m20260511_215008_add_course_ref_to_assignments;
mod m20260511_215047_add_owner_ref_to_courses;
mod m20260511_215151_add_home_page_ref_to_courses;
mod m20260511_215210_add_syllabus_page_ref_to_courses;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260511_203252_courses::Migration),
            Box::new(m20260511_204100_enrollments::Migration),
            Box::new(m20260511_210021_add_description_to_courses::Migration),
            Box::new(m20260511_212010_add_settings_to_course::Migration),
            Box::new(m20260511_212423_assignments::Migration),
            Box::new(m20260511_212652_course_pages::Migration),
            Box::new(m20260511_214101_submissions::Migration),
            Box::new(m20260511_215008_add_course_ref_to_assignments::Migration),
            Box::new(m20260511_215047_add_owner_ref_to_courses::Migration),
            Box::new(m20260511_215151_add_home_page_ref_to_courses::Migration),
            Box::new(m20260511_215210_add_syllabus_page_ref_to_courses::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
