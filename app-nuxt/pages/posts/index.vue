<script setup lang="ts">
const { $truncate } = useNuxtApp();
const config = useRuntimeConfig();

interface Posts {
    id: number;
    title: string;
    content: string;
    status: number;
}

const { pending, data: posts } = await useFetch<Posts[]>(`${config.public.ApiRustBaseUrl}/api/posts`, {
    lazy: true
    // ,server: false
})

</script>

<template>
    <v-container>
        <h1>Index Posts page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col cols="12" sm="6" md="4" lg="3" v-else v-for="row in posts" :key="row.id">
                <v-card class="mt-4">
                    <v-img height="200px" src="https://source.unsplash.com/random/300×300" cover></v-img>
                    <v-card-item>
                        <v-card-title>{{ row.title }}</v-card-title>
                        <v-card-subtitle>{{ row.content }}</v-card-subtitle>
                    </v-card-item>

                    <v-card-text>
                        <p>{{ $truncate(row.content, 70, '...') }}</p>
                        <p>{{ row.status }}</p>
                        <p>
                            <NuxtLink :to="`/posts/${row.id}`">
                                <v-btn>Details</v-btn>
                            </NuxtLink>
                        </p>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>

    </v-container>
</template>