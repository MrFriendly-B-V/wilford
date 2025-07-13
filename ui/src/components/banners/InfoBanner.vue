<template>
  <MaterialBanner
    title="Info"
    type="info"
    icon="mdi-information-outline"
    :text="modelValue"
    @close="$emit('update', undefined)"
  />
</template>

<script lang="ts">
import {defineComponent} from "vue"
import MaterialBanner from "@/components/banners/MaterialBanner.vue";

export default defineComponent({
  components: {MaterialBanner},
  props: {
    modelValue: String
  }
})
</script>