<template>
  <v-app-bar flat class="d-flex flex-row justify-space-between">
    <v-app-bar-title class="font-wilford">
      Wilford
    </v-app-bar-title>

    <v-btn
      icon="mdi-cog"
      slim
      to="/manager"
      density="compact"
    />
  </v-app-bar>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Tinos&display=swap');

.font-wilford {
    font-family: 'Tinos', serif;
}
</style>

<script lang="ts" setup>
  //
</script>
