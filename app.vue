<script setup lang="ts">
console.log("Hello from Vue 3!");
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

async function getAllItems() {
    console.log("Fetching items...");
    try {
        const test = await invoke("test_command");
        console.log("Test command:", test);

        const response = await invoke("get_all_notes");
        console.log("Items retrieved:", response);

        return response;
    } catch (error) {
        console.error("Error retrieving items:", error);
        return null;
    }
}

// Usage example
const items = ref([]);

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

        <ul>
            <li v-for="item in items" :key="item.id">
                {{ item.text }}
            </li>
        </ul>
        ===
        <!-- <NuxtWelcome /> -->
    </div>
</template>
