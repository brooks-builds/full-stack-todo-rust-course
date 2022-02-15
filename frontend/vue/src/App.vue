<template>
  <div id="app" class="app">
    <navbar
      @editTask="handleEditTask"
      @navToAddTask="handleNavToAddTask"
      @logout="handleLogout"
      @deleteTask="handleDeleteTask"
    />
    <main>
      <div :class="errorActive">{{ errorMessage }}</div>
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
        @resetEditedTask="handleResetEditedTask"
        @changeSortBy="handleChangeSortBy"
        @filterSet="handleFilterSet"
        @error="handleError"
        @editCompleteToggle="handleEditCompleteToggle"
        @cancelEditMode="handleCancelEditMode"
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
    handleDeleteTask(taskId) {
      this.$store.dispatch("deleteTask", taskId);
    },
    handleResetEditedTask() {
      this.$store.dispatch("resetEditTask");
      this.$store.commit("turnOffEditTaskMode");
    },
    handleChangeSortBy(sortByValue) {
      this.$store.commit("setSelectedSortBy", sortByValue);
    },
    handleFilterSet(filterOptionValue) {
      this.$store.commit("setSelectedFilterBy", filterOptionValue);
    },
    handleError(errorMessage) {
      this.$store.commit("setErrorMessage", errorMessage);
    },
    handleEditCompleteToggle() {
      this.$store.commit("toggleEditedCompleted");
    },
    handleCancelEditMode() {
      this.$store.commit("turnOffEditTaskMode");
    },
  },
  computed: {
    errorMessage() {
      return this.$store.state.errorMessage;
    },
    errorActive() {
      return `error ${
        this.$store.state.errorMessage ? "errorActive" : "ErrorNotActive"
      }`;
    },
  },
  watch: {
    "$store.state.errorMessage": {
      deep: true,
      handler() {
        setTimeout(
          () => this.$store.commit("setErrorMessage", null),
          this.$store.state.errorMessageTimeout
        );
      },
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

.error {
  height: 4rem;
  font-size: 4rem;
  text-align: center;
  margin: 0;
  position: relative;
  animation: fade-away 1s 9s linear;
}

.errorActive {
  background-color: indianred;
}

@keyframes fade-away {
  from {
    background-color: indianred;
    color: white;
  }
  to {
    background-color: black;
    color: black;
  }
}
</style>
