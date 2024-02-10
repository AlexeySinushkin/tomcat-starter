<script setup lang="ts">
  import Settings from '@/components/settings/Settings.vue'
  import BackendApiMockImpl from './backendApiMockImpl.ts';
import BackendApi, { BackendApiImpl } from './backendApi';
</script>

<template>
  <v-layout class="rounded rounded-md">
    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
      <Settings :api="api"/>
    </v-main>
  </v-layout>
</template>

<script lang="ts">
let api : BackendApi;
if (import.meta.env.VITE_DEBUG_UI===true){
  console.log('using mock');
  api = new BackendApiMockImpl();
}else{
  console.log('using normal backend')
  api = new BackendApiImpl();
}

export default {
  data: () => ({
    tab: null,
    api
  }),
};
</script>