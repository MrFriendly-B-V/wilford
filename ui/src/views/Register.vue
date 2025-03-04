<template>
  <v-container>
    <ErrorBanner v-model="error"/>

    <v-card>
      <v-card-title>Register</v-card-title>
      <v-card-subtitle v-if="isFirstRegister">First time set up. Please register an account</v-card-subtitle>
      <v-card-subtitle v-else>Register an account</v-card-subtitle>
      <v-card-text>
        <v-form v-model="valid" ref="registerForm">
          <v-text-field
            v-model="newName"
            color="primary"
            :rules="rules.required"
            label="Name"
          />
          <v-text-field
            v-model="newEmail"
            color="primary"
            :rules="rules.required"
            label="E-Mail"
          />
          <v-text-field
            v-model="newPassword"
            color="primary"
            type="password"
            :rules="rules.password"
            label="Password"
          />
          <v-text-field
            v-model="newRepeatPassword"
            color="primary"
            type="password"
            :rules="rules.repeatPassword"
            label="Repeat Password"
          />
          <v-select
            v-model="locale"
            color="primary"
            :items="availableLocales"
            item-title="value"
            item-value="key"
            labels="Locale"
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
          :loading="loading"
          :disabled="!valid || loading"
          variant="elevated"
          color="primary"
          @click="register">
          Register
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import {InputValidationRules} from "@/main";
import {User} from "@/scripts/user";
import {VForm} from "vuetify/components";
import {PASSWORD_RULES, REQUIRED_RULES} from "@/scripts/validation_rules";

interface Data {
  error?: string;
  valid: boolean;
  loading: boolean;
  isFirstRegister: boolean;
  newName?: string;
  newEmail?: string;
  newPassword?: string;
  newRepeatPassword?: string;
  locale?: string;
  availableLocales: KVPair[];
  rules: {
    required: InputValidationRules;
    password: InputValidationRules;
    repeatPassword: InputValidationRules;
  }
}

interface KVPair {
  key: string;
  value: string;
}

export default defineComponent({
  components: {ErrorBanner},
  data(): Data {
    return {
      error: undefined,
      valid: true,
      loading: false,
      isFirstRegister: false,
      newName: undefined,
      newEmail: undefined,
      newPassword: undefined,
      newRepeatPassword: undefined,
      locale: 'Nl',
      availableLocales: [
        { key: 'Nl', value: 'Nederlands' },
        { key: 'En', value: 'English' },
      ],
      rules: {
        required: REQUIRED_RULES,
        password: PASSWORD_RULES,
        repeatPassword: PASSWORD_RULES.concat([
            v => v == (<Data> this.$data).newPassword || "Password must be the same",
        ])
      }
    }
  },
  async mounted() {
    await this.loadIsFirstRegister();
  },
  methods: {
    async loadIsFirstRegister() {
      const result = await User.isFirstRegister();
      if(result.isOk()) {
        this.isFirstRegister = result.unwrap();
      } else {
        this.error = result.unwrapErr().message;
      }
    },
    async verifyRegisterForm(): Promise<boolean> {
      return (await (<VForm> this.$refs.registerForm).validate()).valid;
    },
    async register() {
      if (!await this.verifyRegisterForm()) return;

      this.loading = true;
      const result = await User.register(this.newName!, this.newEmail!, this.newPassword!, this.locale!);
      this.loading = false;

      if(result.isOk()) {
        this.$router.push('/');
      } else {
        this.error = result.unwrapErr().message;
      }
    }
  }
})
</script>