<script setup lang="ts">
import { RunTomcat } from "@/domain/taskTemplate";
import { default as FormWrapper, FormMethods } from "./FormWrapper.vue";
const emit = defineEmits<{
  saveRequested: [task: RunTomcat];
  closeRequested: [];
}>();
</script>

<template>
  <FormWrapper
    @save="saveWasRequested"
    @close="$emit('closeRequested')"
    @initFormMethods="initFormMethods"
  >  
    <template #default>
      <v-container>
        <v-row>
          <v-col cols="6">            
            <v-text-field
              v-model="model.listenPort"
              label="Listening port"
            ></v-text-field>
          </v-col>     
        </v-row>
        <v-row>
          <v-col cols="6">            
            <v-text-field
              v-model="model.jdpaPort"
              label="Jdpa port"
            ></v-text-field>
          </v-col>     
        </v-row>
        <v-row>
          <v-col cols="6">
            <v-textarea
              v-model="model.catalinaOpts"
              label="Catalina opts"
            ></v-textarea>
          </v-col>
          <v-col cols="6">
            {{ catalinaOptsComputed }}
          </v-col>          
        </v-row>

      </v-container>
    </template>
    <template #buttons> </template>
  </FormWrapper>
</template>

<script lang="ts">
import { PropType, defineComponent } from "vue";

interface State {
  formMethods: FormMethods | null;
  model: RunTomcat;
}

export default defineComponent({
  props: {
    task: {
      type: Object as PropType<RunTomcat>,
      required: true,
    },
  },
  methods: {
    initFormMethods(formMethods: FormMethods) {
      this.formMethods = formMethods;
    },
    saveWasRequested() {
        //TODO check
        this.$emit("saveRequested", this.model);
    }
  },
  data(): State {
    return {
      model: {...this.task},
      formMethods: null
    };
  },
  computed: {
    catalinaOptsComputed() {     
      return this.model.catalinaOpts;
    }
  }
});
</script>
