import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router";
import Home from "../pages/Home.vue";
import Calendar from "../pages/calendar/Calendar.vue";
import CalendarItem from "../pages/calendar/CalendarItem.vue";
import CourseAnnouncements from "../pages/courses/CourseAnnouncements.vue";
import CourseAssignment from "../pages/courses/CourseAssignment.vue";
import CourseAssignments from "../pages/courses/CourseAssignments.vue";
import CourseGrades from "../pages/courses/CourseGrades.vue";
import CourseGrade from "../pages/courses/CourseGrade.vue";
import CourseHome from "../pages/courses/CourseHome.vue";
import CourseList from "../pages/courses/CourseList.vue";
import CoursePages from "../pages/courses/CoursePages.vue";
import CoursePage from "../pages/courses/CoursePage.vue";
import CourseQuiz from "../pages/courses/CourseQuiz.vue";
import ComposeMessage from "../pages/messages/ComposeMessage.vue";
import Inbox from "../pages/messages/Inbox.vue";
import SettingsHome from "../pages/settings/SettingsHome.vue";
import SessionList from "../pages/settings/SessionList.vue";
import DataExport from "../pages/settings/DataExport.vue";
import Notifications from "../pages/Notifications.vue";

const routes: readonly RouteRecordRaw[] = [
    { path: "/", component: Home },
    { path: "/calendar", component: Calendar },
    { path: "/calendar/:item", component: CalendarItem },
    { path: "/courses/:course/announcements", component: CourseAnnouncements },
    { path: "/courses/:course/assignments/:assignment", component: CourseAssignment },
    { path: "/courses/:course/assignments", component: CourseAssignments },
    { path: "/courses/:course/grades/:assignment", component: CourseGrade },
    { path: "/courses/:course/grades", component: CourseGrades },
    { path: "/courses/:course", component: CourseHome },
    { path: "/courses", component: CourseList },
    { path: "/courses/:course/pages/:page", component: CoursePage },
    { path: "/courses/:course/pages", component: CoursePages },
    { path: "/courses/:course/assignments/:assignment/quiz", component: CourseQuiz },
    { path: "/inbox/compose", component: ComposeMessage },
    { path: "/inbox", component: Inbox },
    { path: "/settings/export", component: DataExport },
    { path: "/settings/sessions", component: SessionList },
    { path: "/settings", component: SettingsHome },
    { path: "/notifications", component: Notifications },
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});
