<template>
  <div id="app" class="app">
    <navbar @editTask="handleEditTask" />
    <main>
      <router-view
        @usernameSet="handleCreateAccountUsernameSet"
        @passwordSet="handleCreateAccountPasswordSet"
        @createAccount="handleCreateAccount"
        @login="handleLogin"
        @editTitle="handleEditTitle"
        @editDescription="handleEditDescription"
        @saveTask="handleSaveTask"
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
    handleEditTask() {
      this.$store.dispatch("switchToEditMode", this.$route.params.taskId);
    },
    handleEditDescription(newTaskDescription) {
      this.$store.commit("setEditTaskDescription", newTaskDescription);
    },
    handleEditTitle(taskTitle) {
      this.$store.commit("setEditTaskTitle", taskTitle);
    },
    handleLogin() {
      this.$store.dispatch("login");
    },
    handleSaveTask() {
      this.$store.dispatch("saveTask");
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
