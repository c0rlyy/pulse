<script setup lang="ts">
import { ref } from "vue";
import axios from "axios";
import { useRouter } from "vue-router";
const router = useRouter();
import SignButton from "../layout/SignButton.vue";
let inputPassword = ref("");
let confirmPassword = ref("");
const form = ref({
  username: "",
  email: "",
  password: "",
  is_private: false,
});
function checkPassword() {
  if (inputPassword.value !== confirmPassword.value) {
    alert("Passwords do not match!");
    return false;
  }

  if (!inputPassword.value.trim()) {
    alert("Password cannot be empty!");
    return false;
  }

  form.value.password = inputPassword.value;
  return true;
}

async function handleRegister() {
  if (!checkPassword()) return;
  try {
    const response = await axios.post("/api/users", form.value);
    console.log("User registered:", response.data);
    router.push({ name: "home" });
  } catch (error) {
    console.error("Registration failed:", error);
  }
}
// const handleRegister = async () => {
//   try {
//     const res = await axios.post("/api/users", form.value);
//     response.value = res.data;
//   } catch (error) {
//     console.error("Error creating user:", error);
//   }
// };

// function handleRegister() {
//   alert("Done!");
// }
</script>
<template>
  <section>
    <p class="welcome-text welcome-nameapp">Pulse!</p>
    <form @submit.prevent="handleRegister">
      <label>Login</label>
      <input v-model="form.username" />
      <label>Email</label>
      <input v-model="form.email" />
      <label>Password</label>
      <input type="password" v-model="inputPassword" />
      <label>Confirm Password</label>
      <input type="password" v-model="confirmPassword" />
      <SignButton type="submit">Register!</SignButton>
    </form>
    <SignButton @click="$router.push({ name: 'welcome' })">Choose Another Option</SignButton>
  </section>
</template>
