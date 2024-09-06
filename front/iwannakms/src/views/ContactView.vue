<script>
import { ref, computed } from "vue";

import AddContact from "../components/contact components/add contacts/AddContact.vue";
import UpdateContact from "../components/contact components/update contacts/UpdateContact.vue";
import ViewContacts from "../components/contact components/view contacts/ViewContacts.vue";

export default {
  name: "ContactView",
  components: {
    AddContact,
    UpdateContact,
    ViewContacts,
  },
  setup() {
    const currentSection = ref("view"); // Default to 'view' section

    const showSection = (section) => {
      currentSection.value = section;
    };

    return {
      currentSection,
      showSection,
    };
  },
};
</script>

<template>
  <div class="contacts-view">
    <h1>Contacts Management</h1>

    <p class="intro-text">
      Welcome to the Contacts Management system. Here you can efficiently manage
      your contacts with three primary functions: viewing your existing
      contacts, adding new ones, and updating existing contact information. Use
      the buttons below to navigate between these functions and keep your
      contact list up-to-date and organized.
    </p>

    <div class="button-group">
      <button
        :class="{ active: currentSection === 'view' }"
        @click="showSection('view')"
      >
        View My Contacts
      </button>
      <button
        :class="{ active: currentSection === 'add' }"
        @click="showSection('add')"
      >
        Add Contact
      </button>
      <button
        :class="{ active: currentSection === 'update' }"
        @click="showSection('update')"
      >
        Update Contact
      </button>
    </div>

    <div class="section-content">
      <ViewContacts v-if="currentSection === 'view'" />
      <AddContact v-if="currentSection === 'add'" />
      <UpdateContact v-if="currentSection === 'update'" />
    </div>

    <p class="info-text">
      <strong>View My Contacts:</strong> Here you can see a list of all your
      existing contacts. You can browse through your contact list to find and
      review the details of your contacts.
    </p>

    <p class="info-text">
      <strong>Add Contact:</strong> Use this section to add new contacts to your
      list. You will need to provide essential details such as user ID and
      contact type. Make sure to fill in all required fields to successfully add
      a new contact.
    </p>

    <p class="info-text">
      <strong>Update Contact:</strong> This section allows you to update the
      information of an existing contact. You can modify details such as
      nicknames, ages, and contact-specific information. To update a contact,
      select the contact you wish to modify, and make the necessary changes.
    </p>
  </div>
</template>

<style scoped>
.contacts-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2em;
  background-color: #000000;
  color: #00ff00;
}

h1 {
  font-size: 2em;
  margin-bottom: 1em;
}

.intro-text {
  font-size: 1.2em;
  margin-bottom: 1em;
  text-align: center;
  padding: 1em;
  border: 2px solid var(--glitch-color);
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 5px var(--glitch-shadow);
}

.button-group {
  display: flex;
  gap: 1em;
  margin-bottom: 1em;
}

.button-group button {
  padding: 0.7em 1.5em;
  font-size: 1em;
  border: none;
  border-radius: var(--border-radius);
  background-color: #003300;
  color: #00ff00;
  cursor: pointer;
  transition: background-color 0.3s, color 0.3s, box-shadow 0.3s;
}

.button-group button.active,
.button-group button:hover {
  background-color: #00ff00;
  color: #000000;
  box-shadow: 0 0 10px var(--glitch-shadow);
}

.section-content {
  width: 100%;
  max-width: 800px;
}

.section-content > * {
  margin-top: 1em;
  border-radius: var(--border-radius);
  padding: 1em;
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 5px var(--glitch-shadow);
}

.info-text {
  font-size: 1em;
  margin-top: 2em;
  padding: 1em;
  border: 2px solid var(--glitch-color);
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 5px var(--glitch-shadow);
  text-align: center;
}
</style>
