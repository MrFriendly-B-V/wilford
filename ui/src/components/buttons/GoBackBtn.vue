<template>
  <v-btn
    density="compact"
    icon="mdi-arrow-left"
    slim
    variant="flat"
    @click="$router.back()"
  />
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({});
</script>