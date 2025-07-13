<template>
    <v-container>
        <v-card>
            <v-card-title>Authorize</v-card-title>
            <v-card-subtitle>Grant authorization</v-card-subtitle>

            <div v-if="loading">
                <v-card-text>
                    <v-progress-circular indeterminate></v-progress-circular>
                </v-card-text>
            </div>
            <div v-else>
                <v-card-text>
                    Grant '{{ clientName }}' access to your account?
                    <div v-if="scopes && scopes.length > 0">
                        Scopes:
                        <v-list>
                            <v-list-item v-for="scope in scopes">
                                {{ scope }}
                            </v-list-item>
                        </v-list>

                    </div>
                </v-card-text>

                <v-card-actions>
                    <v-btn
                        @click="denyAuth">
                        Deny
                    </v-btn>
                    <v-spacer></v-spacer>
                    <v-btn
                        @click="allowAuth">
                        Allow
                    </v-btn>
                </v-card-actions>
            </div>
        </v-card>
    </v-container>
</template>

<script lang="ts">

import {server} from "@/main";
import { defineComponent } from "vue";

interface Data {
  loading: boolean,
  scopes: string[],
  clientName: string | null,
}

export default defineComponent({
  data(): Data {
    return {
      loading: false,
      scopes: [],
      clientName: null,
    }
  },
  async mounted() {
    await this.$router.isReady();
    await this.loadAuthorizationInfo();
  },
  computed: {
    authorization(): string {
      return this.$route.query['authorization']!.toString();
    }
  },
  methods: {
    authorize(grant: boolean) {
      window.location.href = `${server}/api/v1/auth/authorize?authorization=${this.authorization}&grant=${grant}`
    },
    async allowAuth() {
      this.authorize(true);
    },
    async denyAuth() {
      this.authorize(false);
    },
    async loadAuthorizationInfo() {
      const r = await fetch(`${server}/api/v1/auth/authorization-info?authorization=${this.authorization}`);
      switch(r.status) {
        case 200:
          interface Response {
            client_name: string,
            scopes?: string,
          }

          const json: Response = await r.json();
          this.clientName = json.client_name;
          this.scopes = json.scopes?.split(" ") ?? [];
          this.loading = false;

          break;
        default:
          break;
      }
    }
  }
});
</script>