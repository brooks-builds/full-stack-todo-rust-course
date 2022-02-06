import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import CreateAccount from '../views/CreateAccount.vue'
import Login from '../views/Login.vue'
import Task from '../views/Task.vue';

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/create-account',
    component: CreateAccount,
  },
  {
    path: '/login',
    component: Login
  },
  {
    path: '/tasks/:taskId',
    component: Task
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
