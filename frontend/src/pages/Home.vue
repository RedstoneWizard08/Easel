<script setup lang="ts">
    import { Suspense } from "vue";
    import Loading from "../components/ui/Loading.vue";
    import BaseLayout from "../layouts/BaseLayout.vue";
    import DashGrid from "../components/dashboard/DashGrid.vue";
</script>

<template>
    <BaseLayout>
        <p class="font-bold text-xl m-0">Dashboard</p>
        <hr class="w-full border-white/20 mt-2" />

        <Suspense>
            <DashGrid />

            <template #fallback>
                <Loading />
            </template>
        </Suspense>
    </BaseLayout>
</template>
