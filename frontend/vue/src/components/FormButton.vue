<template>
  <button :class="calculateStatus" :disabled="disabled">{{ label }}</button>
</template>

<script>
export default {
  props: {
    label: String,
    status: String,
    disabled: Boolean,
  },
  computed: {
    calculateStatus() {
      if (this.disabled) {
        return "disabled";
      }

      return this.status;
    },
  },
};
</script>

<style scoped>
button {
  height: 4rem;
  width: 51.5%;
  font-size: 2.5rem;
}

.ok {
  background-color: green;
}

.disabled {
  background-color: grey;
  color: black;
}
</style>