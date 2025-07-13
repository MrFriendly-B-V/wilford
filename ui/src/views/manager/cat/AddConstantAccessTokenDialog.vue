<template>
    <v-dialog
        :model-value="enabled"
        max-width="1200"
        :persistent="true">
        <v-card>
            <v-card-title>Add CAT token</v-card-title>
            <v-card-text>
                <v-text-field
                    v-model="name"
                    limit="64"
                ></v-text-field>
            </v-card-text>
            <v-card-actions>
                <v-btn @click="$emit('close', false)">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    :disabled="!name || name.length == 0"
                    @click="addToken">
                    Add
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import {Ref, ref} from "vue";
import {CatToken} from "@/scripts/cat";

const _ = defineProps({
    enabled: Boolean,
})

const emit = defineEmits<{
    close: [ok: boolean]
}>();

let name: Ref<string | null> = ref(null);

async function addToken() {
    await CatToken.create(name.value!);
    emit('close', true);
}

</script>