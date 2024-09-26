<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
import SignButton from "../layout/SignButton.vue";
</script>
<template>
  <section>
    <p class="welcome-text welcome-nameapp">Pulse!</p>
    <div class="buttons-container">
      <SignButton>Create an Anonymous Room</SignButton>
      <SignButton>Join an Existing Room</SignButton>
    </div>
    <SignButton @click="$router.push({ name: 'welcome' })">Choose Another Option</SignButton>
  </section>
</template>
