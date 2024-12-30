// Composables
import {createRouter, createWebHistory} from 'vue-router'

const routes = [
  {
    path: '/',
    children: [
      {
        path: '',
        component: () => import('@/layouts/authorizations/Authorized.vue'),
        children: [
          {
            path: '',
            name: 'Home',
            component: () => import('@/views/Home.vue')
          },
          {
            path: '/me',
            name: 'me',
            component: () => import('@/views/me/Me.vue'),
          }
        ]
      },
      {
        path: '',
        component: () => import('@/layouts/authorizations/Unauthorized.vue'),
        children: [
          {
            path: 'login',
            name: 'Login',
            component: () => import('@/views/Login.vue')
          },
          {
            path: 'register',
            name: 'Register',
            component: () => import('@/views/Register.vue')
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
          {
            path: '/password-forgotten',
            name: "PasswordForgotten",
            component: () => import('@/views/PasswordForgotten.vue')
          }
        ]
      }
    ],
  },
  {
    path: '/manager',
    component: () => import('@/layouts/authorizations/AuthorizedManager.vue'),
    children: [
      {
        path: '',
        name: "Manager",
        component: () => import('@/views/manager/Manager.vue')
      },
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
