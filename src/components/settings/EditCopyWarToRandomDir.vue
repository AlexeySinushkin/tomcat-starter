<script setup lang="ts">
import { CopyWarToRandomDir } from "@/domain/taskTemplate";
import { default as FormWrapper, FormMethods } from "./FormWrapper.vue";
const emit = defineEmits<{
  saveRequested: [task: CopyWarToRandomDir];
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
        Please, use ${random} variable
      <v-container>
        <v-row>
          <v-col cols="6">
            <v-textarea
              v-model="model.sourceWarPath"
              label="Source war path"
            ></v-textarea>
          </v-col>
          <v-col cols="6">
            {{ sourceWarPathComputed }}
          </v-col>          
        </v-row>
        <v-row>
          <v-col cols="6">
            <v-textarea
              v-model="model.destinationCatalinaBase"
              label="Destination Catalina base"
            ></v-textarea>
          </v-col>
          <v-col cols="6">
            {{ destinationCatalinaBaseComputed }}
          </v-col>          
        </v-row>
        <v-row>
          <v-col cols="6">            
            <v-text-field
              v-model="model.destinationWarName"
              label="Destination war name"
            ></v-text-field>
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
  model: CopyWarToRandomDir;
}

export default defineComponent({
  props: {
    task: {
      type: Object as PropType<CopyWarToRandomDir>,
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
    sourceWarPathComputed() {     
      return this.model.sourceWarPath
    },
    destinationCatalinaBaseComputed() {     
      return this.model.destinationCatalinaBase
    },    
  }
});
</script>
