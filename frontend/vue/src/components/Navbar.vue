<template>
  <section>
    <router-link to="/" class="title" data-test-logo>My Todo App</router-link>
    <div class="edit-task" v-if="inOneTask">
      <form-button
        label="Edit Task"
        size="small"
        status="info"
        data-test-edit
        @click="handleEditClicked"
      />
    </div>
    <div class="nav-right">
      <div class="add-task" v-if="$store.getters.loggedIn">
        <router-link to="/add-task">
          <form-button
            label="Add Task"
            size="small"
            status="ok"
            @click="handleNavToAddTask"
          />
        </router-link>
      </div>
      <div v-if="!$store.getters.loggedIn">
        <router-link to="/create-account" class="auth" data-test-create-account
          >Create Account</router-link
        >
        <router-link to="/login" class="auth" data-test-login
          >Login</router-link
        >
      </div>
      <div v-else class="flex">
        <p data-test-welcome>Welcome, {{ $store.getters.username }}</p>
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
  },
};
</script>

<style scoped>
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
}
</style>