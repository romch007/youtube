<template>
  <v-container fluid>
    <div class="d-flex align-center justify-center">
      <v-sheet width="400" class="mx-auto">
        <v-alert
          density="compact"
          type="error"
          title="Upload failed"
          text="Check your connection"
          v-if="errorDuringUpload"
          class="mb-6"
        ></v-alert>

        <v-form @submit.prevent="upload" id="upload-form">
          <v-text-field
            variant="outlined"
            v-model="title"
            label="Title"
            name="title"
            :rules="requiredRule"
          ></v-text-field>

          <v-textarea
            v-model="description"
            label="Description"
            name="description"
            :rules="requiredRule"
          ></v-textarea>

          <v-file-input
            v-model="videoFile"
            label="Video"
            show-size
            name="video"
            :rules="requiredRule"
          ></v-file-input>

          <v-btn
            :loading="isUploading"
            type="submit"
            color="primary"
            block
            class="mt-2"
            >Upload

            <template v-slot:loader>
              <v-progress-circular
                :model-value="uploadProgress"
              ></v-progress-circular>
            </template>
          </v-btn>
        </v-form>
      </v-sheet>
    </div>
  </v-container>
</template>

<script lang="ts" setup>
import api, { Video } from "@/api";
import { AxiosRequestConfig } from "axios";
import { ref, computed } from "vue";
import { useRouter } from "vue-router";

const title = ref();
const description = ref();
const videoFile = ref();

const requiredRule = computed(() => [(value: any) => !!value]);

const uploadProgress = ref(0);
const isUploading = ref(false);
const errorDuringUpload = ref(false);

const router = useRouter();

async function upload() {
  uploadProgress.value = 0;
  errorDuringUpload.value = false;

  const formElement = document.querySelector<HTMLFormElement>("#upload-form");
  if (!formElement) return;

  const formData = new FormData(formElement);

  isUploading.value = true;

  const requestConfig: AxiosRequestConfig = {
    onUploadProgress(progressEvent) {
      uploadProgress.value = (progressEvent.progress ?? 0) * 100;
    },
    headers: {
      "Content-Type": "multipart/form-data",
    },
  };

  try {
    const response = await api.post<Video>(
      "/videos/upload",
      formData,
      requestConfig
    );

    router.push(`/watch/${response.data.id}`);
  } catch (err) {
    errorDuringUpload.value = true;
  }

  isUploading.value = false;
}
</script>
