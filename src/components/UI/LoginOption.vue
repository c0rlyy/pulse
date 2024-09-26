<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
import SignButton from "../layout/SignButton.vue";
import MessageCard from "../layout/MessageCard.vue";
let userLogin = ref("");
let userPassword = ref("");
</script>
<template>
  <section>
    <p class="welcome-text welcome-nameapp">Pulse!</p>
    <label>Login</label>
    <!-- <input v-model="userLogin" /> -->
    <MessageCard v-model="userLogin" />
    <label>Password</label>
    <!-- <input type="password" v-model="userPassword" /> -->
    <MessageCard v-model="userPassword" type="password" />
    <SignButton @click="console.log(userLogin)">Log In!</SignButton>
    <SignButton @click="$router.push({ name: 'welcome' })">Choose Another Option</SignButton>
  </section>
</template>
