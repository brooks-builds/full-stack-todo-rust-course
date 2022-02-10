<template>
  <section class="one-task">
    <h1 data-test-title>
      {{ task.title }}
    </h1>
    <div>
      <span v-if="!task.completed_at">Completed: </span>
      <form-checkbox :checked="!!task.completed_at" :id="task.id" dataTest="completed" />
    </div>
    <div class="priority" data-test-priority>Priority: {{ task.priority }}</div>
    <div class="description">
      <p data-test-description>{{ task.description }}</p>
    </div>
  </section>
</template>

<script>
import FormCheckbox from "../components/FormCheckbox.vue";

export default {
  components: {
    FormCheckbox,
  },
  computed: {
    task() {
      return this.$store.state.tasks.find(
        (storeTask) => storeTask.id == this.$route.params.taskId
      );
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
</style>