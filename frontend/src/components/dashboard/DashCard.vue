<script setup lang="ts">
    import { Icon } from "@iconify/vue";
    import type { Course } from "../../lib/models/course";

    interface Props {
        course: Course;
    }

    const { course } = defineProps<Props>();
</script>

<template>
    <RouterLink
        :to="'/courses/' + course.id"
        class="bg-dark-400 w-full rounded-lg scale-100 hover:scale-105 transition-transform"
    >
        <img
            :src="course.banner"
            :alt="course.name + ' banner'"
            class="object-cover w-full h-30 rounded-t-lg"
        />

        <div class="p-2">
            <p class="text-white font-bold">{{ course.name }}</p>
            <p class="text-xs text-white/40">Course ID: {{ course.code }}</p>
        </div>

        <div class="flex flex-row items-center justify-start bg-dark-500 rounded-b-lg">
            <RouterLink
                :to="'/courses/' + course.id + '/announcements'"
                class="bg-dark-500 hover:bg-dark-200 p-2 flex flex-row items-center justify-center transition-all rounded-bl-lg"
                title="Announcements"
            >
                <Icon icon="tabler:speakerphone" width="24" />
            </RouterLink>

            <RouterLink
                :to="'/courses/' + course.id + '/assignments'"
                class="bg-dark-500 hover:bg-dark-200 p-2 flex flex-row items-center justify-center transition-all"
                title="Assignments"
            >
                <Icon icon="tabler:clipboard-check-filled" width="24" />
            </RouterLink>
        </div>
    </RouterLink>
</template>
