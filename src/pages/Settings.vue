<template>
  <h1 class="text-center" style="color: whitesmoke">Settings</h1>

  <div class="text-center" style="color: whitesmoke">
    <VBtn @click="setDirectoryPath" variant="tonal"> Set Directory Path </VBtn>
    <br />
    <span>Selected Directory: {{ selected }}</span>
  </div>
</template>

<script setup lang="ts">
import { useUserPreferenceStore } from "@/store/UserPreferenceStore";
import { invoke } from "@tauri-apps/api/tauri";
import { computed } from "vue";

const userPreferenceStore = useUserPreferenceStore();
const selected = computed(() => userPreferenceStore.getPath);

async function setDirectoryPath() {
  try {
    const selectedDir = await invoke("open_directory");
    if (selectedDir !== undefined) {
      userPreferenceStore.setPath(selectedDir as string);
      await invoke("set_directory_path", { path: selectedDir });
    }
  } catch (error) {
    alert(`Failed to select directory: ${error}`);
  }
}
</script>

<style scoped></style>
