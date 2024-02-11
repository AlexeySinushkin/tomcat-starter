<!--Provides close & submit buttons, common style-->
<template>
  <v-form ref="form" v-model="valid" @submit.prevent="$emit('save')">
    <v-container>
      <v-row justify="end">
        <v-spacer />
        <v-col cols="1"
          ><v-btn rounded="xl" @click.prevent="$emit('close')"
            ><v-icon center icon="mdi-close" /></v-btn
        ></v-col>
      </v-row>
      <v-row>
        <v-col cols="12"><slot></slot></v-col>
      </v-row>
      <v-row justify="end">
        <v-col cols="4">
          {{ errorText }}
        </v-col>
        <v-col cols="8">
          <slot name="buttons"></slot>
          <v-btn type="submit">
            <v-icon start icon="mdi-content-save-outline"></v-icon>
            Save
          </v-btn>
        </v-col>
      </v-row>
    </v-container>
  </v-form>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export interface FormMethods {
  validate(): Promise<boolean>;
  setValidationErrorText(text: string): void;
  reset(): void;
  resetValidation(): void;
}
interface State {
  valid: boolean;
  errorText: string;
}

export default defineComponent({
  data(): State {
    return { valid: true, errorText: "" };
  },
  mounted() {
    const formMethods: FormMethods = {
      validate: async () => await this.validate(),
      setValidationErrorText: (text) => this.setValidationErrorText(text),
      reset: () => this.reset(),
      resetValidation: () => this.resetValidation(),
    };
    this.$emit("initFormMethods", formMethods);
  },
  methods: {
    async validate(): Promise<boolean> {
      const { valid } = await (this.$refs['form'] as any).validate();
      return valid as boolean;
    },
    setValidationErrorText(text: string): void{
        this.errorText = text;
        this.valid = false;
    },
    reset(): void {
      (this.$refs['form'] as any).reset();
    },
    resetValidation(): void {
      this.errorText="";
      (this.$refs['form'] as any).resetValidation();
    },
  },
});
</script>
