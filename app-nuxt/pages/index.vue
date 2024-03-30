<script setup lang="ts">
const { $truncate } = useNuxtApp();
interface Products {
    id: number;
    title: string;
    description: string;
    price: number;
    url: string;   
    category: string;
    image: string;
}

const config = useRuntimeConfig();
const { pending, data: products } = await useFetch<Products[]>(`${config.public.baseURL}/products`, {
    lazy: true
    // ,server: false
})

</script>

<template>
    <v-container>
        <h1>Index page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col v-else cols="4" v-for="row in products" :key="row.id">
                <v-card width="400" class="mt-4">
                    <v-card-item>
                        <v-card-title>{{ row.title }}</v-card-title>
                        <v-card-subtitle>{{ row.description }}</v-card-subtitle>
                    </v-card-item>

                    <v-card-text>
                        <v-img :lazy-src="row.image" :src="row.image" cover alt="image" aspect-ratio="1"></v-img>
                        <p> {{ $truncate(row.description, 70, '...') }}</p>
                        <p>{{ formatNumberBR(row.price) }}</p>
                        <p>
                            <NuxtLink :to="`/products/${row.id}`">
                                <v-btn block color="success">Details</v-btn>
                            </NuxtLink>
                        </p>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>

    </v-container>
</template>