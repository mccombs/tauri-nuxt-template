<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

async function getAllItems() {
    console.log("Fetching items...");
    try {
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

fetchItems();
</script>

<template>
    <div>
        <h1>Items</h1>

        ===
        <!-- <NuxtWelcome /> -->
    </div>
</template>
