import { faker } from "@faker-js/faker";
import type { Course } from "../models/course";

export const createMockCourse = (): Course => ({
    id: faker.number.int(),
    code: faker.string.alphanumeric({ length: 5 }),
    created_at: faker.date.anytime().toString(),
    updated_at: faker.date.anytime().toString(),
    home_page: faker.number.int(),
    syllabus_page: faker.number.int(),
    name: "Test Course",
    owner: faker.number.int(),

    banner: faker.image.url({
        width: 500,
        height: 200,
    }),
});

// async so we simulate fetching for API usage
export const createMockCourseList = (): Promise<Course[]> => {
    return new Promise((res, _) => {
        const list: Course[] = [];

        for (let i = 0; i < faker.number.int({ min: 5, max: 20 }); i++) {
            list.push(createMockCourse());
        }

        setTimeout(
            () => {
                res(list);
            },
            faker.number.float({ min: 1, max: 2 }) * 1000,
        );
    });
};
