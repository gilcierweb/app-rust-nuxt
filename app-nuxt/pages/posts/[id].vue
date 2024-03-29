<script setup lang="ts">
const { $truncate } = useNuxtApp();
const route = useRoute();
const config = useRuntimeConfig();

interface Post {
    id: number;
    title: string;
    content: string;
    status: boolean;
}
const id = route.params.id;
console.log(route.params.id);

const { data: post, pending } = await useLazyFetch<Post>(`${config.public.ApiRustBaseUrl}/api/posts/${id}`);

</script>

<template>
    <v-container>
        <h1>Show Post page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col v-else cols="12">
                <h2>{{ post?.title }}</h2>
                <p> {{ post?.content }}</p>
                <p>{{ post?.status }}</p>
                <h1>{{ $route.params.id }} - {{ route.params.id }}</h1>
           
            </v-col>

        </v-row>
    </v-container>
</template>
