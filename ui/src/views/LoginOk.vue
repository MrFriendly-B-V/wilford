<template>
    <v-container>
        <v-banner :text="errorBanner" v-if="errorBanner">
            <template v-slot:actions>
                <v-btn
                    v-if="internalClient"
                    :href="internalClient!.getAuthorizationRedirect()">
                    Try again
                </v-btn>
            </template>
        </v-banner>

        <v-card v-if="loading">
            <v-card-title>Almost there..</v-card-title>
            <v-card-subtitle>Finishing up logging in</v-card-subtitle>
            <v-card-text>
                <v-progress-circular indeterminate></v-progress-circular>
            </v-card-text>
        </v-card>

        <v-card v-else>
            <v-card-title>Logged in</v-card-title>
            <v-card-subtitle v-if="!errorBanner">Welcome to Wilford.</v-card-subtitle>

            <v-card-actions>

            </v-card-actions>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">

import {onMounted, Ref, ref} from "vue";
import {useRoute, useRouter} from "vue-router";
import {ClientInfo} from "@/scripts/clients";

const router = useRouter();
const route = useRoute();

let errorBanner: Ref<string | undefined> = ref(undefined);
let loading = ref(true);
let internalClient: Ref<ClientInfo | null> = ref(null);

onMounted(async () => {
    await router.isReady();
    setAccessToken();
})

function setAccessToken() {
    const params = parseQuery(route.hash);
    if(params.has('access_token')) {
        window.localStorage.setItem('access_token', params.get('access_token')!);
    } else {
        errorBanner.value = "Could not log in."
    }

    loading.value = false;
    router.push('/me');
}

function parseQuery(queryString: string): Map<string, string> {
    let query: Map<string, string> = new Map();
    let pairs = ((queryString[0] === '?' || queryString[0] === '#') ? queryString.substring(1) : queryString).split('&');
    for (let i = 0; i < pairs.length; i++) {
        let pair = pairs[i].split('=');
        query.set(decodeURIComponent(pair[0]), decodeURIComponent(pair[1] || ''));
    }
    return query;
}

</script>