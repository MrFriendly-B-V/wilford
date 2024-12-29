<template>
    <v-container>
        <CreateClientDialog
            :enabled="isCreateClientDialogEnabled"
            @close="createClientDialogClosed"
        ></CreateClientDialog>

        <v-card>
            <v-card-title>
              <GoBackBtn/>
              OAuth2 Clients
            </v-card-title>
            <v-card-subtitle>Manage OAuth2 clients</v-card-subtitle>

            <v-card-text v-if="isManager">
                <div class="d-flex flex-row pr-3">
                    <v-spacer></v-spacer>
                    <v-tooltip text="Add OAuth2 Client">
                        <template v-slot:activator="{ props }">
                            <v-btn
                                v-bind="props"
                                :slim="true"
                                size="small"
                                icon="mdi-plus"
                                @click="isCreateClientDialogEnabled = true"
                            ></v-btn>
                        </template>
                    </v-tooltip>
                </div>

                <v-data-table
                    :items="clients"
                    :headers="headers">

                    <template v-slot:[`item.actions`]="{ item }">
                        <v-tooltip text="Delete client">
                            <template v-slot:activator="{ props }">
                                <v-btn
                                    v-bind="props"
                                    :slim="true"
                                    size="small"
                                    @click="deleteClient(item)"
                                    icon="mdi-delete">
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

import {onMounted, ref, Ref} from "vue";
import {ClientInfo} from "@/scripts/clients";
import {Token} from "@/scripts/token";
import CreateClientDialog from "@/views/manager/client/CreateClientDialog.vue";
import GoBackBtn from "@/components/buttons/GoBackBtn.vue";

const headers: { title: string, value: string }[] = [
    {
        title: "Name",
        value: "name"
    },
    {
        title: "Redirect URI",
        value: "redirectUri"
    },
    {
        title: "Client ID",
        value: "clientId"
    },
    {
        title: "Client Secret",
        value: "clientSecret"
    },
    {
        title: "Actions",
        value: "actions"
    }
]

let clients: Ref<ClientInfo[]> = ref([]);
let isManager = ref(false);
let isCreateClientDialogEnabled = ref(false);

onMounted(async () => {
    const tokenInfo = await Token.getCurrentInfo()
    isManager.value = tokenInfo.scopes.includes('wilford.manage');

    if(!isManager.value) {
        const client = await ClientInfo.getInternal();
        window.location.href = client.getAuthorizationRedirect(true);
    }

    await loadClients();
})

async function loadClients() {
    clients.value = await ClientInfo.list();
}

async function deleteClient(client: ClientInfo) {
    await client.remove();
    await loadClients();
}

function createClientDialogClosed(ok: boolean) {
    isCreateClientDialogEnabled.value = false;
    if(ok) {
        loadClients();
    }
}

</script>