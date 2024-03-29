<script setup lang="ts">
const { $truncate } = useNuxtApp();
const route = useRoute();
const config = useRuntimeConfig();

interface Todo {
    id: number;
    title: string;
    description: string; 
}
const id = route.params.id;
console.log(route.params.id);

const { data: todo, pending } = await useLazyFetch<Todo>(`${config.public.ApiRustBaseUrl}/api/todos/${id}`);

</script>

<template>
    <v-container>
        <h1>Show Post Page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col v-else cols="12">
                <h2>{{ todo?.title }}</h2>
                <p> {{ todo?.description }}</p>
                <h1>{{ $route.params.id }} - {{ route.params.id }}</h1>
           
            </v-col>

        </v-row>
    </v-container>
</template>
