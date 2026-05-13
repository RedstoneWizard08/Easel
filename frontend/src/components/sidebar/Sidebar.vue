<script setup lang="ts">
    import SidebarLink from "./SidebarLink.vue";
</script>

<template>
    <div
        class="bg-dark-900 m-4 p-2 rounded-lg group/sidebar peer/sidebar z-20 fixed flex flex-col items-start justify-between h-[calc(100%_-_2rem)] hover:rounded-r-none hover:b-r-1 b-r-white/10"
    >
        <div class="flex flex-col">
            <SidebarLink to="/" icon="tabler:home-filled" name="Home" />
            <SidebarLink to="/courses" icon="tabler:book-filled" name="Courses" />
            <SidebarLink to="/calendar" icon="tabler:calendar-filled" name="Calendar" />
            <SidebarLink to="/inbox" icon="tabler:mail-filled" name="Inbox" last />
        </div>

        <SidebarLink to="/settings" icon="tabler:settings-filled" name="Settings" last />
    </div>

    <div
        class="bg-black/0 pointer-events-none w-full h-full fixed z-10 top-0 left-0 peer-hover/sidebar:bg-black/30 peer-hover/sidebar:pointer-events-unset transition-all"
    ></div>
</template>
