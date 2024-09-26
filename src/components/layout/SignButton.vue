<template>
  <button><slot></slot></button>
</template>

<style scoped>
button {
  padding: 10px;
  font-size: 1rem;
  font-family: "Roboto";
  border-radius: 10px;
  background-color: #f47bcd;
  border: none;
  transition: 0.5s;
  cursor: pointer;
  margin: 10px;
}
button:hover {
  border-color: 1px solid white;
  color: white;
  transform: scale(1.1);
}
</style>
