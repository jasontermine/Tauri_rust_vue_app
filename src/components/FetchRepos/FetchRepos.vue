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
          </tr>
          <tr v-for="repo in repoList" :key="repo.name">
            <td>{{ repo.name }}</td>
            <td><a style="color: lightblue;" :href="repo.html_url">{{ repo.html_url }}</a></td>
            <td>{{ repo.description }}</td>
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
</script>

<style scoped lang="css">
@import "./style.css";
</style>
