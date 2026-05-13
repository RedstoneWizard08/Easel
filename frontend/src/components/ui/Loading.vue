<script setup lang="ts">
    import { onMounted, ref } from "vue";

    const dots = 5;
    const prefix = "Loading";
    const interval = 500;

    let text = ref("");
    let dotCount = 0;

    onMounted(() => {
        setInterval(() => {
            dotCount = dotCount >= dots ? 0 : dotCount + 1;
            text.value = prefix + ".".repeat(dotCount);
        }, interval);
    });
</script>

<template>
    <div class="flex w-full h-full flex-row items-center justify-center">
        <p class="text-lg font-bold">{{ text }}</p>
    </div>
</template>
