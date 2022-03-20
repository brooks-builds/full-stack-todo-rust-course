<template>
  <section class="flex">
    <router-link to="/" class="title" data-test="logo">My Todo App</router-link>
    <div class="edit-task" v-if="inOneTask">
      <form-button
        label="Edit Task"
        size="small"
        status="info"
        data-test="edit"
        @click="handleEditClicked"
      />
      <form-button
        label="Delete Task"
        size="small"
        status="warning"
        @click="handleDeleteTask"
        data-test="delete"
      />
    </div>
    <div class="flex">
      <div class="add-task" v-if="$store.getters.loggedIn">
        <router-link to="/add-task" class="button" data-test="add-task">
          Add Task
        </router-link>
      </div>
      <div v-if="!$store.getters.loggedIn">
        <router-link
          to="/create-account"
          class="button"
          data-test="create-account"
          >Create Account</router-link
        >
        <router-link to="/login" class="button" data-test="login"
          >Login</router-link
        >
      </div>
      <div v-else class="flex">
        <p data-test="welcome">Welcome, {{ $store.getters.username }}</p>
        <form-button
          label="log out"
          size="small"
          status="warning"
          @click="handleLogout"
        />
      </div>
    </div>
  </section>
</template>

<script>
import FormButton from "./FormButton.vue";

export default {
  components: {
    FormButton,
  },
  computed: {
    inOneTask() {
      return (
        this.$route.name == "one task" && !this.$store.state.editingOneTask
      );
    },
  },
  methods: {
    handleEditClicked() {
      this.$emit("editTask");
    },
    handleNavToAddTask() {
      this.$emit("navToAddTask");
    },
    handleLogout() {
      this.$emit("logout");
    },
    handleDeleteTask() {
      const taskId = this.$route.params.taskId;
      if (!taskId) throw new Error("could not find task id to delete");
      this.$emit("deleteTask", taskId);
    },
  },
};
</script>

<style scoped>
section {
  justify-content: space-between;
  border-bottom: 1px solid white;
  padding-bottom: 10px;
}

section * {
  margin-right: 10px;
}

section *:last-child {
  margin-right: 0;
}

.flex {
  display: flex;
  align-items: center;
}

a {
  text-decoration: none;
}

.title {
  color: bisque;
}

.title:hover {
  color: white;
}

.button {
  background-color: cadetblue;
  padding: 15px;
  margin-right: 10px;
  display: inline-block;
  border-radius: 10px;
  line-height: 20px;
  color: black;
}

.button:hover {
  color: white;
}

@media (max-width: 812px) {
  .button {
    padding: 5px;
  }
}

@media (max-width: 375px) {
  .button {
    padding: 2px;
  }
}

/* @import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:ital@1&display=swap");

section {
  border-bottom: 1px solid bisque;
}

section,
.nav-right {
  padding: 0.5rem 2rem;
  display: flex;
  justify-content: space-between;
}

.title {
  color: bisque;
  text-decoration: none;
  font-family: "IBM Plex Sans", sans-serif;
}

.auth {
  margin-right: 1rem;
  color: cadetblue;
}

.auth:last-child {
  margin-right: 0;
}

.flex {
  display: flex;
} */
</style>