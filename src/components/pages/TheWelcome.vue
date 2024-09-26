<script setup lang="ts">
import { ref, watch } from "vue";
import { useRouter, useRoute } from "vue-router";
const router = useRouter();
const route = useRoute();
import SignButton from "../layout/SignButton.vue";
let userChoice = ref<string>("no-choice");

const goTo = (route) => {
  userChoice.value = route;
  router.push({ name: route });
};

watch(route, (newRoute) => {
  if (newRoute.name === "welcome") {
    userChoice.value = "no-choice";
  }
});
</script>

<!-- <template>
  <transition name="fade">
    <section v-if="userChoice === 'no-choice'">
      <NoOption />
      <p class="welcome-text">Welcome in <span class="welcome-nameapp">Pulse!</span></p>
      <p class="welcome-description">Pulse get your messages in a heartbeat</p>
      <div class="buttons-container">
        <SignButton @click="handleUserChoice('login')">Log In to Pulse!</SignButton>
        <SignButton @click="handleUserChoice('register')">Create Your Pulse Account!</SignButton>
      </div>

      <SignButton @click="handleUserChoice('anonymous')">Chat Anonymously</SignButton>
    </section>
  </transition>
  <transition name="fade">
    <section v-if="userChoice === 'login'">
      <p class="welcome-text welcome-nameapp">Pulse!</p>
      <label>Login</label>
      <input />
      <label>Password</label>
      <input />
      <SignButton @click="">Log In!</SignButton>
      <SignButton @click="handleUserChoice('no-choice')">Choose Another Option</SignButton>
    </section>
  </transition>
  <transition name="fade">
    <section v-if="userChoice === 'register'">
      <p class="welcome-text welcome-nameapp">Pulse!</p>
      <label>Login</label>
      <input />
      <label>Password</label>
      <input />
      <label>Confirm Password</label>
      <input />
      <SignButton @click="">Register!</SignButton>
      <SignButton @click="handleUserChoice('no-choice')">Choose Another Option</SignButton>
    </section>
  </transition>
  <transition name="fade">
    <section v-if="userChoice === 'anonymous'">
      <p class="welcome-text welcome-nameapp">Pulse!</p>
      <div class="buttons-container">
        <SignButton>Create an Anonymous Room</SignButton>
        <SignButton>Join an Existing Room</SignButton>
      </div>
      <SignButton @click="handleUserChoice('no-choice')">Choose Another Option</SignButton>
    </section>
  </transition>
</template> -->

<template>
  <div>
    <transition name="fade" v-if="userChoice === 'no-choice'">
      <section>
        <p class="welcome-text">Welcome to <span class="welcome-nameapp">Pulse!</span></p>
        <p class="welcome-description">Pulse lets you chat in a heartbeat!</p>
        <div class="buttons-container">
          <SignButton @click="goTo('login')">Log In to Pulse!</SignButton>
          <SignButton @click="goTo('register')">Create Your Pulse Account!</SignButton>
        </div>
        <SignButton @click="goTo('anonymous')">Chat Anonymously</SignButton>
      </section>
    </transition>
    <transition name="fade" v-else>
      <router-view />
    </transition>
  </div>
</template>

<style scoped>
* {
  padding: 0;
  margin: 0;
}
section {
  box-sizing: content-box;
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  color: white;
  background-color: #141414;
  padding: 30px;
  border-radius: 30px;
  width: 100%;
  height: 50vh;
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
