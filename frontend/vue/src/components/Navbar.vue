<template>
  <section>
    <router-link to="/" class="title" data-test-logo>My Todo App</router-link>
    <div v-if="!$store.getters.loggedIn">
      <router-link to="/create-account" class="auth" data-test-create-account
        >Create Account</router-link
      >
      <router-link to="/login" class="auth">Login</router-link>
    </div>
    <div v-else>
      <p data-test-welcome>Welcome, {{ $store.getters.username }}</p>
    </div>
  </section>
</template>

<style scoped>
section {
  border-bottom: 1px solid bisque;
  padding: 0.5rem 2rem;
  display: flex;
  justify-content: space-between;
}

.title {
  color: bisque;
  text-decoration: none;
}

.auth {
  margin-right: 1rem;
  color: cadetblue;
}

.auth:last-child {
  margin-right: 0;
}
</style>