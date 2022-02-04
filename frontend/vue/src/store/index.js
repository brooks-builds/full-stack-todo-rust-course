import Vue from 'vue'
import Vuex from 'vuex'
import * as api from './api'
import router from '../router';

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    createAccount: {
      username: null,
      password: null
    },
    errorMessage: '',
    user: {
      username: null,
      token: null,
      id: null,
    }
  },
  mutations: {
    resetCreateAccount(state) {
      Vue.set(state, 'createAccount', {username: null, password: null});
    },

    setCreateAccountPassword(state, password) {
      Vue.set(state.createAccount, 'password', password);
    },
    setCreateAccountUsername(state, username) {
      Vue.set(state.createAccount, 'username', username);
    },
    setErrorMessage(state, errorMessage) {
      Vue.set(state, 'errorMessage', errorMessage);
    },
    setUser(state, user) {
      Vue.set(state, 'user', user);
    }
  },
  actions: {
    async createAccount({commit, getters, state}) {
      if(!getters.createAccountFormValid) {
        commit("setErrorMessage", "Can't create account, missing usersname and/or password");
        return;
      }

      try {
        const createdAccount = await api.createAccount(state.createAccount);
        commit('setUser', createdAccount.data);
        commit("resetCreateAccount")
        router.push("/")
      } catch (error) {
        commit("setErrorMessage", error.message);
      }
    }
  },
  modules: {
  },
  getters: {
    createAccountFormValid(state) {
      return !!state.createAccount.username && !!state.createAccount.password
    },
    loggedIn() {
      return false
    }
  }
})
