<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Repo {
  name: string;
  html_url: string;
  description: string;
}

const repoList: Ref<Repo[]> = ref<Repo[]>([]);

const username: Ref<String> = ref("");

async function fetchRepos(): Promise<void> {
  const repos: Repo[] = await invoke("get_repos", { username: username.value });
  console.log(repos);
  repoList.value = repos;
}

function handleSubmit(event: Event): void {
  event.preventDefault();
  fetchRepos();
}
</script>

<template>
  <div>
    <form @submit="handleSubmit">
      <input 
        v-model="username"
        label="Enter Github username to fetch repos"
      />
      <button type="submit">Fetch Repos</button>
      <table style="border: 1px solid">
        <tr>
          <th>Name</th>
          <th>URL</th>
          <th>Description</th>
        </tr>
        <tr v-for="repo in repoList">
          <td>{{ repo.name }}</td>
          <td><a :href="repo.html_url">{{ repo.html_url }}</a></td>
          <td>{{ repo.description }}</td>
        </tr>
      </table>
    </form>
  </div>
</template>

<style scoped>
table, td, th {  
  border: 1px solid #ddd;
  text-align: center;
}

table {
  margin: auto;
  margin-top: 20px;
  border-collapse: collapse;
  width: 75%;
}

th, td {
  padding: 15px;
}
</style>
