<template>
  <section class="one-task">
    <div class="title">
      <form-input label="Title" type="text" v-model="taskTitle" />
    </div>
    <div class="priority">
      <span class="label">Priority</span>
      <select v-model="priority">
        <option value="A">A</option>
        <option value="B">B</option>
        <option value="C">C</option>
      </select>
    </div>
    <div class="description">
      <form-text-area v-model="description" />
    </div>
    <div class="buttons">
      <form-button
        label="Create Task"
        size="medium"
        status="ok"
        @click="handleCreateTask"
      />
    </div>
  </section>
</template>

<script>
import FormInput from "../components/FormInput.vue";
import FormTextArea from "../components/FormTextArea.vue";
import FormButton from "../components/FormButton.vue";

export default {
  components: {
    FormInput,
    FormTextArea,
    FormButton,
  },
  computed: {
    taskTitle: {
      get() {
        return this.$store.state.editedTask.title;
      },
      set(title) {
        return this.$emit("editTitle", title);
      },
    },
    priority: {
      get() {
        return this.$store.state.editedTask.priority;
      },
      set(priority) {
        return this.$emit("editPriority", priority);
      },
    },
    description: {
      get() {
        return this.$store.state.editedTask.description;
      },
      set(newDescription) {
        return this.$emit("editDescription", newDescription);
      },
    },
  },
  methods: {
    handleCreateTask() {
      this.$emit("createTask");
    },
  },
};
</script>

<style scoped>
.one-task {
  text-align: center;
}

h1 {
  font-size: 5rem;
  color: aqua;
}

h1,
div {
  margin-bottom: 3rem;
}

textarea {
  background-color: blanchedalmond;
  font-size: 3rem;
}

.priority > select {
  font-size: 3rem;
}

.priority > .label {
  margin: 0 1rem;
}
</style>