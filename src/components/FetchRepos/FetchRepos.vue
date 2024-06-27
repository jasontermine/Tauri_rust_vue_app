
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

<script setup lang="ts">
import { 
  repoList, 
  username, 
  handleSubmit 
} from '@/components/FetchRepos/script';
</script>

<style scoped lang="css">
@import "./style.css"
</style>

