use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(description = "Easel Platform API"),
    paths(
        // Assignments
        crate::controllers::assignment::list,
        crate::controllers::assignment::add,
        crate::controllers::assignment::update,
        crate::controllers::assignment::remove,
        crate::controllers::assignment::get_one,

        // Auth
        crate::controllers::auth::register,
        crate::controllers::auth::verify,
        crate::controllers::auth::forgot,
        crate::controllers::auth::reset,
        crate::controllers::auth::login,
        crate::controllers::auth::current,
        crate::controllers::auth::magic_link,
        crate::controllers::auth::magic_link_verify,
        crate::controllers::auth::resend_verification_email,

        // Course Pages
        crate::controllers::course_page::list,
        crate::controllers::course_page::add,
        crate::controllers::course_page::update,
        crate::controllers::course_page::remove,
        crate::controllers::course_page::get_one,

        // Courses
        crate::controllers::course::list,
        crate::controllers::course::add,
        crate::controllers::course::update,
        crate::controllers::course::remove,
        crate::controllers::course::get_one,

        // Enrollments
        crate::controllers::enrollment::list,
        crate::controllers::enrollment::add,
        crate::controllers::enrollment::update,
        crate::controllers::enrollment::remove,
        crate::controllers::enrollment::get_one,

        // Submissions
        crate::controllers::submission::list,
        crate::controllers::submission::add,
        crate::controllers::submission::update,
        crate::controllers::submission::remove,
        crate::controllers::submission::get_one,
    )
)]
pub struct ApiDocs;
