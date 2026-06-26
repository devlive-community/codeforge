<template>
  <div class="min-h-screen flex flex-col font-sans antialiased bg-white dark:bg-slate-950 text-slate-800 dark:text-slate-200">
    <SiteHeader/>
    <main class="flex-1">
      <RouterView v-slot="{ Component, route }">
        <DocLayout v-if="route.meta.doc">
          <component :is="Component"/>
        </DocLayout>
        <component :is="Component" v-else/>
      </RouterView>
    </main>
    <SiteFooter/>
  </div>
</template>

<script setup lang="ts">
import SiteHeader from './components/SiteHeader.vue'
import SiteFooter from './components/SiteFooter.vue'
import DocLayout from './layouts/DocLayout.vue'
</script>
