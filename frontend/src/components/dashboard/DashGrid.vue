<script setup lang="ts">
    import { createMockCourseList } from "../../lib/mock/course";
    import DashCard from "./DashCard.vue";

    const courses = await createMockCourseList();
</script>

<template>
    <div class="mt-4 grid grid-cols-5 w-full gap-8">
        <DashCard :course v-for="course in courses" />
    </div>
</template>
