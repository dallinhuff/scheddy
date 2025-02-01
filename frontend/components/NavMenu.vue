<script setup lang="ts">
import type { MenuItem } from "primevue/menuitem";

const items: MenuItem[] = [
  { label: "Home", key: "/" },
  { label: "Tubing", key: "/tubing" },
  { label: "Rafting", key: "/rafting" },
  { label: "Kayaking", key: "/kayaking" },
  { label: "Multi-day Rentals", key: "/multi-day-rentals" },
];
</script>

<template>
  <Menubar :model="items">
    <template #item="{ item }">
      <NuxtLink :to="item.key">{{ item.label }}</NuxtLink>
    </template>
  </Menubar>
</template>

<style scoped></style>
