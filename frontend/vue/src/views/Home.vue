<template>
  <div class="home">
    <h1 v-if="isLoggedIn">My Tasks</h1>
    <div class="filter-by" v-if="isLoggedIn">
      <form-select
        label="Filter By"
        :options="filterByOptions"
        v-model="filterBy"
      />
    </div>
    <div class="sort-by" v-if="isLoggedIn">
      <form-select label="Sort By" :options="sortByOptions" v-model="sortBy" />
    </div>
    <section class="tasks">
      <data-table :data="dataTableTasks" />
      <div
        v-for="task in tasks"
        v-bind:key="task.id"
        class="task"
        data-test-task
      >
        <span class="priority" data-test-priority>{{ task.priority }}</span>
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
import DataTable from "../components/DataTable.vue";

export default {
  name: "Home",
  components: {
    FormCheckbox,
    FormSelect,
    DataTable,
  },
  computed: {
    tasks() {
      const clonedTasks = cloneDeep(this.$store.state.tasks);
      const filteredTasks = clonedTasks.filter(this.filterCallback);
      const sortedFilteredTasks = filteredTasks.sort(this.sortCallback);
      return sortedFilteredTasks;
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
    filterByOptions() {
      return this.$store.state.filterByOptions;
    },
    filterBy: {
      get() {
        return this.$store.state.selectedFilterBy;
      },
      set(filterOptionValue) {
        this.$emit("filterSet", filterOptionValue);
      },
    },
    dataTableTasks() {
      return {
        titles: ["Priority", "Completed", "Task"],
        data: [
          ["A", "[x]", "This is a task title"],
          ["A", "[ ]", "This is a task title, but not completed"],
        ],
      };
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
        id: (taskA, taskB) => taskA.id - taskB.id,
        priority: (taskA, taskB) => {
          if (taskA.priority > taskB.priority) return 1;
          else if (taskA.priority < taskB.priority) return -1;
          else return 0;
        },
        name: (taskA, taskB) => {
          if (taskA.title.toLowerCase() < taskB.title.toLowerCase()) return -1;
          else if (taskA.title.toLowerCase() > taskB.title.toLowerCase())
            return 1;
          else return 0;
        },
      };
      return sortByComparitors[this.$store.state.selectedSortBy](taskA, taskB);
    },
    filterCallback(task) {
      const filterByComparitors = {
        none: true,
        completed: task.completed_at,
        uncompleted: !task.completed_at,
        priorityA: task.priority == "A",
        priorityB: task.priority == "B",
        priorityC: task.priority == "C",
      };

      return filterByComparitors[this.$store.state.selectedFilterBy];
    },
  },
};
</script>

<style scoped>
@import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:ital@1&display=swap");

h1 {
  font-family: "IBM Plex Sans", sans-serif;
}

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