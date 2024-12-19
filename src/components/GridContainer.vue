<script lang="ts" setup>
import { computed, onMounted, ref, toRefs } from "vue";
import { VueDraggable } from "vue-draggable-plus";
import { GroupOptions, SortableEvent } from "sortablejs";
const array = defineModel<any[]>({
  default: [],
});
const props = defineProps<{
  gridwidth: number;
  gridheight: number;
  padding?: number;
  group?: string | GroupOptions | undefined;
  clone?: ((element: any) => any) | undefined;
  onAdd?: ((event: SortableEvent) => void) | undefined;
  setData?:
    | ((dataTransfer: DataTransfer, draggedElement: HTMLElement) => void)
    | undefined;
}>();
const { gridwidth, gridheight, padding } = toRefs(props);
onMounted(() => {
  updateElementHeight();
  window.addEventListener("resize", updateElementHeight);
});

const containerref = ref<HTMLDivElement>();
const gridcontainerwidth = ref(0);
const updateElementHeight = function () {
  if (containerref.value) {
    gridcontainerwidth.value = containerref.value.offsetWidth;
  }
};

const gridcontainerheight = computed(() => {
  return (
    Math.ceil(
      array.value.length /
        Math.trunc(gridcontainerwidth.value / gridwidth.value)
    ) *
    (gridheight.value + 10)
  );
});
</script>

<template>
  <div class="container" ref="containerref">
    <VueDraggable
      v-model="array"
      class="gridcontainer"
      :animation="150"
      :group="group"
      :setData="setData"
      :onAdd="onAdd"
      :clone="clone"
      :style="{
        gridTemplateColumns: `repeat(auto-fill,${gridwidth}px)`,
        gridTemplateRows: `repeat(auto-fit,${gridheight}px)`,
        height: `${gridcontainerheight}px`,
        gap: `${padding}px`,
      }"
    >
      <div
        :data-lnk="JSON.stringify(item)"
        v-for="(item, index) in array"
        :key="item"
        :style="{ width: `${gridwidth}px`, height: `${gridheight}px` }"
      >
        <slot
          :item="item"
          :index="index"
          :style="{ width: `${gridwidth}px`, height: `${gridheight}px` }"
        ></slot>
      </div>
    </VueDraggable>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100%;
  overflow: hidden;
  overflow-y: scroll;
}

.gridcontainer {
  width: 100%;
  height: 100%;
  display: grid;
  grid-auto-flow: row;
  justify-items: center;
  align-items: center;
  justify-content: center;
  min-height: 100%;
}
</style>
