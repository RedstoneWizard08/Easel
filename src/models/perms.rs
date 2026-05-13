use enumflags2::bitflags;

#[bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoursePermission {
    /// Modify course metadata (name, description, etc.)
    EditMetadata,

    /// Enroll users from the course.
    EnrollUsers,

    /// Un-enroll users from the course.
    UnenrollUsers,

    /// Create, edit, and delete assignments created by the user.
    CreateAssignment,

    /// Edit assignment details created by any user (name, description, due date, etc.)
    EditAssignment,

    /// Delete assignments created by any user.
    DeleteAssignment,

    /// Assign work to enrolled users.
    AssignWork,

    /// View enrolled users' submitted work.
    ViewSubmissions,

    /// Modify enrolled users' grades for assignments.
    EditGrades,

    /// Add comments to enrolled users' submitted work.
    AddComments,

    /// Create, edit, and delete announcements created by the user.
    CreateAnnouncement,

    /// Edit announcements created by any user.
    EditAnnouncement,

    /// Delete announcements created by any user.
    DeleteAnnouncement,

    /// Create, edit, and delete pages created by the user.
    CreatePage,

    /// Edit pages created by any user.
    EditPage,

    /// Delete pages created by any user.
    DeletePage,

    /// Re-order and add/remove pages/links from the sidebar.
    EditSidebar,

    /// Edit the course home page.
    EditHome,

    /// Create, edit, and delete events from the course's calendar.
    EditCalendar,

    /// Edit the course syllabus.
    EditSyllabus,

    /// Upload files to the course.
    UploadFiles,
}
