<template>
  <div id="app" class="app">
    <navbar />
    <main>
      <router-view
        @usernameSet="handleCreateAccountUsernameSet"
        @passwordSet="handleCreateAccountPasswordSet"
        @createAccount="handleCreateAccount"
        @login="handleLogin"
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
    handleCreateAccount() {
      this.$store.dispatch("createAccount");
    },

    handleCreateAccountUsernameSet(username) {
      this.$store.commit("setAccountFormUsername", username);
    },
    handleCreateAccountPasswordSet(password) {
      this.$store.commit("setAccountFormPassword", password);
    },
    handleLogin() {
      this.$store.dispatch("login");
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
