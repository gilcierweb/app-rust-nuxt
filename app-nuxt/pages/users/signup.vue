<template>
    <div>
        <h1>Sign Up Page</h1>

        <v-container fluid class="fluid">
            <v-row justify="center" align="center" class="row">

                <v-col cols="12" sm="12" md="6" xs="12" class="text-center">
                    <h1>Welcome to App Rust Nuxt</h1>
                    <p>Please complete this form to create an account</p>
                </v-col>

                <v-col cols="12">
                    <v-text-field  label="Email" required>
                    </v-text-field>
                </v-col>

                <v-col cols="12">
                    <v-text-field type="password" label="Password"
                        required></v-text-field>
                </v-col>
                
                <v-col cols="12">
                    <v-text-field type="password" label="Password"                        required></v-text-field>
                </v-col>
                
                <v-btn class="purple darken-2 white--text mt-5"> Register </v-btn>

            </v-row>
        </v-container>

    </div>
</template>