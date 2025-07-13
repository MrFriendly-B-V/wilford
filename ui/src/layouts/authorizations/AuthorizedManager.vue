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
import {ClientInfo} from "@/scripts/clients";

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
    const isManager = await isAuthorized(true);
    if(!isManager) {
      const client = await ClientInfo.getInternal();
      window.location.href = client.getAuthorizationRedirect(true);
    }

    this.loading = false;
  },
})
</script>
