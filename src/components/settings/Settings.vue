<script setup lang="ts">
import EditUnit from "./EditUnit.vue";
import Dashboard from "./Dashboard.vue";
</script>

<template>
  <v-container>
    <v-row>
      <v-col cols="12" v-if="mode === DisplayMode.Dashboard">
        <Dashboard
          :config="config"
          @edit-global-vars="editGlobalVarsRequested"
          @edit-release="editReleaseRequested"
          @copy-release="copyReleaseRequested"
          @delete-release="deleteReleaseRequested"
          @add-release="addReleaseRequested"
          @edit-platform="editPlatformRequested"
          @copy-platform="copyPlatformRequested"
          @delete-platform="deletePlatformRequested"
          @add-platform="addPlatformRequested"
          @edit-server="editServerRequested"
          @copy-server="copyServerRequested"
          @delete-server="deleteServerRequested"
          @add-server="addServerRequested"
        />

        <ServerInstances
          :servers="availableServerNames"
          :runs="runs"
          @update-in-config="saveServerRunsRequest"
        />        
      </v-col>
      <v-col cols="6" v-else>
        <EditUnit
          v-if="mode === DisplayMode.EditGlobalVariables"
          :task="globalVarsTask"
          caption-name=""
          :caption-show="false"
          @save-requested="saveGlobalVarsRequested"
          @close-requested="closeEditing"
        />
        <EditUnit
          v-if="mode === DisplayMode.EditRelease"
          :task="releaseTask"
          caption-name="Release name"
          @save-requested="saveReleaseRequested"
          @create-requested="createReleaseRequested"
          @close-requested="closeEditing"
        />
        <EditUnit
          v-else-if="mode === DisplayMode.EditPlatform"
          :task="platformTask"
          caption-name="Platform name"
          @save-requested="savePlatformRequested"
          @create-requested="createPlatformRequested"
          @close-requested="closeEditing"
        />
        <EditUnit
          v-else-if="mode === DisplayMode.EditServer"
          :task="serverTask"
          caption-name="Server name"
          @save-requested="saveServerRequested"
          @create-requested="createServerRequested"
          @close-requested="closeEditing"
        />
      </v-col>
    </v-row>
    <v-row v-if="mode === DisplayMode.Dashboard">
      <v-spacer/>
      <v-col cols="1">
        <v-btn @click="save">Save</v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import BackendApi from "@/backendApi";
import { Variables, ServerRun } from "@/domain/config";
import { IntentionTask, TaskType } from "./intentionTask";
import { Release } from "@/domain/release.ts";
import { Platform } from "@/domain/platform";
import { Server } from "@/domain/server";
import { CommonShape } from "@/domain/commonShape";
import ServerInstances from "./ServerInstances.vue";

enum DisplayMode {
  Dashboard,
  EditGlobalVariables,
  EditRelease,
  EditPlatform,
  EditServer,
}

interface State {
  config: Variables;
  runs: ServerRun[];
  mode: DisplayMode;
  globalVarsTask: IntentionTask;
  releaseTask: IntentionTask;
  platformTask: IntentionTask;
  serverTask: IntentionTask;
}

