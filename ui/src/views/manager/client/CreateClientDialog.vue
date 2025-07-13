<template>
    <v-dialog
        :model-value="enabled"
        max-width="1200"
        :persistent="true">
        <v-card :flat="true">
            <v-card-title>Create OAuth2 Client</v-card-title>
            <v-card-text>
                <v-form v-model="valid">
                    <v-text-field
                        v-model="name"
                        label="Client name"
                        :rules="requiredRules"
                    ></v-text-field>
                    <v-text-field
                        v-model="redirectUri"
                        label="Redirect URI"
                        :rules="requiredRules"
                    ></v-text-field>
                </v-form>
            </v-card-text>
            <v-card-actions>
                <v-btn @click="$emit('close', false)">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    @click="createClient"
                    :disabled="!valid">
                    Save
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">

import {Ref, ref} from "vue";
import {ClientInfo} from "@/scripts/clients";

defineProps({
    enabled: Boolean,
});

const emit = defineEmits<{
    close: [ok: boolean]
}>();

const requiredRules = <((v: string | undefined) => string)[]> [
    v => !!v || "Required"
];

let valid = ref(true);
let name: Ref<string | null> = ref(null);
let redirectUri: Ref<string | null> = ref(null);

async function createClient() {
    await ClientInfo.new(name.value!, redirectUri.value!);
    emit('close', true);
}

</script>