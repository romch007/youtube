<template>
  <v-container fluid>
    <div class="d-flex align-center justify-center">
      <v-sheet width="400" class="mx-auto">
        <v-form fast-fail @submit.prevent="login">
          <v-alert
            density="compact"
            type="error"
            title="Login failed"
            text="Check your credentials and try again"
            v-if="errorDuringLogin"
            class="mb-6"
          ></v-alert>

          <v-text-field
            variant="outlined"
            v-model="username"
            label="Username"
          ></v-text-field>

          <v-text-field
            variant="outlined"
            v-model="password"
            label="Password"
            type="password"
          ></v-text-field>

          <v-btn type="submit" color="primary" block class="mt-2"
            >Sign in</v-btn
          >
        </v-form>
      </v-sheet>
    </div>
  </v-container>
</template>

<script lang="ts" setup>
import api from "@/api";
import { ref } from "vue";
import { useRouter } from "vue-router";

const username = ref();
const password = ref();
const errorDuringLogin = ref(false);

const router = useRouter();

async function login() {
  errorDuringLogin.value = false;

  try {
    const response = await api.post("/login", {
      username: username.value,
      password: password.value,
    });

    localStorage.setItem("token", response.data);

    router.push("/");
  } catch (err) {
    errorDuringLogin.value = true;
  }
}
</script>
