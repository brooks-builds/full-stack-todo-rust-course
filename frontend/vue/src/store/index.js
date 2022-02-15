import Vue from 'vue'
import Vuex from 'vuex'
import {cloneDeep} from 'lodash'
import * as api from './api'
import router from '../router';

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    accountForm: {
      username: null,
      password: null
    },
    editedTask: {
      id: null,
      title: null,
      completed_at: null,
      priority: null,
      description: null
    },
    editingOneTask: false,
    errorMessage: '',
    tasks: [],
    user: {
      username: null,
      token: null,
      id: null,
    },
    priorities: [
      {value: "A", label: "A", default: true},
      {value: "B", label: "B", default: false},
      {value: "C", label: "C", default: false},
    ],
    sortBy: [
      {value: "priority", label: "Priority", default: false},
      {value: "name", label: "Name", default: false},
      {value: "id", label: "Created Order", default: true},
    ],
    selectedSortBy: "id",
    filterByOptions: [
      {value: "none", label: "None", default: true},
      {value: "completed", label: "Completed", default: false},
      {value: "uncompleted", label: "Uncompleted", default: false},
      {value: "priorityA", label: "Priority A", default: false},
      {value: "priorityB", label: "Priority B", default: false},
      {value: "priorityC", label: "Priority C", default: false},
    ],
    selectedFilterBy: "none",
    errorMessageTimeout: 10000,
  },
  mutations: {
    resetAccountForm(state) {
      Vue.set(state, 'accountForm', {username: null, password: null});
    },
    resetTasks(state, tasks = []) {
      Vue.set(state, 'tasks', tasks);
    },
    setAccountFormPassword(state, password) {
      Vue.set(state.accountForm, 'password', password);
    },
    setAccountFormUsername(state, username) {
      Vue.set(state.accountForm, 'username', username);
    },
    setEditPriority(state, priority) {
      Vue.set(state.editedTask, "priority", priority);
    },
    setEditedTask(state, editedTask) {
      Vue.set(state, "editedTask", editedTask);
    },
    setEditTaskDescription(state, taskDescription) {
      Vue.set(state.editedTask, "description", taskDescription);
    },
    setEditTaskTitle(state, taskTitle) {
      Vue.set(state.editedTask, "title", taskTitle);
    },
    setErrorMessage(state, errorMessage) {
      Vue.set(state, 'errorMessage', errorMessage);
    },
    setUser(state, user) {
      Vue.set(state, 'user', user);
    },
    turnOffEditTaskMode(state) {
      Vue.set(state, "editingOneTask", false);
    },
    turnOnEditTaskMode(state) {
      Vue.set(state, "editingOneTask", true);
    },
    updateTask(state, updatedTask) {
      const clonedTasks = cloneDeep(state.tasks);
      const taskIndex = clonedTasks.findIndex(task => task.id == updatedTask.id);
      if(taskIndex == -1) throw new Error(`could not find task with id ${updatedTask.id}`);
      clonedTasks[taskIndex] = updatedTask;
      Vue.set(state, "tasks", clonedTasks);
    },
    appendTask(state, task) {
      const clonedTasks = cloneDeep(state.tasks);
      clonedTasks.push(task);
      Vue.set(state, 'tasks', clonedTasks);
    },
    completeTask(state, taskId) {
      const clonedTasks = cloneDeep(state.tasks);
      const taskToComplete = clonedTasks.find(task => task.id == taskId);
      if(!taskToComplete) throw new Error(`could not find task with id ${taskId}`);
      const now = new Date();
      taskToComplete.completed_at = now.toUTCString();
      Vue.set(state, "tasks", clonedTasks);
    },
    logout(state) {
      Vue.set(state, "user", {
      username: null,
      token: null,
      id: null,
    })},
    removeTaskById(state, taskId) {
      const clonedTasks = cloneDeep(state.tasks);
      const filteredTasks = clonedTasks.filter(task => task.id != taskId);
      Vue.set(state, 'tasks', filteredTasks);
    },
    setSelectedSortBy(state, sortBy) {
      Vue.set(state, "selectedSortBy", sortBy);
    },
    setSelectedFilterBy(state, filterByOptionValue) {
      Vue.set(state, "selectedFilterBy", filterByOptionValue);
    },
    toggleEditedCompleted(state) {
      Vue.set(state.editedTask, "completed_at", state.editedTask.completed ? null : (new Date()).toUTCString());
    }
  },
  actions: {
    async createAccount({commit, getters, state, dispatch}) {
      if(!getters.accountFormValid) {
        commit("setErrorMessage", "Can't create account, missing usersname and/or password");
        return;
      }

      try {
        const createdAccount = await api.createAccount(state.accountForm);
        commit('setUser', createdAccount.data);
        commit("resetAccountForm")
        dispatch("loadTasksFromApi");
        router.push("/")
      } catch (error) {
        commit("setErrorMessage", error.message);
      }
    },

    async loadTasksFromApi({state, commit}) {
      const tasks = await api.getTasks(state.user.token);
      commit("resetTasks", tasks);
    },

    async login({state, commit, getters, dispatch}) {
      if(!getters.accountFormValid) {
        commit("setErrorMessage", "Can't login, missing usersname and/or password");
        return;
      }

      try {
        const account = await api.login(state.accountForm);
        commit('setUser', account.data);
        commit("resetAccountForm");
        dispatch("loadTasksFromApi");
        router.push("/")
      } catch (error) {
        commit("setErrorMessage", error.message);
      }
    },
    async saveTask({state, commit}) {
      await api.updateTask(state.editedTask, state.user.token);
      commit("updateTask", state.editedTask);
      commit("turnOffEditTaskMode");
    },
    switchToEditMode({commit, state}, taskId) {
      if(!taskId) return commit("setErrorMessage", "No task ID, please try logging in and then retry");
      const currentTask = state.tasks.find(task => task.id == taskId);
      if(!currentTask) return commit("setErrorMessage", `Could not find task with id ${taskId}`);

      commit("setEditedTask", Object.assign({}, currentTask));
      commit("turnOnEditTaskMode");
    },
    resetEditTask({commit, state}) {
      const editTask = Object.assign({}, state.editedTask);
      editTask.id = null;
      editTask.completed = null;
      editTask.description = null;
      editTask.priority = 'A';
      editTask.title = null;
      commit("setEditedTask", editTask);
    },
    async createTask({state, commit}) {
      const createdTask = await api.createTask(state.editedTask, state.user.token);
      commit("appendTask", createdTask);
      router.push("/")
    },
    async completeTask({state, commit}, taskId) {
      await api.completeTask(taskId, state.user.token);
      commit("completeTask", taskId);
    },
    async logout({state, commit}) {
      await api.logout(state.user.token);
      commit("logout");
      commit("resetTasks");
      router.push('/');
    },
    async deleteTask({state, commit}, taskId) {
      await api.deleteTask(taskId, state.user.token);
      commit("removeTaskById", taskId);
      router.push("/");
    }
  },
  modules: {
  },
  getters: {
    accountFormValid(state) {
      return !!state.accountForm.username && !!state.accountForm.password
    },
    loggedIn(state) {
      return !!state.user.token
    },
    username(state) {
      return state.user.username
    }
  }
})
