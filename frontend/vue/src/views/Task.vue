<template>
  <section class="one-task">
    <h1>{{ task.title }}</h1>
    <div>
      <span v-if="!task.completed_at">Completed: </span>
      <form-checkbox :checked="!!task.completed_at" :id="task.id" />
    </div>
    <div class="priority">Priority: {{ task.priority }}</div>
    <div class="description">
      <p>{{ task.description }}</p>
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