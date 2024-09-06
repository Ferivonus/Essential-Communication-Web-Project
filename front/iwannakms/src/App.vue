<script>
import { computed } from "vue";
import { useRoute } from "vue-router";

export default {
  setup() {
    const route = useRoute();
    const isHomeRoute = computed(
      () => route.path === "/" || route.path === "/home"
    );

    return {
      isHomeRoute,
    };
  },
};
</script>

<template>
  <div class="navigation-links">
    <nav v-if="!isHomeRoute">
      <li>
        <router-link to="/">Home</router-link>
      </li>
      <li>
        <router-link to="/about">About</router-link>
      </li>
      <li>
        <router-link to="/form">form</router-link>
      </li>
      <li>
        <router-link to="/messages">messages</router-link>
      </li>
      <li>
        <router-link to="/wailing-wall">wailing wall</router-link>
      </li>
      <li>
        <router-link to="/contact">contact</router-link>
      </li>
    </nav>
    <router-view />
  </div>
</template>
