<template>
  <v-app>
    <default-bar />

    <default-view v-if="!loading"/>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import DefaultBar from '../AppBar.vue'
import DefaultView from '../View.vue'
import {isAuthorized} from "@/layouts/authorizations/authorization";

interface Data {
  loading: boolean,
}

export default defineComponent({
  components: { DefaultBar, DefaultView },
  data(): Data {
    return {
      loading: true,
    }
  },
  async mounted() {
    await this.$router.isReady();
    await isAuthorized(false);

    this.loading = false;
  },
})
</script>
