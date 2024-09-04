<script>
export default {
  data() {
    return {
      contact: {
        id: "",
        nick: "",
        age: "",
        location: "",
        occupation: "",
        extra_info: "",
      },
    };
  },
  methods: {
    handleAgeInput(event) {
      const value = event.target.value;
      this.contact.age = value === "" ? "" : value;
    },
    async addContact() {
      const { id, nick, age, location, occupation, extra_info } = this.contact;

      // Validate required fields
      if (!id || !nick) {
        alert("ID and Nick are required");
        return;
      }

      // Convert age to a number before processing
      const ageNumber = parseInt(age, 10);
      if (age !== "" && isNaN(ageNumber)) {
        alert("Age must be a valid number");
        return;
      }

      const ageToSubmit = age === "" ? null : ageNumber;

      try {
        // Replace this with your actual API endpoint
        await fetch("/api/add_contact_other_client", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            id,
            nick,
            age: ageToSubmit,
            location: location || null,
            occupation: occupation || null,
            extra_info: extra_info || null,
          }),
        });

        alert("Contact added successfully");

        // Optionally, reset the form or provide feedback
        this.contact = {
          id: "",
          nick: "",
          age: "",
          location: "",
          occupation: "",
          extra_info: "",
        };
      } catch (error) {
        console.error("Error adding contact:", error);
        alert("An error occurred while adding the contact");
      }
    },
  },
};
</script>

<template>
  <div>
    <h2>Add Contact</h2>
    <form @submit.prevent="addContact">
      <label for="id">ID:</label>
      <input id="id" v-model="contact.id" type="text" required />

      <label for="nick">Nick:</label>
      <input id="nick" v-model="contact.nick" type="text" required />

      <label for="age">Age:</label>
      <input
        id="age"
        v-model.number="contact.age"
        type="number"
        @input="handleAgeInput"
      />

      <label for="location">Location:</label>
      <input id="location" v-model="contact.location" type="text" />

      <label for="occupation">Occupation:</label>
      <input id="occupation" v-model="contact.occupation" type="text" />

      <label for="extra_info">Extra Info:</label>
      <textarea id="extra_info" v-model="contact.extra_info"></textarea>

      <button type="submit">Add Contact</button>
    </form>
  </div>
</template>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: 1em;
}
label {
  font-weight: bold;
}
input,
textarea {
  padding: 0.5em;
  border-radius: 4px;
  border: 1px solid #ccc;
}
button {
  padding: 1em;
  border: none;
  border-radius: 4px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
}
button:hover {
  background-color: #0056b3;
}
</style>
