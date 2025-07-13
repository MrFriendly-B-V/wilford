<template>
    <v-dialog
        :model-value="enabled"
        max-width="1200"
        :persistent="true">
        <AddPermittedScopeDialog
            :enabled="addScopeDialogEnabled"
            :user="user"
            @close="addScopeDialogClosed"
        ></AddPermittedScopeDialog>

        <v-card :flat="true">
            <v-card-title v-if="user">Manage permitted scopes for {{ user!.name }}</v-card-title>
            <div class="d-flex flex-row pr-3">
                <v-spacer></v-spacer>
                <v-tooltip text="Add scope">
                    <template v-slot:activator="{ props }">
                        <v-btn
                            v-bind="props"
                            :slim="true"
                            size="small"
                            icon="mdi-plus"
                            @click="addScopeDialogEnabled = true"
                        ></v-btn>
                    </template>
                </v-tooltip>
            </div>
            <v-card-text>
                <v-data-table
                    :items="scopes"
                    :headers="headers">

                    <template v-slot:[`item.actions`]="{ item }">
                        <v-tooltip text="Delete scope">
                            <template v-slot:activator="{ props }">
                                <v-btn
                                    v-bind="props"
                                    :slim="true"
                                    size="small"
                                    icon="mdi-delete"
                                    @click="deleteScope(item.scope)"
                                ></v-btn>
                            </template>
                        </v-tooltip>
                    </template>
                </v-data-table>
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn @click="$emit('close')">
                    Close
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">

import {User} from "@/scripts/user";
import {onMounted, ref, Ref, watch} from "vue";
import AddPermittedScopeDialog from "@/views/manager/user/AddPermittedScopeDialog.vue";

const props = defineProps({
    enabled: Boolean,
    user: User,
});

const _ = defineEmits<{
    close: []
}>();

interface Scope {
    scope: string,
}

const headers: { title: string, value: string }[] = [
    {
        title: "Scope name",
        value: "scope"
    },
    {
        title: "Actions",
        value: "actions"
    }
];

let scopes: Ref<Scope[]> = ref([]);
let addScopeDialogEnabled = ref(false);

watch(() => props.enabled, async() => {
    if(props.enabled && props.user) {
        await loadScopes();
    }
})

onMounted(async () => {
    if(props.user) {
        await loadScopes();
    }
})

async function loadScopes() {
    scopes.value = (await props.user!.listPermittedScopes()).map(f => <Scope> { scope: f });
}

async function deleteScope(name: string) {
    await props.user?.deletePermittedScope(name);
    await loadScopes();
}

async function addScopeDialogClosed(ok: boolean) {
    addScopeDialogEnabled.value = false;
    if(ok) {
        await loadScopes();
    }
}

</script>