<template>
  <v-container>
    <ErrorBanner v-model="error"/>

    <ManagePermittedScopesDialog
      :enabled="dialog.permittedScopes.enabled"
      :user="dialog.permittedScopes.user"
      @close="dialog.permittedScopes.enabled = false"
    ></ManagePermittedScopesDialog>

    <v-card>
      <v-card-title>
        <GoBackBtn/>
        Users
      </v-card-title>
      <v-card-subtitle>Manage users</v-card-subtitle>
      <v-card-text>
        <v-data-table
          :headers="headers"
          :items="users">

          <template v-slot:[`item.isAdmin`]="{ item }">
            <v-checkbox
              v-model="item.isAdmin"
              :disabled="true"
              class="justify-center align-center"
              hide-details
            ></v-checkbox>
          </template>

          <template v-slot:[`item.actions`]="{ item }">
            <v-tooltip text="Manage permitted scopes">
              <template v-slot:activator="{ props }">
                <v-btn
                  :slim="true"
                  icon="mdi-telescope"
                  size="small"
                  v-bind="props"
                  @click="openPermittedScopesDialog(item)">
                </v-btn>
              </template>
            </v-tooltip>
          </template>
        </v-data-table>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import {User} from "@/scripts/user";
import {DataTableHeaders} from "@/main";
import ManagePermittedScopesDialog from "@/views/manager/user/ManagePermittedScopesDialog.vue";
import ErrorBanner from "@/components/banners/ErrorBanner.vue";
import GoBackBtn from "@/components/buttons/GoBackBtn.vue";

interface Data {
  error?: string;
  loading: boolean;
  users: User[];
  dialog: {
    permittedScopes: {
      enabled: boolean,
      user?: User,
    }
  },
  headers: DataTableHeaders,
}

export default defineComponent({
  components: {GoBackBtn, ErrorBanner, ManagePermittedScopesDialog},
  data(): Data {
    return {
      error: undefined,
      loading: true,
      users: [],
      dialog: {
        permittedScopes: {
          enabled: false,
          user: undefined,
        }
      },
      headers: [
        {
          title: "Name",
          value: "name"
        },
        {
          title: "EspoCRM Admin",
          value: "isAdmin"
        },
        {
          title: "Actions",
          value: "actions"
        }
      ]
    }
  },
  async mounted() {
    await this.loadUsers();
  },
  methods: {
    async loadUsers() {
      const result = await User.list();
      if (result.isOk()) {
        this.users = result.unwrap();
      } else {
        this.error = result.unwrapErr().message;
      }
    },
    openPermittedScopesDialog(user: User) {
      this.dialog.permittedScopes.user = user;
      this.dialog.permittedScopes.enabled = true;
    }
  }
})
</script>