export default defineComponent({
  props: {
    api: {
      type: Object as PropType<BackendApi>,
      required: true,
    },
  },
  data(): State {
    return {
      config: this.api.getConfig(),
      runs: this.api.getRuns(),
      mode: DisplayMode.Dashboard,
      globalVarsTask: new IntentionTask(TaskType.CreateNew, {
        name: "",
        properties: [],
      }),
      releaseTask: new IntentionTask(TaskType.CreateNew, new Release("")),
      platformTask: new IntentionTask(TaskType.CreateNew, new Platform("")),
      serverTask: new IntentionTask(TaskType.CreateNew, new Server("")),
    };
  },
  methods: {
    editGlobalVarsRequested() {
      this.globalVarsTask = new IntentionTask(TaskType.Edit, {
        name: "",
        properties: this.config.globalVars,
      });
      this.mode = DisplayMode.EditGlobalVariables;
    },
    saveGlobalVarsRequested(newValue: CommonShape) {
      this.config.globalVars = newValue.properties;
      this.mode = DisplayMode.Dashboard;
    },
    copyReleaseRequested(release: CommonShape) {
      this.releaseTask = new IntentionTask(
        TaskType.Copy,
        new Release(release.name, release.properties)
      );
      this.mode = DisplayMode.EditRelease;
    },
    editReleaseRequested(release: CommonShape) {
      this.releaseTask = new IntentionTask(
        TaskType.Edit,
        new Release(release.name, release.properties)
      );
      this.mode = DisplayMode.EditRelease;
    },
    addReleaseRequested() {
      this.releaseTask = new IntentionTask(TaskType.CreateNew, new Release(""));
      this.mode = DisplayMode.EditRelease;
    },
    saveReleaseRequested(newValue: CommonShape, oldValue: Release) {
      let releases = this.config.releases.filter(
        (r) => r.name !== oldValue.name
      );
      releases.push(newValue);
      this.config.releases = releases;
      this.mode = DisplayMode.Dashboard;
    },
    createReleaseRequested(newValue: CommonShape) {
      this.config.releases.push(newValue);
      this.mode = DisplayMode.Dashboard;
    },
    deleteReleaseRequested(release: CommonShape) {
      this.config.releases = this.config.releases.filter(
        (r) => r.name !== release.name
      );
    },

    copyPlatformRequested(release: CommonShape) {
      this.platformTask = new IntentionTask(
        TaskType.Copy,
        new Platform(release.name, release.properties)
      );
      this.mode = DisplayMode.EditPlatform;
    },
    editPlatformRequested(release: CommonShape) {
      this.platformTask = new IntentionTask(
        TaskType.Edit,
        new Platform(release.name, release.properties)
      );
      this.mode = DisplayMode.EditPlatform;
    },
    addPlatformRequested() {
      this.platformTask = new IntentionTask(
        TaskType.CreateNew,
        new Platform("")
      );
      this.mode = DisplayMode.EditPlatform;
    },
    savePlatformRequested(newValue: CommonShape, oldValue: Release) {
      let platforms = this.config.releases.filter(
        (r) => r.name !== oldValue.name
      );
      platforms.push(newValue);
      this.config.platforms = platforms;
      this.mode = DisplayMode.Dashboard;
    },
    createPlatformRequested(newValue: CommonShape) {
      this.config.platforms.push(newValue);
      this.mode = DisplayMode.Dashboard;
    },
    deletePlatformRequested(release: CommonShape) {
      this.config.platforms = this.config.platforms.filter(
        (r) => r.name !== release.name
      );
    },

    copyServerRequested(server: CommonShape) {
      this.serverTask = new IntentionTask(
        TaskType.Copy,
        new Server(server.name, server.properties)
      );
      this.mode = DisplayMode.EditServer;
    },
    editServerRequested(server: CommonShape) {
      this.serverTask = new IntentionTask(
        TaskType.Edit,
        new Server(server.name, server.properties)
      );
      this.mode = DisplayMode.EditServer;
    },
    addServerRequested() {
      this.serverTask = new IntentionTask(TaskType.CreateNew, new Server(""));
      this.mode = DisplayMode.EditServer;
    },
    saveServerRequested(newValue: CommonShape, oldValue: Release) {
      let servers = this.config.servers.filter((r) => r.name !== oldValue.name);
      servers.push(newValue);
      this.config.servers = servers;
      this.mode = DisplayMode.Dashboard;
    },
    createServerRequested(newValue: CommonShape) {
      this.config.servers.push(newValue);
      this.mode = DisplayMode.Dashboard;
    },
    deleteServerRequested(server: CommonShape) {
      this.config.servers = this.config.servers.filter(
        (r) => r.name !== server.name
      );
    },
    closeEditing() {
      this.mode = DisplayMode.Dashboard;
    },
    saveServerRunsRequest(runs: ServerRun[]) {
      this.runs = runs;
    },
    save() {
      console.log([this.config, this.runs])
    },    
  },
  computed: {
    availableServerNames() {     
      return this.config.servers.map((server)=>server.name);
    } 
  }
});
</script>
