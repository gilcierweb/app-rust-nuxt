<script setup lang="ts">
interface Photos {
    id: number;
    title: string;
    url: string;
    thumbnailUrl: string;
}

const { pending, data: photos } = await useFetch<Photos[]>('https://jsonplaceholder.typicode.com/photos', {
    lazy: true
    // ,server: false
})
</script>

<template>
    <v-container>
        <h1>Index page</h1>
        <v-row>
            <v-col v-if="pending">
                Loading ...
                <v-progress-circular indeterminate :size="50" model-value="20"></v-progress-circular>
            </v-col>
            <v-col v-else cols="4" v-for="row in photos" :key="row.id">
                <v-card width="400" class="mt-4">
                    <v-card-item>
                        <v-card-title>{{ row.title }}</v-card-title>

                        <v-card-subtitle>{{ row.title }}</v-card-subtitle>
                    </v-card-item>

                    <v-card-text>
                        <v-img class="bg-grey-lighten-2" max-height="125" src="https://picsum.photos/350/165?random"
                            aspect-ratio="1"></v-img>
                        <v-img :lazy-src="row.thumbnailUrl" :src="row.url" alt="image" aspect-ratio="1"></v-img>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>

    </v-container>
</template>