<template>
  <div class="mt-3">
    <ErrorBanner v-model="error"/>

    <v-form v-model="valid" ref="changePasswordForm">
      <v-text-field
        v-model="oldPassword"
        color="primary"
        :rules="rules.required"
        type="password"
        label="Old Password"
      />
      <v-text-field
        v-model="newPassword"
        color="primary"
        :rules="rules.password"
        type="password"
        label="New Password"
      />
      <v-text-field
        v-model="repeatNewPassword"
        color="primary"
        :rules="rules.repeatPassword"
        type="password"
        label="Repeat New Password"
      />
    </v-form>

    <div class="d-flex flex-row justify-end">
      <v-btn
        color="primary"
        :loading="loading"
        :disabled="!valid"
        @click="updatePassword">
        Update password
      </v-btn>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue';
import {User} from "@/scripts/user";
import {InputValidationRules} from "@/main";
import {VForm} from "vuetify/components";
import MaterialBanner from "@/components/banners/MaterialBanner.vue";
import ErrorBanner from "@/components/banners/ErrorBanner.vue";

interface Data {
  error?: string,
  loading: boolean,
  valid: boolean,
  oldPassword?: string,
  newPassword?: string,
  repeatNewPassword?: string,
  rules: {
    required: InputValidationRules,
    password: InputValidationRules,
    repeatPassword: InputValidationRules,
  }
}

export default defineComponent({
  components: {ErrorBanner, MaterialBanner},
  props: {
    user: {
      type: Object as PropType<User>,
      required: true,
    }
  },
  data(): Data {
    return {
      valid: true,
      loading: false,
      error: undefined,
      newPassword: undefined,
      oldPassword: undefined,
      repeatNewPassword: undefined,
      rules: {
        required: [
          v => !!v || "Required",
        ],
        password: [
          v => !!v || "Required",
        ],
        repeatPassword: [
          v => !!v || "Required",
          v => v == (<Data> this.$data).newPassword || "Password must be the same",
        ]
      }
    }
  },
  methods: {
    async validateForm(): Promise<boolean> {
      return (await (<VForm> this.$refs.changePasswordForm).validate()).valid;
    },
    async updatePassword() {
      if(!await this.validateForm()) return;

      const user = await User.getCurrent();
      await user.updatePassword(this.oldPassword!, this.newPassword!);
    }
  }
})
</script>