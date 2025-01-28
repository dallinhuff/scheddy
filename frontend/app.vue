<script setup lang="ts">
import type {MenuItem} from "primevue/menuitem";
import NavMenu from "~/components/NavMenu.vue";

const menuItems: MenuItem[] = [
  {label: 'Home', key: '/'},
  {label: 'Tubing', key: '/tubing'},
  {label: 'Rafting', key: '/rafting'},
  {label: 'Kayaking', key: '/kayaking'},
  {label: 'Multi-day Rentals', key: '/multi-day-rentals'},
];
</script>

<template>
  <div>
    <NavMenu/>
    <NuxtPage class="px-3 py-2"/>
  </div>
</template>
