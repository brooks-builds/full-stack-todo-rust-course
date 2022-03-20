<template>
  <div class="home">
    <h1 v-if="isLoggedIn">My Tasks</h1>
    <section class="sort-and-filter">
      <div class="filter-by" v-if="isLoggedIn">
        <form-select
          label="Filter By"
          :options="filterByOptions"
          v-model="filterBy"
          dataTest="filter"
        />
      </div>
      <div class="sort-by" v-if="isLoggedIn">
        <form-select
          label="Sort By"
          :options="sortByOptions"
          v-model="sortBy"
          dataTest="sort"
        />
      </div>
    </section>
    <section class="tasks">
      <data-table :data="dataTableTasks">
        <template v-slot:Completed="{ data: task }">
          <form-checkbox
            :checked="!!task.completed_at"
            :id="task.id.toString()"
            @checked="handleCompletedToggle(task.id)"
            dataTest="completed"
          />
        </template>
        <template v-slot:Task="{ data: task }">
          <span class="task-title">
            <router-link :to="taskLink(task.id)" data-test="tasklink">{{
              task.title
            }}</router-link>
          </span>
        </template>
        <template v-slot:Priority="{ data: priority }">
          <span data-test="priority">{{ priority }}</span>
        </template>
      </data-table>
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
        data: this.tasks.map((task) => {
          return [task.priority, task, task];
        }),
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
    handleCompletedToggle(taskId) {
      this.$emit("toggleCompletedTask", taskId);
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
@import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans@1&display=swap");

h1 {
  text-align: center;
  margin: 20px 0;
  font-family: "IBM Plex Sans", sans-serif;
}

.sort-and-filter {
  display: flex;
  justify-content: space-evenly;
}

a {
  color: lightblue;
  text-decoration: none;
}
</style>