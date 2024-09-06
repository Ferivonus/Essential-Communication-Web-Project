<script>
export default {
  name: "FormView",
  data() {
    return {
      formPages: [],
      selectedForm: null,
      error: null,
    };
  },
  mounted() {
    this.load();
  },
  methods: {
    async load() {
      try {
        const response = await fetch("https://api.example.com/form-pages");
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        this.formPages = await response.json();
      } catch (e) {
        this.error = "Failed to fetch form pages.";
        console.error(e);
      }
    },
    selectForm(title) {
      this.selectedForm =
        this.formPages.find((form) => form.title === title) || null;
    },
  },
};
</script>

<template>
  <div class="page-wrapper">
    <div class="container">
      <h1>Form Pages</h1>
      <p v-if="error" class="error">{{ error }}</p>
      <ul v-else>
        <li v-for="form in formPages" :key="form.title">
          <button @click="selectForm(form.title)">{{ form.title }}</button>
        </li>
      </ul>

      <div v-if="selectedForm">
        <h2>{{ selectedForm.title }}</h2>
        <p>Details for the form: {{ selectedForm.title }}</p>
      </div>
    </div>
  </div>
</template>
