<template>
  <div class="form-input">
    <input
      :type="type"
      :name="label"
      :placeholder="label"
      :value="value"
      @input="handleChange"
    />
  </div>
</template>

<script>
export default {
  props: {
    label: String,
    value: String,
    type: String,
  },
  methods: {
    handleChange(event) {
      const input = event.target.value;
      this.$emit("input", input);
    },
  },
};
</script>

<style scoped>
input {
  height: 3rem;
  font-size: 3rem;
  padding: 0.5rem;
  margin: 1rem 0;
  width: 50%;
  background-color: blanchedalmond;
}

.label {
  text-align: left;
}
</style>