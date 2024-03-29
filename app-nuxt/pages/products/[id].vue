<script setup lang="ts">
const route = useRoute();
const config = useRuntimeConfig();

interface Product {
    id: number;
    title: string;
    description: string;
    price: number;
    url: string;
    category: string;
    image: string;
}
const id = route.params.id;
console.log(route.params.id);

const { data: product, pending } = await useLazyFetch<Product>(`${config.public.baseURL}/products/${id}`);

</script>

<template>
    <v-container>
        <h1>Show Product Page</h1>
        <v-row v-if="pending">
            <v-col align="center" :cols="12" align-self="center">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
        </v-row>

        <v-row v-else>
            <v-col cols="6">
                <v-img :lazy-src="product?.image" :src="product?.image" alt="image" aspect-ratio="1"></v-img>
            </v-col>
            <v-col cols="6">
                <h1 class="text-h3">{{ product?.title }}</h1>
                <p class="text-green font-weight-bold">{{ formatNumberBR(product?.price) }}</p>
                <p class="my-3"><v-chip>{{ product?.category }}</v-chip></p>
                <v-btn size="x-large" color="success">Buy Now</v-btn>
            </v-col>
            <v-col cols="12">
                <p>{{ product?.description }}</p>
            </v-col>

        </v-row>
    </v-container>
</template>
