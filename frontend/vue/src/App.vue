<template>
  <div id="app" class="app">
    <navbar
      @editTask="handleEditTask"
      @navToAddTask="handleNavToAddTask"
      @logout="handleLogout"
    />
    <main>
      <router-view
        @usernameSet="handleCreateAccountUsernameSet"
        @passwordSet="handleCreateAccountPasswordSet"
        @createAccount="handleCreateAccount"
        @login="handleLogin"
        @editTitle="handleEditTitle"
        @editDescription="handleEditDescription"
        @saveTask="handleSaveTask"
        @editPriority="handleEditPriority"
        @createTask="handleCreateTask"
        @completedTask="handleCompletedTask"
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
    handleEditPriority(priority) {
      this.$store.commit("setEditPriority", priority);
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
    handleNavToAddTask() {
      this.$store.dispatch("resetEditTask");
    },
    handleCreateTask() {
      this.$store.dispatch("createTask");
    },
    handleCompletedTask(taskId) {
      this.$store.dispatch("completeTask", taskId);
    },
    handleLogout() {
      this.$store.dispatch("logout");
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
