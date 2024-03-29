<script setup lang="ts">
const { $truncate } = useNuxtApp();
const config = useRuntimeConfig();

interface Todos {
    id: number;
    title: string;
    description: string;
}

const { pending, data: todos } = await useFetch<Todos[]>(`${config.public.ApiRustBaseUrl}/api/todos`, {
    lazy: true
    // ,server: false
})

</script>

<template>
    <v-container>
        <h1>Index Todos Page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col v-else cols="4" v-for="row in todos" :key="row.id">
                <v-card width="400" class="mt-4">
                    <v-card-item>
                        <v-card-title>{{ row.title }}</v-card-title>
                        <v-card-subtitle>{{ row.description }}</v-card-subtitle>
                    </v-card-item>

                    <v-card-text>
                        <p>{{ $truncate(row.description, 70, '...') }}</p>
                        <p>
                            <NuxtLink :to="`/todos/${row.id}`">
                                <v-btn>Details</v-btn>
                            </NuxtLink>
                        </p>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>

    </v-container>
</template>