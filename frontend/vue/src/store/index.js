import Vue from 'vue'
import Vuex from 'vuex'
import * as api from './api'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    createAccount: {
      username: null,
      password: null
    },
    errorMessage: '',
  },
  mutations: {
    setCreateAccountPassword(state, password) {
      Vue.set(state.createAccount, 'password', password);
    },
    setCreateAccountUsername(state, username) {
      Vue.set(state.createAccount, 'username', username);
    },
    setErrorMessage(state, errorMessage) {
      Vue.set(state, 'errorMessage', errorMessage);
    }
  },
  actions: {
    async createAccount({commit}, newAccount) {
      try {
        const createdAccount = await api.createAccount(newAccount);
        console.log('created account', createdAccount);
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
