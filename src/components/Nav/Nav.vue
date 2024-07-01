<template>
  <MDBNavbar light bg="light" expand="lg" container>
    <MDBNavbarBrand href="#">Navbar</MDBNavbarBrand>
    <MDBNavbarToggler
      target="#navbarNav"
      @click="collapse2 = !collapse2"
    ></MDBNavbarToggler>
    <MDBCollapse id="navbarNav" v-model="collapse2">
      <MDBNavbarNav>
        <MDBNavbarItem to="#" active> Home </MDBNavbarItem>
        <MDBNavbarItem to="#"> Features </MDBNavbarItem>
        <MDBNavbarItem to="#"> Pricing </MDBNavbarItem>
        <MDBNavbarItem to="#" disabled> Disabled </MDBNavbarItem>
      </MDBNavbarNav>
    </MDBCollapse>
  </MDBNavbar>
</template>

<script setup lang="ts">
  import {
    MDBNavbar,
    MDBNavbarToggler,
    MDBNavbarBrand,
    MDBNavbarNav,
    MDBNavbarItem,
    MDBCollapse,
  } from 'mdb-vue-ui-kit';
  import { ref } from 'vue';

  const collapse2 = ref(false);
</script>
