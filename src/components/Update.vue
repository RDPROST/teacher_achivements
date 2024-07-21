<template>
  <v-container>
    <v-card v-if="release">
      <v-card-title>{{ release.tag_name }}</v-card-title>
      <v-card-text>{{ release.body }}</v-card-text>
      <v-card-actions>
        <v-btn @click="downloadUpdate" :loading="downloading"
          >Скачать обновление</v-btn
        >
		<v-btn @click="openLink" >Перейти к обновлению</v-btn>
      </v-card-actions>
    </v-card>
    <v-btn v-else @click="checkUpdates">Проверить обновления</v-btn>
  </v-container>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
  data() {
    return {
      release: null,
      downloading: false,
    };
  },
  methods: {
    async checkUpdates() {
      try {
        const release = await invoke("check_for_updates");
        this.release = release;
        console.log(release);
      } catch (error) {
        console.error("Failed to check for updates:", error);
      }
    },
    async downloadUpdate() {
      if (!this.release) {
        console.error("No release data available");
        return;
      }

      this.downloading = true;
      try {
        // Get the download URL and file name
        const [downloadUrl, fileName] = await invoke("get_download_url", {
          release: this.release,
        });

        // Open the save dialog
        const filePath = await invoke("open_save_dialog", {
          defaultFileName: fileName,
        });

        // Download the file
        await invoke("download_file", {
          url: downloadUrl,
          path: filePath,
        });
      } catch (error) {
        console.error("Failed to download file:", error);
      } finally {
        this.downloading = false;
      }
    },
	openLink() {
		invoke("open_link");
	}
  },
};
</script>
