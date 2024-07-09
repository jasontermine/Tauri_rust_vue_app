<template>
  <div class="container">
    <FetchRepos />
  </div>
</template>

<script setup lang="ts">
import FetchRepos from '@/components/FetchRepos/FetchRepos.vue';

</script>

<style scoped>
.container {
  margin-top: 5rem;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
