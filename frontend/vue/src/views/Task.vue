<template>
  <section class="one-task">
    <h1>{{ task.title }}</h1>
    <div>
      <p v-if="!task.completed_at">Not Completed</p>
    </div>
  </section>
</template>

<script>
export default {
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
</style>