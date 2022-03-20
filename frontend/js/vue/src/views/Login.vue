<template>
  <div class="login">
    <h1>Login</h1>
    <form-input
      label="username"
      type="text"
      v-model="username"
      dataTest="username"
    />
    <form-input
      label="password"
      type="password"
      v-model="password"
      dataTest="password"
    />
    <form-button
      label="Login"
      status="ok"
      :disabled="isFormValid"
      @click="handleFormSubmitted"
      data-test="submit"
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
div {
  text-align: center;
}

div > * {
  margin-bottom: 10px;
}

h1 {
  margin-bottom: 25px;
}
</style>