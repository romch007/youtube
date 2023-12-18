<template>
  <v-container fluid>
    <v-text-field
      placeholder="Search videos..."
      clearable
      variant="outlined"
      v-model="searchTerm"
    ></v-text-field>

    <div class="d-flex flex-row flex-wrap">
      <v-card
        class="mx-auto my-5"
        width="355"
        height="200"
        variant="elevated"
        v-for="video in videos"
        link
        :to="`/watch/${video.id}`"
      >
        <v-card-title>{{ video.title }}</v-card-title>
        <v-card-subtitle
          >{{ video.author.username }} - {{ formatDate(video.published_at) }} -
          {{ formatDuration(video.duration_seconds) }}</v-card-subtitle
        >

        <v-card-text class="text-justify">
          {{ truncate(video.description) }}
        </v-card-text>
      </v-card>
    </div>
  </v-container>
</template>

<script lang="ts" setup>
import { fetcher, Video } from "@/api";
import dayjs from "@/dayjs";
import useSWRV from "swrv";
import { ref } from "vue";

const searchTerm = ref("");

const { data: videos } = useSWRV<Video[]>(
  () => "/videos?search=" + searchTerm.value,
  fetcher
);

function formatDate(d: string): string {
  return dayjs.utc(d).from(dayjs());
}

const MAX_DESCRIPTION_LENGTH = 200;

function truncate(text: string): string {
  if (text.length < MAX_DESCRIPTION_LENGTH) return text;

  return text.slice(0, MAX_DESCRIPTION_LENGTH) + "...";
}

function formatDuration(seconds: number): string {
  return dayjs.duration(seconds, "seconds").format("mm[min] ss[s]");
}
</script>
