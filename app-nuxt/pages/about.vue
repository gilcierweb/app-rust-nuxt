<script setup lang="ts">

interface Posts {
    id: number;
    title: string;
    body: string;
    userId: number;
}

const { pending, data: posts } = await useFetch<Posts[]>('https://jsonplaceholder.typicode.com/posts', {
    lazy: true
})
</script>

<template>
    <v-container>
        <h1>About page</h1>
        <v-row>
            <v-col v-if="pending">
                Loading ...
                <v-progress-circular indeterminate :size="50" model-value="20"></v-progress-circular>
            </v-col>
            <v-col cols="4" v-else v-for="row in posts" :key="row.id">
                <v-card width="400" class="mt-4">
                    <v-card-item>
                        <v-card-title>{{ row.title }}</v-card-title>
                    </v-card-item>

                    <v-card-text>
                        <p>{{ row.body }}</p>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>
    </v-container>

</template>