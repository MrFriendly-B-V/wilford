<template>
  <v-container>
    <ErrorBanner v-model="error"/>
    <InfoBanner v-model="info"/>
    <v-card>
      <v-card-title>Password forgotten</v-card-title>
      <v-card-text>
        <v-form v-model="valid">
          <v-text-field
            label="Email"
            v-model="email"
            :rules="rules.required"
            color="primary"
          />
        </v-form>
      </v-card-text>
      <v-card-actions>
        <v-btn
          to="/login"
          color="primary"
          variant="tonal">
          Login
        </v-btn>
        <v-spacer/>
        <v-btn
          @click="resetPassword"
          :loading="loading"
          variant="elevated"
          color="primary"
          :disabled="!valid || loading">
          Submit
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {InputValidationRules} from "@/main";
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import {User} from "@/scripts/user";
import InfoBanner from "@/components/banners/InfoBanner.vue";

interface Data {
  error?: string;
  info?: string;
  loading: boolean;
  valid: boolean;
  email?: string;
  rules: {
    required: InputValidationRules,
  }
}

export default defineComponent({
  components: {InfoBanner, ErrorBanner},
  data(): Data {
    return {
      error: undefined,
      info: undefined,
      loading: false,
      valid: true,
      email: undefined,
      rules: {
        required: [
          v => !!v || "Required",
        ]
      }
    }
  },
  methods: {
    async resetPassword() {
      this.loading = true;
      const result = await User.resetPassword(this.email!);
      this.loading = false;

      if(result.isOk()) {
        this.info = "If the email address is associated with a user, you will have received a temporary password. Please check your email.";
        this.email = undefined;
      } else {
        this.error = result.unwrapErr().message;
      }
    }
  }
})
</script>