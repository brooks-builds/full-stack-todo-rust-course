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

@media (max-width: 376px) {
  * {
    padding: 0;
    margin: 0;
  }

  tr {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  td {
    inline-size: 340px;
    overflow-wrap: break-word;
  }

  th {
    display: none;
  }

  tr > td:first-child::before {
    content: "Priority: ";
  }

  tr > td:nth-child(2)::before {
    content: "Completed: ";
  }
}
/* table {
  margin: 1rem auto;
}

td,
th {
  padding: 2rem;
} */
</style>