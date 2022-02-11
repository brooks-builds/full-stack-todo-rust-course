<template>
  <button :class="classes" :disabled="disabled" @click="handleClick">
    {{ label }}
  </button>
</template>

<script>
export default {
  props: {
    label: String,
    status: String,
    disabled: Boolean,
    size: String,
  },
  computed: {
    classes() {
      if (this.disabled) {
        return "disabled";
      }

      return `${this.status} ${this.size}`;
    },
  },
  methods: {
    handleClick() {
      this.$emit("click");
    },
  },
};
</script>

<style scoped>
button {
  height: 4rem;
  width: 51.5%;
  font-size: 2.5rem;
  margin: 0 1rem;
}

.ok {
  background-color: green;
}

.info {
  background-color: aquamarine;
}

.disabled {
  background-color: grey;
  color: black;
}

.small {
  width: auto;
}

.medium {
  width: 15%;
}
</style>