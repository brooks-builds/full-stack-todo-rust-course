<template>
  <div class="home">
    <h1 v-if="isLoggedIn">My Tasks</h1>
    <div class="sort-by" v-if="isLoggedIn">
      <form-select label="Sort By" :options="sortByOptions" v-model="sortBy" />
    </div>
    <section class="tasks">
      <div
        v-for="task in tasks"
        v-bind:key="task.id"
        class="task"
        data-test-task
      >
        <span>Id: {{ task.id }}</span>
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
import FormSelect from "../components/FormSelect.vue";
import { cloneDeep } from "lodash";

export default {
  name: "Home",
  components: {
    FormCheckbox,
    FormSelect,
  },
  computed: {
    tasks() {
      const clonedTasks = cloneDeep(this.$store.state.tasks);
      clonedTasks.sort(this.sortCallback);
      return clonedTasks;
    },
    sortByOptions() {
      const clonedSortByOptions = cloneDeep(this.$store.state.sortBy);
      return clonedSortByOptions.map((sortByOption) => {
        sortByOption.default =
          sortByOption.value == this.$store.state.selectedSortBy;
        return sortByOption;
      });
    },
    isLoggedIn() {
      return this.$store.getters.loggedIn;
    },
    sortBy: {
      get() {
        return this.$store.state.selectedSortBy;
      },
      set(sortByValue) {
        this.$emit("changeSortBy", sortByValue);
      },
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
    sortCallback(taskA, taskB) {
      const sortByComparitors = {
        id: taskA.id > taskB.id,
        priority: taskA.priority > taskB.priority,
        name: taskA.title.toLowerCase() > taskB.title.toLowerCase(),
      };

      return sortByComparitors[this.$store.state.selectedSortBy];
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