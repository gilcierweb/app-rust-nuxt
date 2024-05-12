<script lang="ts" setup>
const { $truncate } = useNuxtApp();
const config = useRuntimeConfig();

interface Profiles {
    id: number;
    first_name: String,
    last_name: String,
    full_name: String,
    nickname: String,
    bio: String,
    birthday: Date,
    avatar: String, 
    phone: number,
    user_id: String,
}

const { pending, data: profiles } = await useFetch<Profiles[]>(`${config.public.ApiRustBaseUrl}/api/v1/profiles`, {
    lazy: true
    // ,server: false
})
</script>

<template>
   <v-container>
        <h1>Index Profiles Page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col cols="12" sm="6" md="4" lg="3" v-else v-for="row in profiles" :key="row.id">
                <v-card class="mt-4">
                    <v-img :lazy-src="row.avatar" :src="row.avatar" cover alt="image" aspect-ratio="1"></v-img>

                    <v-card-item>
                        <v-card-title>{{ row.first_name }}</v-card-title>
                        <v-card-subtitle>{{ row.last_name }}</v-card-subtitle>
                    </v-card-item>

                    <v-card-text>                       
                        <p>{{ row.full_name}} - {{ row.nickname}}</p>
                        <p>{{ $truncate(row.bio, 70, '...') }}</p>
                        <p>
                            <NuxtLink :to="`/profiles/${row.id}`">
                                <v-btn>Details</v-btn>
                            </NuxtLink>
                        </p>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>

    </v-container>
</template>
