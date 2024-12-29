<template>
    <v-dialog
        :model-value="enabled"
        max-width="1200"
        :persistent="true">
        <v-card>
            <v-card-title v-if="user">Add scope to {{ user.name }}</v-card-title>
            <v-card-text>
                <v-text-field
                    v-model="scopeName"
                    limit="64"
                ></v-text-field>
            </v-card-text>
            <v-card-actions>
                <v-btn @click="$emit('close', false)">
                    Cancel
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    :disabled="!scopeName || scopeName.length == 0"
                    @click="addScope">
                    Add
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import {User} from "@/scripts/user";
import {Ref, ref} from "vue";

const props = defineProps({
    enabled: Boolean,
    user: User,
})

const emit = defineEmits<{
    close: [ok: boolean]
}>();

let scopeName: Ref<string | null> = ref(null);

async function addScope() {
    await props.user!.addPermittedScope(scopeName.value!)
    emit('close', true);
}

</script>