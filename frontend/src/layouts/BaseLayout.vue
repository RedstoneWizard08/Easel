<script setup lang="ts">
    import Sidebar from "../components/sidebar/Sidebar.vue";
</script>

<template>
    <div
        class="bg-dark w-full h-full m-0 p-0 b-0 flex flex-row items-start justify-start text-white"
    >
        <Sidebar />

        <div class="bg-dark-800 rounded-lg p-6 w-full h-[calc(100%_-_2rem)] m-4 ml-24">
            <slot />
        </div>
    </div>
</template>
