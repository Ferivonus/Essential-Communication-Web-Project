<template>
  <div class="update-contact">
    <h2>Update Contact</h2>
    <form @submit.prevent="updateContact">
      <!-- Basic Info -->
      <div v-if="!sectionToUpdate || sectionToUpdate === 'basic'">
        <label for="id">ID:</label>
        <input id="id" v-model="contact.id" type="text" required />

        <label for="nick">Nick:</label>
        <input id="nick" v-model="contact.nick" type="text" required />

        <label for="age">Age:</label>
        <input id="age" v-model.number="contact.age" type="number" />

        <label for="location">Location:</label>
        <input id="location" v-model="contact.location" type="text" />

        <label for="occupation">Occupation:</label>
        <input id="occupation" v-model="contact.occupation" type="text" />

        <label for="extra_info">Extra Info:</label>
        <textarea id="extra_info" v-model="contact.extra_info"></textarea>

        <button
          type="button"
          class="update-phone-button"
          @click="showSection('phone')"
        >
          Update Phone Info
        </button>
        <button
          type="button"
          class="update-socials-button"
          @click="showSection('socials')"
        >
          Update Social Media
        </button>
      </div>

      <!-- Phone Info -->
      <div v-if="sectionToUpdate === 'phone'">
        <label for="phone">Phone Number:</label>
        <input id="phone" v-model="contact.phone" type="tel" />

        <button type="button" class="update-phone-button" @click="updatePhone">
          Update Phone
        </button>
        <button type="button" class="back-button" @click="showSection('basic')">
          Back to Basic Info
        </button>
      </div>

      <!-- Social Media Info -->
      <div v-if="sectionToUpdate === 'socials'">
        <label for="facebook">Facebook:</label>
        <input id="facebook" v-model="contact.facebook" type="text" />

        <label for="instagram">Instagram:</label>
        <input id="instagram" v-model="contact.instagram" type="text" />

        <label for="github">GitHub:</label>
        <input id="github" v-model="contact.github" type="text" />

        <button
          type="button"
          class="update-socials-button"
          @click="updateSocials"
        >
          Update Socials
        </button>
        <button type="button" class="back-button" @click="showSection('basic')">
          Back to Basic Info
        </button>
      </div>

      <!-- Form Submit -->
      <div v-if="!sectionToUpdate">
        <button type="submit">Update Contact</button>
      </div>
    </form>
  </div>
</template>

<script>
import { ref, onMounted } from "vue";

export default {
  props: {
    contactId: {
      type: String,
      required: true,
    },
  },
  setup(props) {
    const contact = ref({
      id: "",
      nick: "",
      age: "",
      location: "",
      occupation: "",
      extra_info: "",
      phone: "",
      facebook: "",
      instagram: "",
      github: "",
    });

    const sectionToUpdate = ref("");

    const fetchContact = async () => {
      try {
        const response = await fetch(`/api/contact/${props.contactId}`);
        if (response.ok) {
          contact.value = await response.json();
        } else {
          console.error("Error fetching contact:", response.statusText);
        }
      } catch (error) {
        console.error("Error fetching contact:", error);
      }
    };

    const showSection = (section) => {
      sectionToUpdate.value = section;
    };

    const updateContact = async () => {
      try {
        await fetch(`/api/update_contact/${props.contactId}`, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(contact.value),
        });
        alert("Contact updated successfully");
      } catch (error) {
        console.error("Error updating contact:", error);
        alert("An error occurred while updating the contact");
      }
    };

    const updatePhone = async () => {
      try {
        await fetch(`/api/update_phone/${props.contactId}`, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            id: contact.value.id,
            phone: contact.value.phone,
          }),
        });
        alert("Phone number updated successfully");
      } catch (error) {
        console.error("Error updating phone:", error);
        alert("An error occurred while updating the phone number");
      }
    };

    const updateSocials = async () => {
      try {
        await fetch(`/api/update_socials/${props.contactId}`, {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            id: contact.value.id,
            facebook: contact.value.facebook,
            instagram: contact.value.instagram,
            github: contact.value.github,
          }),
        });
        alert("Social media info updated successfully");
      } catch (error) {
        console.error("Error updating social media:", error);
        alert("An error occurred while updating social media info");
      }
    };

    onMounted(() => {
      fetchContact();
    });

    return {
      contact,
      sectionToUpdate,
      showSection,
      updateContact,
      updatePhone,
      updateSocials,
    };
  },
};
</script>

<style scoped>
.update-contact {
  max-width: 600px;
  margin: auto;
  padding: 2em;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 8px;
  color: #00ff00;
  font-family: Arial, sans-serif;
}

.update-contact h2 {
  font-size: 1.6em;
  margin-bottom: 1em;
}

.update-contact form {
  display: flex;
  flex-direction: column;
  gap: 1em; /* Ensure spacing between elements */
}

.update-contact label {
  font-weight: bold;
  margin-bottom: 0.5em;
}

.update-contact input,
.update-contact textarea {
  padding: 0.75em;
  border-radius: 4px;
  border: 1px solid #00ff00;
  background: rgba(0, 0, 0, 0.9);
  color: #00ff00;
  width: 100%; /* Full-width input fields */
  box-sizing: border-box; /* Include padding and border in the element's total width and height */
}

.update-contact textarea {
  resize: vertical; /* Allow vertical resizing only */
}

.update-contact button {
  padding: 0.6em 1.2em;
  border: none;
  border-radius: 4px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  font-size: 1em;
  margin-top: 0.5em; /* Space above buttons */
}

.update-contact button:hover {
  background-color: #0056b3;
}

.update-contact .section-button {
  margin-top: 1em;
}

.update-contact .update-phone-button,
.update-contact .update-socials-button {
  margin-right: 1em; /* Space between buttons */
}

.update-contact .back-button {
  margin-top: 1em;
  background-color: #6c757d;
}

.update-contact .back-button:hover {
  background-color: #5a6268;
}
</style>
