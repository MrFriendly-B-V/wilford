<template>
  <div>
    <ErrorBanner v-model="error" />
    <InfoBanner v-model="info" />

    <v-form v-model="valid" ref="changeNameForm">
      <v-text-field
        v-model="newName"
        color="primary"
        :rules="rules.required"
        label="New Name"
      />

      <div class="d-flex flex-row justify-end">
        <v-btn
          color="primary"
          :loading="loading"
          :disabled="!valid || loading"
          @click="updateName">
          Update name
        </v-btn>
      </div>
    </v-form>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import InfoBanner from "@/components/banners/InfoBanner.vue";
import {InputValidationRules} from "@/main";
import {REQUIRED_RULES} from "@/scripts/validation_rules";
import {User} from "@/scripts/user";
import {VForm} from "vuetify/components";

interface Data {
  error?: string,
  info?: string,
  loading: boolean,
  newName?: string,
  valid: boolean,
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
      newName: undefined,
      loading: false,
      valid: true,
      rules: {
        required: REQUIRED_RULES,
      }
    }
  },
  methods: {
    async validateForm(): Promise<boolean> {
      return (await (<VForm> this.$refs.changeNameForm).validate()).valid
    },
    async updateName() {
      if(!await this.validateForm()) return;

      this.loading = true;

      const user = await User.getCurrent();
      const r = await user.changeName(
        this.newName!,
      );

      this.loading = false;

      if(r.isOk()) {
        this.info = "Your name has been updated!";
        await new Promise(resolve => setTimeout(resolve, 750));
        this.refreshPage();
      } else {
        this.error = r.unwrapErr().message?.toString();
      }
    },
    refreshPage() {
      window.location.reload();
    }
  }
})
</script>