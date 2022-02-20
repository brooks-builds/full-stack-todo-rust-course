<template>
  <table>
    <thead>
      <tr>
        <th v-for="(title, index) in data.titles" v-bind:key="index">
          {{ title }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(row, rowIndex) in data.data" v-bind:key="rowIndex">
        <td v-for="(cell, cellIndex) in row" v-bind:key="cellIndex">
          <slot :name="data.titles[cellIndex]" :data="cell">{{ cell }}</slot>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script>
export default {
  props: {
    data: Object,
  },
};
</script>

<style scoped>
table {
  text-align: center;
  margin: 25px auto;
}

td,
th {
  padding: 10px;
}

tr:nth-child(even) {
  background-color: #2d2929;
}
/* table {
  margin: 1rem auto;
}

td,
th {
  padding: 2rem;
} */
</style>