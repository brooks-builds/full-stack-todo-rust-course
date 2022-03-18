<template>
  <div id="app" class="app">
    <navbar
      @editTask="handleEditTask"
      @navToAddTask="handleNavToAddTask"
      @logout="handleLogout"
      @deleteTask="handleDeleteTask"
    />
    <main>
      <div :class="errorActive" data-test="error">{{ errorMessage }}</div>
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
        @toggleCompletedTask="handleToggleCompletedTask"
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
    handleToggleCompletedTask(taskId) {
      this.$store.dispatch("toggleCompletedTask", taskId);
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
        this.$store.state.errorMessage ? "error-active" : "error-not-active"
      }`;
    },
  },
  watch: {
    "$store.state.errorMessage": {
      deep: true,
      handler() {
        setTimeout(
          () => this.$store.commit("setErrorMessage", ""),
          this.$store.state.errorMessageTimeout
        );
      },
    },
  },
  mounted() {
    this.$store.dispatch("loadUser");
  },
};
</script>


<style>
.error {
  text-align: center;
  margin: 0;
  position: relative;
}

.error-active {
  animation: fade-away 30s linear;
}

.error-not-active {
  background-color: black;
}

@keyframes fade-away {
  from {
    background-color: black;
    color: black;
  }
  3% {
    background-color: indianred;
    color: white;
  }
  95% {
    background-color: indianred;
    color: white;
  }
  99% {
    background-color: black;
    color: black;
  }
  to {
    background-color: black;
    color: black;
  }
}

@media (max-width: 376px) {
  .error {
    font-size: 16px;
    padding: 10px;
    position: fixed;
    bottom: 0;
    margin-left: -10px;
    width: 100%;
  }
}
</style>
