<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
import SignButton from "../layout/SignButton.vue";
</script>

<template>
  <div class="no-option">
    <p class="welcome-text">Welcome in <span class="welcome-nameapp">Pulse!</span></p>
    <p class="welcome-description">Pulse get your messages in a heartbeat</p>
    <div class="buttons-container">
      <SignButton @click="handleUserChoice('login')">Log In to Pulse!</SignButton>
      <SignButton @click="handleUserChoice('register')">Create Your Pulse Account!</SignButton>
    </div>

    <SignButton @click="handleUserChoice('anonymous')">Chat Anonymously</SignButton>
  </div>
</template>

<style scoped>
* {
  padding: 0;
  margin: 0;
}
.no-option {
}
.welcome-text {
  font-size: 3rem;
  font-weight: bold;
}
.welcome-nameapp {
  color: #f47bcd;
  /* color: white; */
  font-family: "Playwrite CU", cursive;
}
.welcome-description {
  font-size: 1.1rem;
  margin: 30px 0 40px 0;
  font-weight: bold;
  font-family: "Playwrite CU", cursive;
  /* color: #f47bcd; */
  color: white;
}
.buttons-container {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
