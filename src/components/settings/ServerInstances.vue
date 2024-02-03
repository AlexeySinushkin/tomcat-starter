<script setup lang="ts">
import {
  CopyWarToRandomDir,
  RunTomcat,
  Task,
  TaskType,
} from "@/domain/taskTemplate";
import EditCopyWarToRandomDir from "./EditCopyWarToRandomDir.vue";

const emit = defineEmits<{
  updateInConfig: [ServerRun[]];
}>();
</script>

<template>
  <v-container v-if="mode == DisplayMode.View">
    <v-row>
      <v-col cols="12" v-for="run in runs">
        <v-card class="ma-5">
          <template #title>
            {{ `${run.serverRunName} [${run.serverName}]` }} 
          </template>
          <template #text>
            <ol>
              <li v-for="task in run.tasks">
                <div v-if="task.type == TaskType.CopyWarRandom">
                  <h3>Copy war to new (random) directory</h3>
                  <ul>
                    <li>
                      Source war path:
                      {{ (task as CopyWarToRandomDir).sourceWarPath }}
                    </li>
                    <li>
                      Destination catalina base:
                      {{ (task as CopyWarToRandomDir).destinationCatalinaBase }}
                    </li>
                    <li>
                      Destination war name:
                      {{ (task as CopyWarToRandomDir).destinationWarName }}
                    </li>
                  </ul>
                  <v-btn @click="editCopyWarToRandomDir(task, run)">Edit</v-btn>
                  <v-btn @click="removeTask(task, run)">Remove</v-btn>
                </div>
                <div v-if="task.type == TaskType.RunTomcat">
                  <h3>Run Tomcat</h3>
                  <ul>
                    <li>
                      Listening port:
                      {{ (task as RunTomcat).listenPort }}
                    </li>
                    <li>
                      JDPA port:
                      {{ (task as RunTomcat).jdpaPort }}
                    </li>
                    <li>
                      Catalina opts:
                      {{ (task as RunTomcat).catalinaOpts }}
                    </li>
                  </ul>
                  <v-btn @click="editRunTomcat(task, run)">Edit</v-btn>
                  <v-btn @click="removeTask(task, run)">Remove</v-btn>
                </div>
              </li>
            </ol>
          </template>
          <v-card-actions>
            <v-container>
              <v-row>
                <v-col cols="3"
                  ><v-combobox
                    label="Task type"
                    v-model="newTaskType"
                    :items="[TaskType.CopyWarRandom, TaskType.RunTomcat]"
                  />
                </v-col>
                <v-col cols="1">
                  <v-btn @click="addTaskRequest(run)">Add</v-btn></v-col
                >
                <v-col cols="3">
                  <v-btn @click="removeRun(run)"
                    >Remove {{ run.serverRunName }}</v-btn
                  ></v-col
                >
                <v-spacer />
              </v-row>
            </v-container>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="3"
        ><v-combobox label="Server" v-model="newServerName" :items="servers" />
      </v-col>      
      <v-col cols="3">
        <v-text-field v-model="newServerRunName" label="Run name"></v-text-field
      ></v-col>
      <v-col cols="3">
        <v-btn @click="addServerRun" :disabled="!serverChoosen">Add Server's run</v-btn></v-col
      >
      <v-spacer />
    </v-row>
  </v-container>

  <EditCopyWarToRandomDir
    :task="(editTask?.task as CopyWarToRandomDir)"
    @close-requested="mode = DisplayMode.View"
    @save-requested="saveRequested"
    v-else-if="mode === DisplayMode.EditCopyWarToRandomDir"
  />
  <EditRunTomcat
    :task="(editTask?.task as RunTomcat)"
    @close-requested="mode = DisplayMode.View"
    @save-requested="saveRequested"
    v-else-if="mode === DisplayMode.EditRunTomcat"
  />
</template>

<script lang="ts">
import { ServerRun } from "@/domain/config";
import { PropType, defineComponent } from "vue";
import EditRunTomcat from "./EditRunTomcat.vue";

enum DisplayMode {
  View,
  EditCopyWarToRandomDir,
  EditRunTomcat,
}

type EditingTask = {
  task: Task;
  run: ServerRun;
};

interface State {
  mode: DisplayMode;
  newServerRunName: string;
  newServerName: string;
  newTaskType: TaskType;
  editTask?: EditingTask;
}

export default defineComponent({
  props: {
    runs: {
      type: Object as PropType<ServerRun[]>,
      required: true,
    },
    servers: {
      type: Object as PropType<string[]>,
      required: true,
    },
  },
  data(): State {
    return {
      mode: DisplayMode.View,
      newServerRunName: "",
      newServerName: "",
      newTaskType: TaskType.CopyWarRandom,
    };
  },
  methods: {
    editCopyWarToRandomDir(task: Task, run: ServerRun) {
      this.editTask = { task, run };
      this.mode = DisplayMode.EditCopyWarToRandomDir;
    },
    saveRequested(task: Task) {
      if (this.editTask) {
        this.editTask.run.tasks = this.editTask.run.tasks.map((t) =>
          t.id == task.id ? task : t
        );
        this.emitChanges();
      }
      this.mode = DisplayMode.View;
    },
    editRunTomcat(task: Task, run: ServerRun) {
      this.editTask = { task, run };
      this.mode = DisplayMode.EditRunTomcat;
    },
    removeTask(task: Task, run: ServerRun) {
      const changedRun = { ...run };
      changedRun.tasks = changedRun.tasks.filter((t) => t.id != task.id);
      const list = this.runs.map((r) =>
        r.serverRunName == changedRun.serverRunName ? changedRun : r
      );
      this.emitChanges(list);
    },
    addTaskRequest(run: ServerRun) {
      let newTask: Task | null = null;

      if (this.newTaskType == TaskType.CopyWarRandom) {
        newTask = new CopyWarToRandomDir("", "", "");
      } else if (this.newTaskType == TaskType.RunTomcat) {
        newTask = new RunTomcat("", 8080, 12345);
      } else {
        throw new Error("Not implemented");
      }

      if (newTask != null) {
        run.tasks.push(newTask);
        this.emitChanges();
      }
    },
    removeRun(run: ServerRun) {
      const list = this.runs.filter(
        (r) => r.serverRunName !== run.serverRunName
      );
      this.emitChanges(list);
    },
    addServerRun() {
      let newName = this.newServerRunName.trim();
      if (newName) {
        const sameNameAlreadyExist = this.runs.find(
          (r) => r.serverRunName === newName
        );
        if (sameNameAlreadyExist) {
          newName = newName + ` Copy ${Date.now()}`;
        }
        this.runs.push({ serverRunName: newName, serverName: this.newServerName, tasks: [] });
        this.emitChanges();
        this.newServerRunName = "";
      }
    },
    emitChanges(list?: ServerRun[]) {
      if (list) {
        this.$emit("updateInConfig", list);
      } else {
        this.$emit("updateInConfig", this.runs);
      }
    },
  },
  computed: {
    serverChoosen() {     
      if(this.newServerName && this.newServerRunName.trim()){
        return true;
      }
      return false;
    } 
  }
});
</script>
