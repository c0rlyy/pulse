<template>
  <input class="message-card" type="text" :value="modelValue" @input="$emit('update:modelValue', $event.target.value)" />
</template>

<script setup lang="ts">
// Zdefiniuj props z obiektem konfiguracyjnym
const props = defineProps({
  modelValue: {
    type: String,
    default: "",
  },
});
</script>

<style scoped>
.message-card {
  width: 100%;
  padding: 5px;
  /* margin: 10px; */
  border: 1px solid #ccc;
  border-radius: 5px;
  box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.1);
  outline: none;
  resize: vertical;
  overflow-y: auto;
  font-size: 16px;
  line-height: 1.5;
}
</style>
