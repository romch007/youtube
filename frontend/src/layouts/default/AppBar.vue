<template>
  <v-app-bar color="primary">
    <v-app-bar-title to="/"> YouTube </v-app-bar-title>

    <v-btn icon to="/upload" v-if="isLoggedIn">
      <v-icon>mdi-upload</v-icon>
    </v-btn>
    <v-btn icon to="/login" v-else>
      <v-icon>mdi-login</v-icon>
    </v-btn>
  </v-app-bar>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const isLoggedIn = computed(() => localStorage.getItem("token") != null);
</script>
