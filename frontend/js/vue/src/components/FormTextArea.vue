<template>
  <textarea cols="30" rows="10" :value="value" @input="handleChange"></textarea>
</template>

<script>
export default {
  props: {
    value: String,
  },
  methods: {
    handleChange(event) {
      const text = event.target.value;
      this.$emit("input", text);
    },
  },
};
</script>

<style scoped>
textarea {
  background-color: bisque;
}
</style>