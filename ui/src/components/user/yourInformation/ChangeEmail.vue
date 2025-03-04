<template>
  <div>
    <ErrorBanner v-model="error" />
    <InfoBanner v-model="info" />

    <br/>
    <p>
      <strong>Your current email address: </strong> {{ oldEmail }}
    </p>

    <v-form v-model="valid" ref="changeEmailForm">
      <v-text-field
        v-model="newEmail"
        color="primary"
        :rules="rules.email"
        label="New Email"
      />

      <v-text-field
        v-model="password"
        color="primary"
        :rules="rules.required"
        type="password"
        label="Password"
      />

      <div class="d-flex flex-row justify-end">
        <v-btn
          color="primary"
          :loading="loading"
          :disabled="!valid || loading"
          @click="updateEmail">
          Update email
        </v-btn>
      </div>
    </v-form>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue'
import {User} from "@/scripts/user";
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import {InputValidationRules} from "@/main";
import {EMAIL_RULES, REQUIRED_RULES} from "@/scripts/validation_rules";
import {VForm} from "vuetify/components";
import InfoBanner from "@/components/banners/InfoBanner.vue";

interface Data {
  error?: string,
  info?: string,
  loading: boolean,
  oldEmail?: string,
  newEmail?: string,
  password?: string,
  valid: boolean,
  rules: {
    required: InputValidationRules,
    email: InputValidationRules,
  }
}

export default defineComponent({
  components: {InfoBanner, ErrorBanner},
  props: {
    user: {
      type: Object as PropType<User>,
      required: true,
    }
  },
  data(): Data {
    return {
      error: undefined,
      info: undefined,
      oldEmail: undefined,
      newEmail: undefined,
      password: undefined,
      loading: false,
      valid: true,
      rules: {
        required: REQUIRED_RULES,
        email: EMAIL_RULES,
      }
    }
  },
  async mounted() {
    const user = await User.getCurrent();
    this.oldEmail = user.email;
  },
  methods: {
    async validateForm(): Promise<boolean> {
      return (await (<VForm> this.$refs.changeEmailForm).validate()).valid
    },
    async updateEmail() {
      if(!await this.validateForm()) return;

      this.loading = true;

      const user = await User.getCurrent();
      const r = await user.changeEmail(
        this.newEmail!,
        this.password!,
      );

      this.loading = false;

      if(r.isOk()) {
        this.info = "Please check your old email for a verification link"
      } else {
        this.error = r.unwrapErr().message?.toString()
      }
    }
  }
})
</script>