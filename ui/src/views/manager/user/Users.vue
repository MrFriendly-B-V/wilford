<template>
    <v-container>
        <ManagePermittedScopesDialog
            :enabled="managePermittedScopesDialogData.enabled"
            :user="managePermittedScopesDialogData.user"
            @close="managePermittedScopesDialogData.enabled = false"
        ></ManagePermittedScopesDialog>

        <v-card>
            <v-card-title>
              <v-btn
                @click="$router.back()"
                icon="mdi-arrow-left"
                density="compact"
                variant="flat"
                slim
              />
              Users
            </v-card-title>
            <v-card-subtitle>All users who have logged in via Wilford</v-card-subtitle>
            <v-card-text v-if="isManager">
                <v-data-table
                    :items="users"
                    :headers="headers">

                    <template v-slot:[`item.isAdmin`]="{ item }">
                        <v-checkbox
                            class="justify-center align-center"
                            hide-details
                            v-model="item.isAdmin"
                            :disabled="true"
                        ></v-checkbox>
                    </template>

                    <template v-slot:[`item.actions`]="{ item }">
                        <v-tooltip text="Manage permitted scopes">
                            <template v-slot:activator="{ props }">
                                <v-btn
                                    v-bind="props"
                                    :slim="true"
                                    size="small"
                                    @click="openPermittedScopesDialog(item)"
                                    icon="mdi-telescope">
                                </v-btn>
                            </template>
                        </v-tooltip>
                    </template>
                </v-data-table>
            </v-card-text>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {onMounted, Ref, ref} from "vue";
import {Token} from "@/scripts/token";
import {User} from "@/scripts/user";
import ManagePermittedScopesDialog from "@/views/manager/user/ManagePermittedScopesDialog.vue";
import {ClientInfo} from "@/scripts/clients";

let isManager = ref(false);
let users: Ref<User[]> = ref([]);

interface ManagePermittedScopesDialogData {
    enabled: boolean,
    user?: User
}

let managePermittedScopesDialogData = ref(<ManagePermittedScopesDialogData> {
    enabled: false,
    user: undefined,
})

const headers: { title: string, value: string }[] = [
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

onMounted(async () => {
    const tokenInfo = await Token.getCurrentInfo()
    isManager.value = tokenInfo.scopes.includes('wilford.manage');

    if(!isManager.value) {
        const client = await ClientInfo.getInternal();
        window.location.href = client.getAuthorizationRedirect(true);
    }

    users.value = await User.list();
})

function openPermittedScopesDialog(user: User) {
    managePermittedScopesDialogData.value.user = user;
    managePermittedScopesDialogData.value.enabled = true;
}

</script>