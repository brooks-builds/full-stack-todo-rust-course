import Vue from 'vue'
import Vuex from 'vuex'
import * as api from './api'
import router from '../router';

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    accountForm: {
      username: null,
      password: null
    },
    editingOneTask: false,
    errorMessage: '',
    tasks: [],
    user: {
      username: null,
      token: null,
      id: null,
    }
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
