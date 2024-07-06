<script setup lang="ts">
console.log("Hello from Nuxt and Vue 3!");
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

async function getAllItems(): Promise<Array<any>> {
    console.log("Fetching items...");
    try {
        const testSync = await invoke("test_sync");
        console.log("test_sync:", testSync);

        return testSync as Array<any>;

        } catch (error: any) {
        console.error("Error retrieving items:", error);
        return [];
    }
}

// Usage example
const items = ref<Array<any>>([]);

async function fetchItems() {
    items.value = await getAllItems();
}

onMounted(() => {
    fetchItems();
});
</script>

<template>
    <div>
        <h1>Items</h1>
        <p v-if="items.length === 0">No items found. If you see this... something went wrong.</p>
        <ul>
            <li v-for="item in items" :key="item.id">
                {{ item.text }}
            </li>
        </ul>
        <!-- <NuxtWelcome /> -->
    </div>
</template>
