<script setup lang="ts">
defineProps<{ pageName?: string }>();

const route = useRoute();
const { data: content } = await useAsyncData(() => {
  return queryCollection("content").path(route.path).first();
});

useSeoMeta({
  title: content.value?.title,
  description: content.value?.description,
});
</script>

<template>
  <ContentRenderer
    v-if="content"
    :value="content"
    class="prose lg:prose-lg dark:prose-invert"
  />
  <div v-else>{{ pageName || "Page" }} not found</div>
</template>
