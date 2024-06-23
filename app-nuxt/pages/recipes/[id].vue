<script setup lang="ts">
const route = useRoute();
const config = useRuntimeConfig();

interface Product {
  id: number;
  name: string;
  description: string;
  price: number;
  url: string;
  category: string;
  image: string;
  rating: number;
}
const id = route.params.id;
console.log(route.params.id);

const { data: product, pending } = await useLazyFetch<Product>(
  `https://dummyjson.com/recipes/${id}`
);
</script>

<template>
  <v-container>
    <h1>Show Recipe Page</h1>
    <v-row v-if="pending">
      <v-col align="center" :cols="12" align-self="center">
        Loading ... <br />
        <v-progress-circular
          indeterminate
          :size="200"
          model-value="20"
        ></v-progress-circular>
      </v-col>
    </v-row>

    <v-row v-else>
      <v-col cols="6">
        <v-img
          :lazy-src="product?.image"
          :src="product?.image"
          alt="image"
          aspect-ratio="1"
        ></v-img>
      </v-col>
     
      <v-col cols="12">
        <h1 class="text-h3">{{ product?.name }}</h1>
        <p class="text-green font-weight-bold">{{ product?.rating }}</p>
        <h2>ingredients</h2>
        <template v-for="row in product.ingredients" :key="row.id">
          <div>
         
            <p>{{ row }}</p>
          </div>
        </template>
        <h2>instructions</h2>
        <template v-for="row in product.instructions" :key="row.id">
          <div>
         
            <p>{{ row }}</p>
          </div>
        </template>
        <template v-for="row in product.tags" :key="row.id">
          <div>
            <v-chip>{{ row }}</v-chip>
          </div>
        </template>
      </v-col>
    </v-row>
  </v-container>
</template>
