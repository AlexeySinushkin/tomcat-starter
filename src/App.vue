<script setup lang="ts">
  import Settings from '@/components/settings/Settings.vue'
  import BackendApiMockImpl from './backendApiMockImpl.ts';
</script>

<template>
  <v-layout class="rounded rounded-md">
    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
      <Settings :api="api"/>
    </v-main>
  </v-layout>
</template>

<script lang="ts">

export default {
  data: () => ({
    tab: null,
    api: import.meta.env.VITE_DEBUG_UI ? new BackendApiMockImpl() : new BackendApiMockImpl()
  }),
};
</script>