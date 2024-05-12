<template>
    <div>
        <v-navigation-drawer>
            <v-list-item title="My Application" subtitle="Vuetify"></v-list-item>
            <v-divider></v-divider>
            <NuxtLink to="/">
                <v-list-item link>
                    Home page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/posts">
                <v-list-item link>
                    Posts page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/todos">
                <v-list-item link>
                    Todos page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/users">
                <v-list-item link>
                    Users page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/profiles">
                <v-list-item link>
                    Profiles page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/about">
                <v-list-item link>
                    About page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/privacy">
                <v-list-item link>
                    Privacy page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/terms">
                <v-list-item link>
                    Terms page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/plans">
                <v-list-item link>
                    Plans page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/faq">
                <v-list-item link>
                    FAQ page
                </v-list-item>
            </NuxtLink>
            <NuxtLink to="/contacts">
                <v-list-item link>
                    Contacts page
                </v-list-item>
            </NuxtLink>
        </v-navigation-drawer>
    </div>
</template>