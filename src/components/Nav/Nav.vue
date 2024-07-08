<template>
  <VLayout>
    <VNavigationDrawer
      style="background-color: #323243; color: whitesmoke"
      expand-on-hover
      rail
    >
      <VList>
        <VListItem
          prepend-avatar="https://randomuser.me/api/portraits/women/85.jpg"
          subtitle="sandra_a88@gmailcom"
          title="Sandra Adams"
        ></VListItem>
      </VList>

      <VDivider />

      <VList density="compact" nav>
        <VListItem
          prepend-icon="mdi-github"
          title="Overview"
          value="overview"
        ></VListItem>
        <VListItem
          prepend-icon="mdi-cog"
          title="Settings"
          value="settings"
        ></VListItem>
      </VList>
    </VNavigationDrawer>

    <v-main style="height: 250px"></v-main>
  </VLayout>
</template>

<script setup lang="ts">
import SvgIcon from '@jamescoyle/vue-icon';
</script>
