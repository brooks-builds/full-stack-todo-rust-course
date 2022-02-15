<template>
  <div class="login">
    <h1>Login</h1>
    <form-input
      label="username"
      type="text"
      v-model="username"
      data-test-username
    />
    <form-input
      label="password"
      type="password"
      v-model="password"
      data-test-password
    />
    <form-button
      label="Login"
      status="ok"
      :disabled="isFormValid"
      @click="handleFormSubmitted"
      data-test-submit
    ></form-button>
  </div>
</template>

<script>
import formInput from "../components/FormInput.vue";
import formButton from "../components/FormButton.vue";
export default {
  components: {
    formInput,
    formButton,
  },
  methods: {
    handleFormSubmitted() {
      this.$emit("login");
    },
  },
  computed: {
    isFormValid() {
      return !this.$store.getters.accountFormValid;
    },
    password: {
      get() {
        return this.$store.state.accountForm.password;
      },
      set(password) {
        return this.$emit("passwordSet", password);
      },
    },
    username: {
      get() {
        return this.$store.state.accountForm.username;
      },
      set(newUsername) {
        this.$emit("usernameSet", newUsername);
      },
    },
  },
};
</script>

<style scoped>
@import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:ital@1&display=swap");

h1 {
  font-family: "IBM Plex Sans", sans-serif;
}

.login {
  text-align: center;
}
</style>