<template>
  <v-alert
    v-if="text != null"
    class="mt-2"
    :icon="icon"
    :title="title"
    :text="text"
    :type="type1"
    closable
    @click:close="$emit('close')"
  />
</template>

<script lang="ts">

import {defineComponent} from "vue";

export default defineComponent({
  props: {
    icon: {
      type: String,
      required: true,
    },
    title: {
      type: String,
      required: true,
    },
    type: {
      type: String,
      required: true,
    },
    text: {
      type: String,
      required: false,
      default: undefined,
    }
  },
  emits: [
    'close'
  ],
  computed: {
    type1(): 'info' | 'warning' | 'error' | 'success' | undefined {
      return this.type as 'info' | 'warning' | 'error' | 'success' | undefined;
    }
  },
  watch: {
    text(val?: string) {
      if(val) {
        setTimeout(() => this.$emit('close'),  5000);
      }
    }
  }
})
</script>