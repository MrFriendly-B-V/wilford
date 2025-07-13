<template>
  <v-container>
    <h3 class="mt-3">Change Email</h3>
    <div>
      <ChangeEmail :user="user" />
    </div>

    <h3 class="mt-3">Change name</h3>
    <div>
      <ChangeName :user="user" />
    </div>
  </v-container>
</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue';
import {User} from "@/scripts/user";
import ChangeName from "@/components/user/yourInformation/ChangeName.vue";
import ChangeEmail from "@/components/user/yourInformation/ChangeEmail.vue";

export default defineComponent({
  components: {ChangeEmail, ChangeName},
  props: {
    user: {
      type: Object as PropType<User>,
      required: true,
    }
  }
})
</script>