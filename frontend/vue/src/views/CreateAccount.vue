<template>
  <div class="create-account">
    <h1>Create Account</h1>
    <form-input label="username" type="text" @input="handleUsernameChanged" />
    <form-input
      label="password"
      type="password"
      @input="handlePasswordChanged"
    />
    <form-button
      label="Create Account"
      status="ok"
      :disabled="isFormValid"
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
    handleUsernameChanged(username) {
      this.$emit("usernameSet", username);
    },

    handlePasswordChanged(password) {
      this.$emit("passwordSet", password);
    },
  },
  computed: {
    isFormValid() {
      return !this.$store.getters.createAccountFormValid;
    },
  },
};
</script>

<style scoped>
.create-account {
  text-align: center;
}
</style>