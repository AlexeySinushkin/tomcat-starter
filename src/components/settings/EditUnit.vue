<script setup lang="ts">
import { default as FormWrapper, FormMethods } from "./FormWrapper.vue";
import { nameRules } from "./validationRules.ts";
import { default as PropertyRow, FormProperty } from "./PropertyRow.vue";
import { PropType, defineComponent } from "vue";
import { appendPropertiesToHash } from "@/domain/property.ts";
import { hashable } from "@/hash/hashable.ts";
import { appendToHash } from "@/hash/hashUtils.ts";
import { IntentionTask, TaskType } from "./intentionTask";
import { CommonShape, hashCode } from "@/domain/commonShape";
const emit = defineEmits<{
  saveRequested: [newValue: CommonShape, oldValue: CommonShape];
  createRequested: [newValue: CommonShape];
  closeRequested: [];
}>();
</script>
<template>
  <FormWrapper
    @save="saveWasRequested"
    @close="closeWasRequested"
    @initFormMethods="initFormMethods"
  >
    <template #default>
      <v-container>
        <v-row v-if="captionShow!==false">
          <v-col cols="6">
            <v-text-field
              v-model="model.name"
              :rules="nameRules"
              :counter="20"
              :label="captionName"
              required
              hide-details
            ></v-text-field>
          </v-col>
        </v-row>
        <PropertyRow
          v-for="prop in model.properties"
          :prop="prop"
          :focus="prop.id === focusedPropertyId"
          @valueEnterPress="checkNewPropertySlotRequired"
          @remove="removeProperty"
        />
      </v-container>
    </template>
    <template #buttons>
      <v-btn @click="checkNewPropertySlotRequired">
        <v-icon start icon="mdi-plus"></v-icon>
        Add property
      </v-btn>
    </template>
  </FormWrapper>
</template>

<script lang="ts">
class Model implements CommonShape, hashable {
  constructor(public name: string, public properties: FormProperty[] = []) {}
  hashCode(): number {
    let hashCode = appendToHash(this.name);
    hashCode = appendPropertiesToHash(this.properties, hashCode);
    return hashCode;
  }
}

interface State {
  focusedPropertyId: string;
  formMethods: FormMethods | null;
  model: Model;
}

export default defineComponent({
  props: {
    task: {
      type: Object as PropType<IntentionTask>,
      required: true,
    },
    captionName: {
      type: Object as PropType<string>,
      required: false
    },
    captionShow: {
      type: Object as PropType<boolean>,
      required: false,
    }     
  },
  methods: {
    initFormMethods(formMethods: FormMethods) {
      this.formMethods = formMethods;
    },
    saveWasRequested() {
      const original = this.task.target;
      this.formMethods?.resetValidation();
      //remove dummy properties
      this.model.properties = this.model.properties.filter(
        (prop) => prop.name.trim().length > 0
      );
      //check duplicate names
      var map = new Map<string, number>();
      this.model.properties.forEach((prop) => {
        var count = map.get(prop.name) ?? 0;
        map.set(prop.name, count + 1);
      });
      map.forEach((value, key) => {
        if (value > 1) {
          this.formMethods?.setValidationErrorText(
            `Duplicate property name ${key}`
          );
          return;
        }
      });

      //restrict setup origin's name and target's name
      if (
        this.task.type == TaskType.Copy &&
        original.name === this.model.name
      ) {
        this.formMethods?.setValidationErrorText(
          `You must specify different name "${this.model.name}" (relative to copy source)`
        );
        return;
      }

      //validate form
      this.formMethods?.validate().then((formValid) => {
        if (formValid) {
          if (hashCode(original) === this.model.hashCode()) {
            this.formMethods?.setValidationErrorText(
              "No changes were found. Nothing to save"
            );
            return;
          }
          if (this.task.type == TaskType.Edit) {
            this.$emit("saveRequested", this.model, original);
          } else {
            this.$emit("createRequested", this.model);
          }
        }
      });
    },
    closeWasRequested() {
      this.$emit("closeRequested");
    },
    checkNewPropertySlotRequired() {
      var dummyProp = this.model.properties.find(
        (prop) => prop.name.trim().length === 0
      );
      if (!dummyProp) {
        const newDummyProp = new FormProperty();
        this.focusedPropertyId = newDummyProp.id;
        this.model.properties.push(newDummyProp);
      }
    },
    removeProperty(prop: FormProperty) {
      this.model.properties = this.model.properties.filter(
        (item) => item !== prop
      );
    },
  },
  data(): State {
    const original = this.task.target;
    if (this.task.type == TaskType.CreateNew) {
      let model = new Model("");
      model.properties.push(new FormProperty("name", "value"));
      return {
        model: model,
        formMethods: null,
        focusedPropertyId: ""
      };
    }

    let model = new Model(
      this.task.type == TaskType.Copy ? `${original.name} Copy` : original.name
    );
    original.properties.forEach((p) => {
      model.properties.push(new FormProperty(p.name, p.value));
    });
    return {
      model: model,
      formMethods: null,
      focusedPropertyId: ""
    };
  },
});
</script>
