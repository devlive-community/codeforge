<template>
  <h1>发布日志</h1>
  <p>选择一个版本查看详细更新内容。</p>
  <ul>
    <li v-for="r in releases" :key="r.version">
      <RouterLink :to="`/release/${r.version}`">{{ r.title || r.version }}</RouterLink>
    </li>
  </ul>
</template>

<script setup lang="ts">
import {releases} from '../content/releases'
</script>
