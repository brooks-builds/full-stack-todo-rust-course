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
    errorMessage: '',
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
    }
  },
  actions: {
    async createAccount({commit, getters, state}) {
      if(!getters.accountFormValid) {
        commit("setErrorMessage", "Can't create account, missing usersname and/or password");
        return;
      }

      try {
        const createdAccount = await api.createAccount(state.accountForm);
        commit('setUser', createdAccount.data);
        commit("resetAccountForm")
        router.push("/")
      } catch (error) {
        commit("setErrorMessage", error.message);
      }
    },

    async login({state, commit, getters}) {
      if(!getters.accountFormValid) {
        commit("setErrorMessage", "Can't login, missing usersname and/or password");
        return;
      }

      try {
        const account = await api.login(state.accountForm);
        commit('setUser', account.data);
        commit("resetAccountForm")
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
