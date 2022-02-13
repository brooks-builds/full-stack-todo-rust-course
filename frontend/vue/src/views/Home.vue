<template>
  <div class="home">
    <h1>My Tasks</h1>
    <section class="tasks">
      <div
        v-for="task in tasks"
        v-bind:key="task.id"
        class="task"
        data-test-task
      >
        <span class="priority">{{ task.priority }}</span>
        <span>
          <form-checkbox
            :id="taskId(task.id)"
            :checked="!!task.completed_at"
            @checked="handleCompletedTask(task.id)"
          />
        </span>
        <router-link :to="taskLink(task.id)" data-test-tasklink>{{
          task.title
        }}</router-link>
      </div>
    </section>
  </div>
</template>

<script>
import FormCheckbox from "../components/FormCheckbox.vue";

export default {
  name: "Home",
  components: {
    FormCheckbox,
  },
  computed: {
    tasks() {
      return this.$store.state.tasks;
    },
  },
  methods: {
    taskLink(taskId) {
      return `/tasks/${taskId}`;
    },
    taskId(taskId) {
      return `completed-${taskId}`;
    },
    handleCompletedTask(taskId) {
      this.$emit("completedTask", taskId);
    },
  },
};
</script>

<style scoped>
h1 {
  margin-bottom: 2rem;
  font-size: 4rem;
}

.home {
  text-align: center;
}

.tasks {
  display: flex;
  justify-content: space-around;
  flex-direction: column;
}

.task {
  margin-bottom: 1rem;
}

.task > a {
  text-decoration: none;
  color: lightblue;
}

.task > span {
  margin-right: 0.5rem;
}
</style>