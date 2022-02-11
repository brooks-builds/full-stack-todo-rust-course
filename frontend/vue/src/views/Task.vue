<template>
  <section class="one-task">
    <h1 data-test-title v-if="!isEditMode">
      {{ task.title }}
    </h1>
    <div v-else>
      <form-input
        label="Task Title"
        type="text"
        :value="task.title"
        v-model="editTitle"
      />
    </div>
    <div>
      <span v-if="!task.completed_at">Completed: </span>
      <form-checkbox
        :checked="!!task.completed_at"
        :id="`${task.id}`"
        dataTest="completed"
      />
    </div>
    <div class="priority" data-test-priority>Priority: {{ task.priority }}</div>
    <div class="description" v-if="!isEditMode">
      <p data-test-description>{{ task.description }}</p>
    </div>
    <div class="edit-description" v-else>
      <form-text-area v-model="editDescription" />
    </div>
    <div class="buttons">
      <form-button label="Save" status="ok" size="medium" @click="handleSave" />
    </div>
  </section>
</template>

<script>
import FormCheckbox from "../components/FormCheckbox.vue";
import FormInput from "../components/FormInput.vue";
import FormButton from "../components/FormButton.vue";
import FormTextArea from "../components/FormTextArea.vue";

export default {
  components: {
    FormCheckbox,
    FormInput,
    FormButton,
    FormTextArea,
  },
  computed: {
    editDescription: {
      get() {
        return this.$store.state.editedTask.description;
      },
      set(newEditedDescription) {
        return this.$emit("editDescription", newEditedDescription);
      },
    },
    editTitle: {
      get() {
        return this.$store.state.editedTask.title;
      },
      set(newEditedTaskTitle) {
        return this.$emit("editTitle", newEditedTaskTitle);
      },
    },
    isEditMode() {
      return this.$store.state.editingOneTask;
    },
    task() {
      return this.$store.state.tasks.find(
        (storeTask) => storeTask.id == this.$route.params.taskId
      );
    },
  },
  methods: {
    handleSave() {
      this.$emit("saveTask");
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
</style>