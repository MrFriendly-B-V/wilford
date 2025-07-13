<template>
  <v-container>
    <MaterialBanner
      title="Error"
      type="error"
      icon="mdi-alert-circle-outline"
      :text="error"
      @close="error = undefined"
    />

    <h3 class="mt-3">Change password</h3>
    <div>
      <v-progress-circular indeterminate v-if="loading.passwordChange"/>
      <ChangePassword v-else-if="supportPasswordChange" :user="user"/>
      <p v-else>
        Password change is not supported
      </p>
    </div>

    <h3 class="mt-3">Two-Factor authentication</h3>
    <p>Not implemented</p>
  </v-container>
</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue';
import {User} from "@/scripts/user";
import ChangePassword from "@/components/user/security/ChangePassword.vue";
import MaterialBanner from "@/components/banners/MaterialBanner.vue";

interface Data {
  loading: {
    passwordChange: boolean;
  };
  error?: string;
  supportPasswordChange: boolean;
}

export default defineComponent({
  components: {MaterialBanner, ChangePassword},
  props: {
    user: {
      type: Object as PropType<User>,
      required: true,
    }
  },
  data(): Data {
    return {
      loading: {
        passwordChange: true,
      },
      supportPasswordChange: false,
      error: undefined,
    }
  },
  async mounted() {
    await this.supportsPasswordChange();
  },
  methods: {
    async supportsPasswordChange() {
      const result = await User.passwordChangeSupported();
      this.loading.passwordChange = false;

      if(result.isOk()) {
        this.supportPasswordChange = result.unwrap();
      } else {
        this.error = result.unwrapErr().message;
      }
    }
  }
})
</script>