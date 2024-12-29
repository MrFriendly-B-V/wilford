<template>
  <v-container>
    <v-card>
      <v-card-title>Home</v-card-title>
      <v-card-text>
        Redirecting...
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  mounted() {
    this.$router.replace('/me');
  }
});
</script>