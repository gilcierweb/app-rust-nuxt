<script setup lang="ts">
const route = useRoute();
const config = useRuntimeConfig();

interface Profile {
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
const id = route.params.id;
console.log(route.params.id);

const { data: profile, pending } = await useLazyFetch<Profile>(`${config.public.ApiRustBaseUrl}/api/v1/profiles/${id}`);

</script>

<template>
    <v-container>
        <h1>Show Profile Page</h1>
        <v-row>

            <v-col align="center" :cols="12" align-self="center" v-if="pending">
                Loading ... <br>
                <v-progress-circular indeterminate :size="200" model-value="20"></v-progress-circular>
            </v-col>
            <v-col v-else cols="12">
                <v-img :lazy-src="profile?.avatar" :src="profile?.avatar" cover alt="image" aspect-ratio="1"></v-img>
                <h2>{{ profile?.first_name }} {{ profile?.last_name }}</h2>
                <p>{{ profile?.nickname }} - {{ profile?.birthday }}</p>
                <p>{{ profile?.phone }}</p>
                <p>{{ profile?.bio }}</p>
                <h1>{{ $route.params.id }} - {{ route.params.id }}</h1>

            </v-col>

        </v-row>
    </v-container>
</template>
