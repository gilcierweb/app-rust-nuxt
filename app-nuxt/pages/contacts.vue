<script>
export default {
  data: () => ({
    loading: false,
  }),

  watch: {
    loading(val) {
      if (!val) return

      setTimeout(() => (this.loading = false), 2000)
    },
  },
}
</script>

<template>
  <div>
    <h1>Contacts page</h1>
    <v-form v-model="valid">
      <v-row>
        <v-col cols="12"><v-text-field label="Label" variant="outlined"></v-text-field></v-col>
        <v-col cols="12"><v-text-field label="Label" variant="outlined"></v-text-field></v-col>
        <v-col cols="12"><v-text-field label="Label" variant="outlined"></v-text-field></v-col>
        <v-col cols="12"><v-text-field label="Label" variant="outlined"></v-text-field></v-col>
      </v-row>
      <v-btn :disabled="loading" :loading="loading" class="text-none mb-4" color="indigo-darken-3" size="x-large"
        variant="flat" block @click="loading = !loading">
        Verify and continue
      </v-btn>

      <v-btn class="text-none" color="grey-lighten-3" size="x-large" variant="flat" block>
        Cancel
      </v-btn>
    </v-form>
  </div>
</template>