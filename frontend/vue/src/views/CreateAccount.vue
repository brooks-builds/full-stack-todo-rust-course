<template>
  <div class="create-account">
    <h1>Create Account</h1>
    <form-input label="username" type="text" v-model="username" />
    <form-input
      label="password"
      type="password"
      v-model="password"
    />
    <form-button
      label="Create Account"
      status="ok"
      :disabled="isFormValid"
      @click="handleFormSubmitted"
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
      this.$emit('createAccount');
    }
  },
  computed: {
    isFormValid() {
      return !this.$store.getters.createAccountFormValid;
    },
    password: {
      get() {
        return this.$store.state.createAccount.password;
      },
      set(password) {
        return this.$emit("passwordSet", password);
      }
    },
    username: {
      get() {
        return this.$store.state.createAccount.username;
      },
      set(newUsername) {
        this.$emit("usernameSet", newUsername);
      }
    }
  },
};
</script>

<style scoped>
.create-account {
  text-align: center;
}
</style>