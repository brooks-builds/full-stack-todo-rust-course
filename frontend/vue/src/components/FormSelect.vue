<template>
  <div class="priority">
    <span class="label">{{ label }}</span>
    <select
      @input="handleInput"
      :value="defaultOption.value"
      :data-test="dataTest"
    >
      <option
        v-for="(option, index) in options"
        :value="option.value"
        v-bind:key="index"
      >
        {{ option.label }}
      </option>
    </select>
  </div>
</template>

<script>
export default {
  props: {
    // An option looks like {value: 'A', label: 'A', default: false}
    options: Array,
    label: String,
    dataTest: String,
  },
  methods: {
    handleInput(event) {
      const value = event.target.value;
      this.$emit("input", value);
    },
  },
  computed: {
    defaultOption() {
      return this.options.find((option) => option.default);
    },
  },
};
</script>

<style scoped>
.label {
  margin-right: 10px;
}
/* .priority {
  margin: 2rem;
}

.priority > select {
  font-size: 3rem;
}

.priority > .label {
  margin: 0 1rem;
} */
</style>