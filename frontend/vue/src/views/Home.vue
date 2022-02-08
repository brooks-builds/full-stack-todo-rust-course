<template>
  <div class="home">
    <h1>My Tasks</h1>
    <section class="tasks">
      <div v-for="task in tasks" v-bind:key="task.id" class="task" data-test-task>
        <span class="priority">{{ task.priority }}</span>
        <span>
          <input
            type="checkbox"
            :id="taskId(task.id)"
            :checked="task.completed"
          />
          <label :for="taskId(task.id)"></label>
        </span>
        <router-link :to="taskLink(task.id)">{{ task.title }}</router-link>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: "Home",
  components: {},
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

input[type="checkbox"] + label::before {
  content: "\a0";
  display: inline-block;
  vertical-align: 0.2rem;
  width: 3rem;
  height: 3rem;
  margin-right: 0.2rem;
  border-radius: 0.2rem;
  background: silver;
  text-indent: 0.15rem;
  line-height: 0.65;
}

input[type="checkbox"]:checked + label::before {
  content: "\2713";
  background: green;
}

input[type="checkbox"] {
  position: absolute;
  clip: rect(0, 0, 0, 0);
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