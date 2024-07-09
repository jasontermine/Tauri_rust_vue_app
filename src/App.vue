<template>
  <RouterView />
  <Nav style="position: absolute;" /> 
</template>

<script setup lang="ts">
import Nav from "@/components/Nav/Nav.vue";
import { RouterView } from "vue-router";
</script>

<style scoped>
#app {
  font-family: Roboto, Helvetica, Arial, sans-serif;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
</style>
