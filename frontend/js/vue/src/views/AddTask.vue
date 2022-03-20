<template>
  <section class="one-task">
    <div class="title">
      <form-input
        label="Title"
        type="text"
        v-model="taskTitle"
        dataTest="title"
      />
    </div>
    <form-select
      label="Priority"
      :options="priorities"
      v-model="priority"
      dataTest="priority"
    />
    <div class="description">
      <form-text-area v-model="description" dataTest="description" />
    </div>
    <div class="buttons">
      <form-button
        label="Create Task"
        size="medium"
        status="ok"
        @click="handleCreateTask"
        dataTest="submit"
      />
      <form-button
        label="Cancel"
        size="medium"
        status="info"
        @click="handleCancel"
        dataTest="cancel"
      />
    </div>
  </section>
</template>

<script>
import FormInput from "../components/FormInput.vue";
import FormTextArea from "../components/FormTextArea.vue";
import FormButton from "../components/FormButton.vue";
import FormSelect from "../components/FormSelect.vue";

export default {
  components: {
    FormInput,
    FormTextArea,
    FormButton,
    FormSelect,
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
    priorities() {
      return this.$store.state.priorities;
    },
  },
  methods: {
    handleCreateTask() {
      this.$emit("createTask");
    },
    handleCancel() {
      this.$router.push("/");
    },
  },
};
</script>

<style scoped>
section {
  text-align: center;
  margin-top: 25px;
}

section > * {
  margin-bottom: 10px;
}

.buttons > *:first-child {
  margin-right: 10px;
}
</style>