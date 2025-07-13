<template>
  <v-container>
    <ErrorBanner v-model="error"/>

    <v-card :loading="loading">
      <v-card-title>Password change required</v-card-title>
      <v-card-subtitle>You must change your password before you can continue</v-card-subtitle>

      <v-card-text>
        <ChangePassword
          :user="user!"
          @complete="$router.replace('/me')"
        />
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {User} from "@/scripts/user";
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import ChangePassword from "@/components/user/security/ChangePassword.vue";

interface Data {
  error?: string;
  loading: boolean;
  user?: User,
}

export default defineComponent({
  components: {ChangePassword, ErrorBanner},
  data(): Data {
    return {
      error: undefined,
      loading: true,
      user: undefined,
    }
  },
  async mounted() {
    await this.loadUser();
  },
  methods: {
    async loadUser() {
      const userInfo = await User.getCurrent();
      if(!userInfo.requirePasswordChange) {
        this.$router.push('/me');
        return;
      }

      this.user = userInfo;
      this.loading = false;
    }
  }
})
</script>