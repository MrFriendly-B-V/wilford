<template>
  <v-container>
    <ErrorBanner v-model="error"/>

    <v-card>
      <v-card-title>Login</v-card-title>
      <v-card-subtitle v-if="!hideAll">Please log in with your EspoCRM account</v-card-subtitle>
      <v-card-text v-if="!hideAll">
        <div v-if="enterUsernamePassword">
          <v-form v-model="usernamePasswordValid">
            <v-text-field
              v-model="username"
              :rules="rules.required"
              label="Username"
            ></v-text-field>
            <v-text-field
              v-model="password"
              :rules="rules.required"
              label="Password"
              type="password"
            ></v-text-field>
          </v-form>
        </div>

        <div v-if="enterTotp">
          <v-form v-model="totpValid">
            <v-text-field
              v-model="totpCode"
              :rules="rules.required"
              label="2FA Code"
            ></v-text-field>
          </v-form>
        </div>
      </v-card-text>
      <v-card-actions v-if="!hideAll">
        <v-btn
          to="/register"
          color="primary"
          variant="tonal">
          Register
        </v-btn>
        <v-spacer></v-spacer>
        <v-btn
          :disabled="(enterUsernamePassword && !usernamePasswordValid) || (enterTotp && !totpValid) || loading"
          :loading="loading"
          variant="elevated"
          color="primary"
          @click="login">
          Login
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import {InputValidationRules, server} from "@/main";
import {defineComponent} from 'vue';
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import {ClientInfo} from "@/scripts/clients";

interface Data {
  error?: string;
  loading: boolean,
  enterUsernamePassword: boolean;
  enterTotp: boolean;
  hideAll: boolean;
  usernamePasswordValid: boolean;
  totpValid: boolean;
  username?: string;
  password?: string;
  totpCode?: string;
  rules: {
    required: InputValidationRules;
  }
}

export default defineComponent({
  components: {ErrorBanner},
  data(): Data {
    return {
      error: undefined,
      loading: false,
      enterUsernamePassword: true,
      enterTotp: false,
      hideAll: false,
      usernamePasswordValid: true,
      totpValid: true,
      username: undefined,
      password: undefined,
      totpCode: undefined,
      rules: {
        required: [
          v => !!v || "Required",
        ]
      }
    }
  },
  async mounted() {
    await this.checkAuthorizationPresent();
  },
  computed: {
    /**
     * The authorization query parameter
     */
    authorizationCode(): string | undefined {
      return this.$route.query['authorization']?.toString()
    }
  },
  methods: {
    /**
     * Check if the `authorization` code is present in the query parameter. If not, fetch it.
     */
    async checkAuthorizationPresent() {
      if(!this.authorizationCode) {
        const client = await ClientInfo.getInternal();
        window.location.href = client.getAuthorizationRedirect();
      }

    },
    async login() {
      const r = await fetch(`${server}/api/v1/auth/login`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          'username': this.username,
          'password': this.password,
          'totp_code': this.totpCode,
          'authorization': this.authorizationCode,
        })
      });

      switch (r.status) {
        case 200:

        interface Response {
          status: boolean,
          totp_required: boolean
        }

          const v: Response = await r.json();

          if (!v.status && !v.totp_required) {
            this.error = "Invalid username or password";
            break;
          }

          if (!v.status && v.totp_required) {
            this.enterTotp = true;
            this.enterUsernamePassword = false;
            break;
          }

          if (v.status) {
            await this.$router.push(`/authorize?authorization=${this.authorizationCode}`);
            return;
          }

          break;
        case 403:
          // Returned in case a (subset)set of requested scopes isnt allowed
          this.error = "You are not allowed to access the requested resource. Please contact your administrator."

          // Hide input fields, don't need them anymore
          this.hideAll = true;
          break;
        default:
          this.error = r.statusText;
          break;
      }
    }
  }
})
</script>