import { createRouter, createWebHistory } from 'vue-router'
//import Login from '../components/Login.vue'
//import Register from '../components/Register.vue'
//import Home from '../components/Home.vue'
import Update from '../components/Update.vue'
import Main from '../views/Main.vue'

const routes = [
//  { path: '/login', component: Login },
//  { path: '/register', component: Register },
  {
    path: '/',
    component: Main,
    children: [
    //  { path: '', component: Home },
      { path: 'update', component: Update },
      // Добавьте здесь другие маршруты для страниц из меню
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
