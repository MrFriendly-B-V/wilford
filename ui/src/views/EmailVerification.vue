<template>
    <v-container>
      <ErrorBanner v-model="error" />
      <InfoBanner v-model="info" />

      <v-card>
        <v-card-title>Email verification</v-card-title>

        <v-card-text>
          <div class="d-flex flex-row justify-start align-center" v-if="loading">
            <v-progress-circular
              indeterminate
              class="mr-3"
              color="primary"
            />
            Loading...
          </div>
        </v-card-text>
      </v-card>
    </v-container>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import {User} from "@/scripts/user";
import InfoBanner from "@/components/banners/InfoBanner.vue";

interface Data {
  error?: string,
  info?: string,
  loading: boolean,
}

export default defineComponent({
  components: {InfoBanner, ErrorBanner},
  data(): Data {
    return {
      error: undefined,
      info: undefined,
      loading: true,
    }
  },
  mounted() {
    if(this.isDebug()) return;
    this.verifyEmail()
  },
  methods: {
    async verifyEmail() {
      this.loading = true;

      const userId = this.userId();
      const code = this.code();

      if(!userId || !code) {
        this.error = "The page you opened is invalid. Please double check the URL.";
        this.loading = false;
        return;
      }

      const result = await User.verifyEmail(userId, code);
      if(result.isOk()) {
        this.info = "Your email address has been verified! You can now close this page";
      } else {
        this.error = result.unwrapErr().message?.toString() ?? "Something went wrong";
      }

      this.loading = false;
    },
    queryParameter(key: string): string | null {
      const value = this.$route.query[key];
      if (value && typeof value === 'string') {
        return value;
      } else {
        return null;
      }
    },
    isDebug(): boolean {
      const val = this.queryParameter('debugMode');
      return val === 'true';
    },
    userId(): string | null {
      return this.queryParameter('user_id');
    },
    code(): string | null {
      return this.queryParameter('code');
    },
  }
})
</script>