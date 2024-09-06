<template>
  <div class="view-contacts">
    <h2>My Contacts</h2>

    <p class="info-text">
      Here you can view a list of all your existing contacts. Browse through the
      list to see detailed information about each contact. Use the search
      feature to quickly find specific contacts.
    </p>

    <input
      v-model="searchQuery"
      type="text"
      placeholder="Search contacts..."
      class="search-input"
    />

    <ul class="contact-list">
      <li
        v-for="contact in filteredContacts"
        :key="contact.id"
        class="contact-item"
      >
        <div class="contact-info">
          <h3>{{ contact.nick }}</h3>
          <p><strong>ID:</strong> {{ contact.id }}</p>
          <p><strong>Age:</strong> {{ contact.age }}</p>
          <p><strong>Location:</strong> {{ contact.location }}</p>
          <p><strong>Occupation:</strong> {{ contact.occupation }}</p>
          <p><strong>Extra Info:</strong> {{ contact.extra_info }}</p>
          <p><strong>Contact Type:</strong> {{ contact.contact_type }}</p>
        </div>
        <button class="edit-button" @click="editContact(contact.id)">
          Edit
        </button>
      </li>
    </ul>

    <p v-if="contacts.length === 0" class="no-contacts">
      No contacts found. Please add some contacts to view them here.
    </p>
  </div>
</template>

<script>
import { ref, computed, onMounted } from "vue";

export default {
  name: "ViewContacts",
  setup() {
    const contacts = ref([]);
    const searchQuery = ref("");
    const filteredContacts = computed(() => {
      return contacts.value.filter(
        (contact) =>
          contact.nick
            .toLowerCase()
            .includes(searchQuery.value.toLowerCase()) ||
          contact.id.toString().includes(searchQuery.value)
      );
    });

    const fetchContacts = async () => {
      try {
        const response = await fetch("/api/contacts"); // Replace with your API endpoint
        if (response.ok) {
          contacts.value = await response.json();
        } else {
          console.error("Failed to fetch contacts");
        }
      } catch (error) {
        console.error("Error fetching contacts:", error);
      }
    };

    const editContact = (id) => {
      window.location.href = `/update-contact?id=${id}`;
    };

    onMounted(() => {
      fetchContacts();
    });

    return {
      contacts,
      searchQuery,
      filteredContacts,
      editContact,
    };
  },
};
</script>

<style scoped>
.view-contacts {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2em;
  background-color: #000000;
  color: #00ff00;
}

h2 {
  font-size: 2em;
  margin-bottom: 1em;
}

.info-text {
  font-size: 1.1em;
  margin-bottom: 1em;
  text-align: center;
  padding: 1em;
  border: 2px solid var(--glitch-color);
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 5px var(--glitch-shadow);
}

.search-input {
  width: 100%;
  max-width: 400px;
  padding: 0.7em;
  margin-bottom: 1em;
  border: 2px solid var(--glitch-color);
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.8);
  color: #00ff00;
}

.contact-list {
  list-style: none;
  padding: 0;
  margin: 0;
  width: 100%;
  max-width: 800px;
}

.contact-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1em;
  margin-bottom: 1em;
  border: 2px solid var(--glitch-color);
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 5px var(--glitch-shadow);
}

.contact-info {
  flex: 1;
}

.edit-button {
  padding: 0.5em 1em;
  border: none;
  border-radius: var(--border-radius);
  background-color: #003300;
  color: #00ff00;
  cursor: pointer;
  transition: background-color 0.3s, color 0.3s, box-shadow 0.3s;
}

.edit-button:hover {
  background-color: #00ff00;
  color: #000000;
  box-shadow: 0 0 10px var(--glitch-shadow);
}

.no-contacts {
  font-size: 1em;
  margin-top: 2em;
  text-align: center;
  padding: 1em;
  border: 2px solid var(--glitch-color);
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 5px var(--glitch-shadow);
}
</style>
