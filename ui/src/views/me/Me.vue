<template>
  <v-container>


    <v-card :loading="loading">
      <div v-if="user">
        <v-card-title>Hi, {{ user.name }}</v-card-title>
        <v-card-subtitle>Manage your account</v-card-subtitle>

        <v-card-text>
          <v-expansion-panels>
            <v-expansion-panel>
              <v-expansion-panel-title>Security</v-expansion-panel-title>
              <v-expansion-panel-text>
                <Security :user="user"/>
              </v-expansion-panel-text>
            </v-expansion-panel>
            <v-expansion-panel>
              <v-expansion-panel-title>Your information</v-expansion-panel-title>
              <v-expansion-panel-text>
                <Information :user="user"/>
              </v-expansion-panel-text>
            </v-expansion-panel>
          </v-expansion-panels>
        </v-card-text>
      </div>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import {User} from "@/scripts/user";
import Security from "@/components/user/Security.vue";
import Information from "@/components/user/Information.vue";

interface Data {
  error?: string;
  loading: boolean;
  user?: User;
}

export default defineComponent({
  components: {Information, Security},
  data(): Data {
    return {
      error: undefined,
      loading: true,
      user: undefined,
    }
  },
  async mounted() {
    this.user = await User.getCurrent();
    this.loading = false;
  }
});
</script>