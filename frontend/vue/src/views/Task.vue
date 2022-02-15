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
        data-test-editing-title
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
    <div class="priority" data-test-priority v-if="!isEditMode">
      Priority: {{ task.priority }}
    </div>
    <div class="priority" v-else>
      <form-select
        label="Priority"
        :options="priorities"
        v-model="editPriority"
      />
    </div>
    <div class="description" v-if="!isEditMode">
      <p data-test-description>{{ task.description }}</p>
    </div>
    <div class="edit-description" v-else>
      <form-text-area v-model="editDescription" data-test-editing-description />
    </div>
    <div class="buttons">
      <form-button label="Save" status="ok" size="medium" @click="handleSave" data-test-submit />
    </div>
  </section>
</template>

<script>
import FormCheckbox from "../components/FormCheckbox.vue";
import FormInput from "../components/FormInput.vue";
import FormButton from "../components/FormButton.vue";
import FormTextArea from "../components/FormTextArea.vue";
import FormSelect from "../components/FormSelect.vue";
import { cloneDeep } from "lodash";

export default {
  components: {
    FormCheckbox,
    FormInput,
    FormButton,
    FormTextArea,
    FormSelect,
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
    priorities() {
      const prioritiesClone = cloneDeep(this.$store.state.priorities);
      console.log("cloned priorities", prioritiesClone);
      const prioritiesWithDefaultSet = prioritiesClone.map((priority) => {
        console.log(
          "currently edited priority",
          this.$store.state.editedTask.priority
        );
        priority.default =
          priority.value == this.$store.state.editedTask.priority;
        return priority;
      });
      return prioritiesWithDefaultSet;
    },
    editPriority: {
      get() {
        return this.$store.state.editedTask.priority;
      },
      set(newPriority) {
        this.$emit("editPriority", newPriority);
      },
    },
  },
  methods: {
    handleSave() {
      this.$emit("saveTask");
    },
  },
  mounted() {
    this.$emit("resetEditedTask");
    if(!this.$store.getters.loggedIn) {
      this.$emit("error", "You must be logged in to view tasks");
    }
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