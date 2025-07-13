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
        <v-btn
          to="/password-forgotten"
          color="primary"
          variant="tonal">
          Password forgotten
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
import {InputValidationRules} from "@/main";
import {defineComponent} from 'vue';
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import {ClientInfo} from "@/scripts/clients";
import {User} from "@/scripts/user";
import {Auth, LoginStatus} from "@/scripts/auth";

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
    await this.registrationRequired();
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
    async registrationRequired() {
      const requireRegister = await User.isFirstRegister();
      if(requireRegister.isOk()) {
        if(requireRegister.unwrap()) {
          this.$router.replace('/register');
        }
      }
    },
    async login() {
      this.loading = true;

      const loginResult = await Auth.login(
        this.username!,
        this.password!,
        this.authorizationCode!,
        this.totpCode
      );

      this.loading = false;

      if (loginResult.isOk()) {
        switch (loginResult.unwrap()) {
          case LoginStatus.OK: {
            await this.$router.push(`/authorize?authorization=${this.authorizationCode}`);
            return;
          }
          case LoginStatus.INVALID_CREDENTIALS: {
            this.error = "Invalid username or password";
            break;
          }
          case LoginStatus.TOTP_REQUIRED: {
            this.enterTotp = true;
            this.enterUsernamePassword = false;
            break;
          }
          case LoginStatus.EMAIL_UNVERIFIED: {
            this.error = "Your email address is unverified. Please check your email.";
            break;
          }
          case LoginStatus.SCOPE_ERROR: {
            // Returned in case a (subset)set of requested scopes isnt allowed
            this.error = "You are not allowed to access the requested resource. Please contact your administrator."

            // Hide input fields, don't need them anymore
            this.hideAll = true;
          }
        }
      } else {
        this.error = loginResult.unwrapErr().message?.toString() ?? "Something went wrong";
      }
    }
  }
})
</script>