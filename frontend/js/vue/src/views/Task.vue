<template>
  <section class="one-task">
    <h1 data-test="title" v-if="!isEditMode">
      {{ task.title }}
    </h1>
    <div v-else>
      <form-input
        label="Task Title"
        type="text"
        :value="task.title"
        v-model="editTitle"
        dataTest="editing-title"
      />
    </div>
    <div>
      <span>Completed: </span>
      <span v-if="!isEditMode" :class="completedClass" data-test="completed">{{
        completedIcon
      }}</span>
      <form-checkbox
        :checked="!!task.completed_at"
        :id="`${task.id}`"
        dataTest="completed"
        v-else
        @checked="handleEditCompletedToggle"
      />
    </div>
    <div class="priority" data-test="priority" v-if="!isEditMode">
      Priority: <span>{{ task.priority }}</span>
    </div>
    <div class="priority" v-else>
      <form-select
        label="Priority"
        :options="priorities"
        v-model="editPriority"
        data-test="editing-priority"
      />
    </div>
    <div class="description" v-if="!isEditMode">
      <p data-test="description">{{ task.description }}</p>
    </div>
    <div class="edit-description" v-else>
      <form-text-area v-model="editDescription" data-test="editing-description" />
    </div>
    <div class="buttons" v-if="isEditMode">
      <form-button
        label="Save"
        status="ok"
        size="medium"
        @click="handleSave"
        data-test="submit"
      />
      <form-button
        label="Cancel"
        status="info"
        size="medium"
        @click="handleCancel"
        data-test="cancel"
      />
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
  data() {return {
    checkIfLoggedIn: null,
  }},
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
        (task) => task.id == this.$route.params.taskId
      );
    },
    priorities() {
      const prioritiesClone = cloneDeep(this.$store.state.priorities);
      const prioritiesWithDefaultSet = prioritiesClone.map((priority) => {
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
    completedIcon() {
      return this.task.completed_at ? "✓" : "X";
    },
    completedClass() {
      return this.task.completed_at ? "completed" : "not-completed";
    },
  },
  methods: {
    handleSave() {
      this.$emit("saveTask");
    },
    handleEditCompletedToggle() {
      this.$emit("editCompleteToggle");
    },
    handleCancel() {
      this.$emit("cancelEditMode");
    },
  },
  mounted() {
    this.$emit("resetEditedTask");
    this.checkIfLoggedIn = setTimeout(() => {
      if (!this.$store.getters.loggedIn) {
        this.$emit("error", "You must be logged in to view tasks");
      }
    }, 500);
  },
  beforeDestroy() {
    if(this.checkIfLoggedIn) {
      clearTimeout(this.checkIfLoggedIn);
    }
  }
};
</script>

<style scoped>
section {
  text-align: center;
}

section > * {
  margin-bottom: 10px;
}

h1 {
  margin-bottom: 25px;
}

.completed {
  color: green;
}

.not-completed {
  color: indianred;
}

.priority > span {
  color: aquamarine;
}

input[type="text"] {
  width: 100%;
}

.buttons > *:first-child {
  margin-right: 10px;
}
/* @import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:ital@1&display=swap");

h1 {
  font-family: "IBM Plex Sans", sans-serif;
}

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

.completed {
  color: green;
}

.not-completed {
  color: palevioletred;
} */
</style>