<template>
  <v-container fluid>
    <div v-if="!isValidating">
      <h1>{{ video!.title }}</h1>
      <div class="d-flex justify-center align-center">
        <video controls class="mx-auto" width="800">
          <source :src="resolveVideo(video!.bucket)" />
          <p>Video format not supported</p>
        </video>
      </div>
      {{ video!.description }}
    </div>
    <v-skeleton-loader v-else></v-skeleton-loader>
  </v-container>
</template>

<script lang="ts" setup>
import { fetcher, Video, resolveVideo } from "@/api";
import useSWRV from "swrv";
import { useRoute } from "vue-router";

const route = useRoute();

const { data: video, isValidating } = useSWRV<Video>(
  `/videos/${route.params.id}`,
  fetcher,
  { revalidateOnFocus: false }
);
</script>
