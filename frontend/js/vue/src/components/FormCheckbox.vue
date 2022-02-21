<template>
  <span>
    <input
      type="checkbox"
      :id="id"
      :checked="checked"
      :data-test="dataTest"
      @input="handleChecked"
    />
    <label :for="id"></label>
  </span>
</template>

<script>
export default {
  props: {
    id: String,
    checked: Boolean,
    dataTest: String,
  },
  methods: {
    handleChecked() {
      this.$emit("checked");
    },
  },
};
</script>

<style scoped>
input[type="checkbox"] + label::before {
  content: "\a0";
  display: inline-block;
  /* vertical-align: 0.2rem; */
  width: 50px;
  height: 50px;
  margin-right: 10px;
  border-radius: 10px;
  background: silver;
  text-indent: 5px;
  line-height: 50px;
}

input[type="checkbox"]:checked + label::before {
  content: "\2713";
  background: green;
}

input[type="checkbox"] {
  position: absolute;
  clip: rect(0, 0, 0, 0);
}

@media (max-width: 376px) {
  input[type="checkbox"] + label::before {
    width: 25px;
    height: 25px;
    line-height: 25px;
    border-radius: 5px;
  }
}
</style>