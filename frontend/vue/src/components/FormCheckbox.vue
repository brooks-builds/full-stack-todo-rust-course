<template>
  <span>
    <input type="checkbox" :id="id" :checked="checked" />
    <label :for="id"></label>
  </span>
</template>

<script>
export default {
  props: {
    id: String,
    checked: Boolean,
  },
};
</script>

<style scoped>
input[type="checkbox"] + label::before {
  content: "\a0";
  display: inline-block;
  vertical-align: 0.2rem;
  width: 3rem;
  height: 3rem;
  margin-right: 0.2rem;
  border-radius: 0.2rem;
  background: silver;
  text-indent: 0.15rem;
  line-height: 0.65;
}

input[type="checkbox"]:checked + label::before {
  content: "\2713";
  background: green;
}

input[type="checkbox"] {
  position: absolute;
  clip: rect(0, 0, 0, 0);
}
</style>