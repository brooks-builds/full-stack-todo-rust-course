<template>
  <button :class="classes" :disabled="disabled" @click="handleClick" :data-test="dataTest">
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
    dataTest: String,
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
  border: none;
  border-radius: 10px;
  font-size: 40px;
  padding: 2px;
}

.warning {
  background-color: indianred;
}
.warning:hover {
  color: white;
}

.ok {
  background-color: green;
}
.ok:hover {
  color: white;
}

.info {
  background-color: cadetblue;
}
.info:hover {
  color: white;
}

.disabled {
  color: black;
}

@media (max-width: 812px) {
  button {
    font-size: 19px;
  }
}

@media (max-width: 375px) {
  button {
    font-size: 12px;
  }
}
/* button {
  height: 4rem;
  width: 51.5%;
  font-size: 2.5rem;
  margin: 0 1rem;
  border: 1px solid black;
  cursor: pointer;
}

button:hover {
  color: white;
}

button:active {
  border: 1px solid white;
}

.ok {
  background-color: green;
}

.info {
  background-color: aquamarine;
}

.warning {
  background-color: palevioletred;
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
} */
</style>