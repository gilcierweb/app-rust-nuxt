<template>
    <v-app>
        <AppHeader></AppHeader>

        <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
            <slot />
        </v-main>

        <AppFooter></AppFooter>
    </v-app>
</template>