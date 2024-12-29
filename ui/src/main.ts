/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'

const isLocalhost = window.location.host.includes('localhost');
export const server = isLocalhost
    ? "http://localhost:2521"
    : "/api";

// Type definitions
export type InputValidationRule = (v: string) => string | boolean;
export type InputValidationRules = InputValidationRule[];

export type DataTableHeader = { title: string, value: string };
export type DataTableHeaders = DataTableHeader[];

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

const app = createApp(App)

registerPlugins(app)

app.mount('#app')
