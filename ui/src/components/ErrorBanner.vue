<template>
  <MaterialBanner
    title="Error"
    type="error"
    icon="mdi-alert-circle-outline"
    :text="modelValue"
    @close="$emit('update', undefined)"
  />
</template>

<script lang="ts">
import {defineComponent} from "vue"
import MaterialBanner from "@/components/MaterialBanner.vue";

export default defineComponent({
  components: {MaterialBanner},
  props: {
    modelValue: String
  }
})
</script>