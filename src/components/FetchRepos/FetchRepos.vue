<template>
  <div>
    <form @submit="handleSubmit">
      <input 
        v-model="username"
        label="Enter Github username to fetch repos"
      />
      <button type="submit">Fetch Repos</button>
      <div style="overflow-x: auto;">
        <table>
          <tr>
            <th>Name</th>
            <th>URL</th>
            <th>Description</th>
            <th>Actions</th>
          </tr>
          <tr v-for="repo in repoList" :key="repo.name">
            <td>{{ repo.name }}</td>
            <td><a style="color: lightblue;" :href="repo.html_url">{{ repo.html_url }}</a></td>
            <td>{{ repo.description }}</td>
            <td><VBtn @click="cloneRepo(repo.html_url)" variant="tonal">Clone</VBtn></td>
          </tr>
        </table>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { 
  repoList, 
  username, 
  handleSubmit 
} from '@/components/FetchRepos/script';
import { invoke } from "@tauri-apps/api/tauri";

async function cloneRepo(repoUrl: string) {
  try {
    await invoke("clone_repo", { repoUrl });
  } catch (error) {
    alert(`Failed to clone repo: ${error}`);
  }
}
</script>

<style scoped lang="css">
@import "./style.css";
</style>
