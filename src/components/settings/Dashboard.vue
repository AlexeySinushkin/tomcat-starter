<script setup lang="ts">
import { CommonShape } from "@/domain/commonShape";

const emit = defineEmits<{
  editGlobalVars: [];
  editRelease: [release: CommonShape];
  deleteRelease: [release: CommonShape];
  copyRelease: [release: CommonShape];
  addRelease: [];
  editPlatform: [platform: CommonShape];
  deletePlatform: [platform: CommonShape];
  copyPlatform: [platform: CommonShape];
  addPlatform: [];
  editServer: [server: CommonShape];
  deleteServer: [server: CommonShape];
  copyServer: [server: CommonShape];
  addServer: [];
}>();
</script>

<template>
  <v-container>
    <v-row>
      <v-col cols="6">
        <v-sheet class="pa-5" color="grey-lighten-1">
          Global variables
          <v-card class="ma-5">
            <template #text>
              <ul>
                <li v-for="prop in config.globalVars">
                  {{ prop.name }} - {{ prop.value }}
                </li>
              </ul>
            </template>
            <v-card-actions>
              <v-btn @click="editGlobalVars">Edit</v-btn>
            </v-card-actions>
          </v-card>
        </v-sheet>
      </v-col>
      <v-col cols="6">
        <v-sheet class="pa-5" color="grey-lighten-1">
          Releases
          <v-container>
            <v-row>
              <v-col v-for="release in config.releases" cols="12">
                <v-card class="ma-5">
                  <template #title>
                    {{ release.name }}
                  </template>
                  <template #text>
                    <ul>
                      <li v-for="p in release.properties">
                        {{ "${release." + p.name + "}" }} - {{ p.value }}
                      </li>
                    </ul>
                  </template>
                  <v-card-actions>
                    <v-btn @click="copyRelease(release)">Copy</v-btn>
                    <v-btn @click="editRelease(release)">Edit</v-btn>
                    <v-spacer />
                    <v-btn @click="deleteRelease(release)">Delete</v-btn>
                  </v-card-actions>
                </v-card>
              </v-col></v-row
            ></v-container
          >
          <v-btn @click="addRelease">Add</v-btn>
        </v-sheet>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="6">
        <v-sheet class="pa-5" color="grey-lighten-2">
          Platforms
          <v-container>
            <v-row>
              <v-col v-for="platform in config.platforms" cols="12">
                <v-card class="ma-5">
                  <template #title>
                    {{ platform.name }}
                  </template>
                  <template #text>
                    <ul>
                      <li v-for="p in platform.properties">
                        {{ "${platform." + p.name + "}" }} - {{ p.value }}
                      </li>
                    </ul>
                  </template>
                  <v-card-actions>
                    <v-btn @click="copyPlatform(platform)">Copy</v-btn>
                    <v-btn @click="editPlatform(platform)">Edit</v-btn>
                    <v-spacer />
                    <v-btn @click="deletePlatform(platform)">Delete</v-btn>
                  </v-card-actions>
                </v-card>
              </v-col></v-row
            ></v-container
          >
          <v-btn @click="addPlatform">Add</v-btn>
        </v-sheet>
      </v-col>
      <v-col cols="6">
        <v-sheet class="pa-5" color="grey-lighten-3">
          Servers
          <v-container>
            <v-row>
              <v-col v-for="server in config.servers" cols="12">
                <v-card class="ma-5">
                  <template #title>
                    {{ server.name }}
                  </template>
                  <template #text>
                    <ul>
                      <li v-for="p in server.properties">
                        {{ "${server." + p.name + "}" }} - {{ p.value }}
                      </li>
                    </ul>
                  </template>
                  <v-card-actions>
                    <v-btn @click="copyServer(server)">Copy</v-btn>
                    <v-btn @click="editServer(server)">Edit</v-btn>
                    <v-spacer />
                    <v-btn @click="deleteServer(server)">Delete</v-btn>
                  </v-card-actions>
                </v-card>
              </v-col></v-row
            ></v-container
          >
          <v-btn @click="addServer">Add</v-btn>
        </v-sheet>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { Config } from "@/domain/config";
import { PropType, defineComponent } from "vue";

export default defineComponent({
  props: {
    config: {
      type: Object as PropType<Config>,
      required: true,
    },
  },
  methods: {
    editGlobalVars() {
      this.$emit("editGlobalVars");
    },
    editRelease(release: CommonShape) {
      this.$emit("editRelease", release);
    },
    copyRelease(release: CommonShape) {
      this.$emit("copyRelease", release);
    },
    deleteRelease(release: CommonShape) {
      this.$emit("deleteRelease", release);
    },
    addRelease() {
      this.$emit("addRelease");
    },
    editPlatform(platform: CommonShape) {
      this.$emit("editPlatform", platform);
    },
    copyPlatform(platform: CommonShape) {
      this.$emit("copyPlatform", platform);
    },
    deletePlatform(platform: CommonShape) {
      this.$emit("deletePlatform", platform);
    },
    addPlatform() {
      this.$emit("addPlatform");
    },
    editServer(server: CommonShape) {
      this.$emit("editServer", server);
    },
    copyServer(server: CommonShape) {
      this.$emit("copyServer", server);
    },
    deleteServer(server: CommonShape) {
      this.$emit("deleteServer", server);
    },
    addServer() {
      this.$emit("addServer");
    },
  },
});
</script>
