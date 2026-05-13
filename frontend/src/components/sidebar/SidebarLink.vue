<script setup lang="ts">
    import { Icon, type IconifyIcon } from "@iconify/vue";
    import { RouterLink } from "vue-router";

    interface Props {
        last?: boolean;
        icon: IconifyIcon | string;
        name: string;
        to: string;
    }

    const { last, icon, name, to } = defineProps<Props>();
</script>

<template>
    <RouterLink
        :to
        class="bg-dark-300 hover:bg-dark-100 transition-all p-2 rounded-lg text-white flex flex-row items-center justify-start w-full"
        :class="{ 'mb-2': !last }"
    >
        <Icon :icon="icon" width="24" />

        <p
            class="m-0 group-hover/sidebar:ml-2 w-0 overflow-hidden group-hover/sidebar:w-unset transition-all pr-0 group-hover/sidebar:pr-2"
        >
            {{ name }}
        </p>
    </RouterLink>
</template>
