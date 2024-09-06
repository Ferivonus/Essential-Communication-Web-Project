<template>
  <div class="add-contact">
    <h2>Add Contact</h2>
    <form @submit.prevent="addContact">
      <label for="userKey">User Key:</label>
      <input id="userKey" v-model="contact.userKey" type="text" required />

      <label for="id">ID:</label>
      <input id="id" v-model="contact.id" type="text" required />

      <label for="contactType">Contact Type:</label>
      <select id="contactType" v-model="contact.contactType" required>
        <option value="my-server">My Server</option>
        <option value="other-server">Other Server</option>
      </select>

      <button type="submit">Add Contact</button>
    </form>
  </div>
</template>

<script>
import { ref, computed } from "vue";

export default {
  data() {
    return {
      contact: {
        userKey: "",
        id: "",
        contactType: "my-server", // Default to "My Server"
      },
    };
  },
  methods: {
    async addContact() {
      const { userKey, id, contactType } = this.contact;

      // Validate required fields
      if (!userKey || !id) {
        alert("User Key and ID are required");
        return;
      }

      try {
        // Replace this with your actual API call
        await fetch("/api/add_contact", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            userKey,
            id,
            contactType,
          }),
        });

        alert("Contact added successfully");

        // Optionally, reset the form
        this.contact = {
          userKey: "",
          id: "",
          contactType: "my-server",
        };
      } catch (error) {
        console.error("Error adding contact:", error);
        alert("An error occurred while adding the contact");
      }
    },
  },
};
</script>

<style scoped>
.add-contact form {
  display: flex;
  flex-direction: column;
  gap: 1em;
}
.add-contact label {
  font-weight: bold;
}
.add-contact input,
.add-contact select {
  padding: 0.5em;
  border-radius: 4px;
  border: 1px solid #ccc;
}
.add-contact button {
  padding: 1em;
  border: none;
  border-radius: 4px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
}
.add-contact button:hover {
  background-color: #0056b3;
}
</style>
