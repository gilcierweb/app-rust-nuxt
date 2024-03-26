<script>
  export default {
    data: () => ({
      links: [
        {name: 'Home', url:'/' },
        {name:'About Us', url:'/about' },
        {name:'Team', url:'/faq' },
        {name:'Plans', url:'/plans' },
        {name:'Privacy', url:'/privacy' },
        {name:'Terms', url:'/terms' },   
        {name:'Contact Us', url:'/contacts' },
      ],
    }),
  }
</script>

<template>
    <div>
        <v-footer class="bg-grey-lighten-1">
    <v-row justify="center" no-gutters>
      <NuxtLink v-for="row in links"
        :key="row.link" :to="row.url">
   
      <v-btn      
        :key="row.link"
        class="mx-2"
        color="white"
        rounded="xl"
        variant="text"
      >
        {{ row.name }}
      </v-btn>
    </NuxtLink>
      <v-col class="text-center mt-4" cols="12">
        {{ new Date().getFullYear() }} — <strong>Vuetify</strong>
      </v-col>
    </v-row>
  </v-footer>
    </div>
</template>