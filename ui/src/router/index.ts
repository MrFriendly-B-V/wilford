// Composables
import {createRouter, createWebHistory} from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        component: () => import('@/views/Home.vue')
      },
      {
        path: 'login',
        name: 'Login',
        component: () => import('@/views/Login.vue')
      },
      {
        path: 'authorize',
        name: 'Authorize',
        component: () => import('@/views/Authorize.vue')
      },
      {
        path: 'login-ok',
        name: 'Login OK',
        component: () => import('@/views/LoginOk.vue')
      },
    ],
  },
  {
    path: '/manager',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: 'users',
        name: 'Users',
        component: () => import("@/views/manager/user/Users.vue")
      },
      {
        path: 'cat',
        name: 'CAT Tokens',
        component: () => import('@/views/manager/cat/ConstantAccessTokens.vue')
      },
      {
        path: 'clients',
        name: 'OAuth2 Clients',
        component: () => import('@/views/manager/client/Clients.vue')
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
