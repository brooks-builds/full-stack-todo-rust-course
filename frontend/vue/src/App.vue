<template>
  <div id="app" class="app">
    <navbar />
    <main>
      <router-view
        @usernameSet="handleCreateAccountUsernameSet"
        @passwordSet="handleCreateAccountPasswordSet"
      />
    </main>
  </div>
</template>

<script>
import navbar from "./components/Navbar.vue";

export default {
  components: {
    navbar,
  },
  methods: {
    handleCreateAccountUsernameSet(username) {
      this.$store.commit("setCreateAccountUsername", username);
    },
    handleCreateAccountPasswordSet(password) {
      this.$store.commit("setCreateAccountPassword", password);
    },
  },
};
</script>


<style>
body {
  background-color: black;
  color: bisque;
  font-size: 3rem;
}

main {
  margin: 1rem;
}
</style>
