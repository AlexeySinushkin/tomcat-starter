<script setup lang="ts">
import { PropertyShape } from "@/domain/property.ts";
import { nameRules } from "./validationRules.ts";
const emit = defineEmits<{
  valueEnterPress: [propRow: FormProperty];
  remove: [propRow: FormProperty];
}>();
</script>

<template>
  <v-row>
    <v-col cols="6">
      <v-text-field
        v-model="prop.name"
        :rules="nameRules"
        :counter="10"
        label="Property name"
        hide-details
        required
        ref="nameInput"
      ></v-text-field>
    </v-col>

    <v-col cols="6">
      <v-text-field
        v-model="prop.value"
        label="Property value"
        hide-details
        append-icon="mdi-close-thick"
        @keyup.native.enter="valueEnterPress"
        @click:append="remove"
      >
      </v-text-field>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import { PropType, defineComponent } from "vue";

export class FormProperty implements PropertyShape {
  public id: string;
  constructor(public name: string = "", public value: string = "") {
    this.id = `id_${Date.now()}`;
  }
}

export default defineComponent({
  props: {
    prop: {
      type: Object as PropType<FormProperty>,
      required: true,
    },
    focus: Boolean
  },
  mounted() {
    if (this.focus){
        this.focusName()
    }
  },
  watch: {
    // whenever question changes, this function will run
    focus(newValue: boolean) {
      if (newValue) {
        this.focusName();
      }
    },
  },
  methods: {
    valueEnterPress() {
      this.$emit("valueEnterPress", this.prop);
    },
    remove() {
      this.$emit("remove", this.prop);
    },
    focusName() {
        this.$refs.nameInput.focus();
    },
  },
});
</script>
