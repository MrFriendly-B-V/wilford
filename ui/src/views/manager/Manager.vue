<template>
  <v-container>
    <v-card>
      <v-card-title>
        <v-btn
          @click="$router.back()"
          icon="mdi-arrow-left"
          density="compact"
          variant="flat"
          slim
        />
        Manage Wilford
      </v-card-title>
      <v-card-text>
        <!-- Links to manager pages -->
        <v-list>
          <v-list-item v-for="item in adminLinks" :key="item.to">
            <v-btn
              :flat="true"
              prepend-icon="mdi-link"
              :to="item.to">
              {{ item.name }}
            </v-btn>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue"
import { defineComponent } from "vue";

interface Data {
  loading: boolean,
  adminLinks: {
    name: string,
    to: string
  }[]
}

export default defineComponent({
  data(): Data {
    return {
      loading: true,
      adminLinks: [
        {
          name: "Manage users",
          to: "/manager/users"
        },
        {
          name: "Manage CAT tokens",
          to: "/manager/cat"
        },
        {
          name: "Manage OAuth2 clients",
          to: "/manager/clients"
        }
      ]
    }
  }
})
</script>