<template>

</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue';
import {User} from "@/scripts/user";

export default defineComponent({
  props: {
    user: {
      type: Object as PropType<User>,
      required: true,
    }
  }
})
</script>