<template>
    <v-container>
        <AddConstantAccessTokenDialog
            :enabled="addTokenDialogEnabled"
            @close="addConstantTokenDialogClosed"
        ></AddConstantAccessTokenDialog>

        <v-card>
            <v-card-title>
              <v-btn
                @click="$router.back()"
                icon="mdi-arrow-left"
                density="compact"
                variant="flat"
                slim
              />
              Constant Access Tokens
            </v-card-title>
            <v-card-subtitle>Manage CAT Tokens</v-card-subtitle>
            <div class="d-flex flex-row pr-3">
                <v-spacer></v-spacer>
                <v-tooltip text="Add CAT Token">
                    <template v-slot:activator="{ props }">
                        <v-btn
                            v-bind="props"
                            :slim="true"
                            size="small"
                            icon="mdi-plus"
                            @click="addTokenDialogEnabled = true"
                        ></v-btn>
                    </template>
                </v-tooltip>
            </div>
            <v-card-text v-if="isManager">
                <v-data-table
                    :items="tokens"
                    :headers="headers">

                    <template v-slot:[`item.actions`]="{ item }">
                        <v-tooltip text="Revoke token (DANGER!)">
                            <template v-slot:activator="{ props }">
                                <v-btn
                                    v-bind="props"
                                    :slim="true"
                                    size="small"
                                    icon="mdi-delete"
                                    @click="revokeToken(item)"
                                ></v-btn>
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
import {ClientInfo} from "@/scripts/clients";
import {CatToken} from "@/scripts/cat";
import AddConstantAccessTokenDialog from "@/views/manager/cat/AddConstantAccessTokenDialog.vue";

let isManager = ref(false);
let tokens: Ref<CatToken[]> = ref([]);

let addTokenDialogEnabled = ref(false);

const headers: { title: string, value: string }[] = [
    {
        title: "Name",
        value: "name"
    },
    {
        title: "Token",
        value: "token"
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

    tokens.value = await CatToken.list();
})

async function addConstantTokenDialogClosed(ok: boolean) {
    addTokenDialogEnabled.value = false;

    if(ok) {
        tokens.value = await CatToken.list();
    }
}

async function revokeToken(token: CatToken) {
    await token.revoke();
    tokens.value = await CatToken.list();
}

</script>