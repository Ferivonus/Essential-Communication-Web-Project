<script>
import { ref, watch, onMounted } from "vue";

export default {
  props: {
    selectedContact: {
      type: String,
      required: true,
    },
  },
  emits: ["contactSelected"],
  setup(props, { emit }) {
    const contacts = ref([]);
    const messages = ref([]);
    const newMessageContent = ref("");
    const sender = ref("");
    const closeOnePoint = ref("");
    const selectedContactLocal = ref(props.selectedContact || "");

    const fetchContacts = async () => {
      try {
        const response = await fetch("/api/get_contacts_other_client");
        if (response.ok) {
          const contactsResponse = await response.json();
          if (Array.isArray(contactsResponse)) {
            contacts.value = contactsResponse;
          } else {
            console.error("Unexpected response format for contacts");
          }
        } else {
          console.error("Error fetching Contacts:", response.statusText);
        }
      } catch (error) {
        console.error("Error fetching Contacts:", error);
      }
    };

    const fetchMessageBySelectedContact = async () => {
      if (!selectedContactLocal.value) return;

      try {
        const response = await fetch("/api/get_messages_other_client", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            connected: selectedContactLocal.value,
          }),
        });

        if (response.ok) {
          const getMessageResponse = await response.json();
          if (Array.isArray(getMessageResponse)) {
            messages.value = getMessageResponse;
          } else if (
            getMessageResponse &&
            typeof getMessageResponse === "object"
          ) {
            messages.value = [getMessageResponse];
          } else {
            console.error("Unexpected response format for messages");
          }
        } else {
          console.error("Error fetching messages:", response.statusText);
        }
      } catch (error) {
        console.error("Error fetching messages:", error);
      }
    };

    const sendMessage = async () => {
      if (
        !newMessageContent.value ||
        !sender.value ||
        !selectedContactLocal.value
      ) {
        alert("All fields must be filled out");
        return;
      }

      const newMessage = {
        sender: sender.value,
        receiver: selectedContactLocal.value,
        content: newMessageContent.value,
        close_one_point: closeOnePoint.value || null,
        connected: selectedContactLocal.value,
      };

      try {
        const response = await fetch("/api/send_message_other_client", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ message: newMessage }),
        });

        if (response.ok) {
          newMessageContent.value = "";
          sender.value = "";
          closeOnePoint.value = "";

          await fetchMessageBySelectedContact();
        } else {
          console.error("Error sending message:", response.statusText);
          alert(
            "Failed to send message. Please check the console for details."
          );
        }
      } catch (error) {
        console.error("Error sending message:", error);
        alert("Failed to send message. Please check the console for details.");
      }
    };

    const selectContact = (contact) => {
      selectedContactLocal.value = contact;
      fetchMessageBySelectedContact();
      emit("contactSelected", { contact });
    };

    watch(
      () => props.selectedContact,
      (newVal) => {
        if (newVal) {
          selectedContactLocal.value = newVal;
          fetchMessageBySelectedContact();
        }
      },
      { immediate: true }
    );

    onMounted(() => {
      fetchContacts();
    });

    return {
      contacts,
      messages,
      newMessageContent,
      sender,
      closeOnePoint,
      selectedContactLocal,
      sendMessage,
      selectContact,
    };
  },
};
</script>

<template>
  <div class="app-container">
    <!-- Contacts List -->
    <div class="contacts-container">
      <h1>Contacts</h1>
      <ul>
        <li v-for="contact in contacts" :key="contact.id">
          <button
            type="button"
            :class="{ selected: contact.nick === selectedContactLocal }"
            @click="selectContact(contact.nick)"
          >
            {{ contact.nick }}
          </button>
        </li>
      </ul>
    </div>

    <!-- Message Area -->
    <div v-if="selectedContactLocal" class="message-area">
      <h2>Messages with {{ selectedContactLocal }}</h2>
      <div class="messages">
        <div v-for="message in messages" :key="message.id" class="message">
          <p><strong>Sender:</strong> {{ message.sender }}</p>
          <p><strong>Content:</strong> {{ message.content }}</p>
          <p><strong>Timestamp:</strong> {{ message.timestamp }}</p>
        </div>
      </div>
      <form @submit.prevent="sendMessage">
        <input
          v-model="sender"
          type="text"
          placeholder="Sender Name"
          required
        />
        <input
          v-model="closeOnePoint"
          type="text"
          placeholder="Close One Point"
        />
        <textarea
          v-model="newMessageContent"
          placeholder="Enter your message"
          required
        ></textarea>
        <button type="submit">Send Message</button>
      </form>
    </div>
    <div v-else>
      <p>Select a contact to view messages.</p>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
}

.contacts-container {
  width: 30%;
  padding: 1em;
  border-radius: var(--border-radius);
  box-shadow: 0 0 5px var(--glitch-shadow);
  background-color: rgba(112, 13, 13, 0.219);
  overflow-y: auto;
}

.contacts-container ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.contacts-container li {
  margin: 0.5em 0;
}

.contacts-container button {
  width: 100%;
  margin: 0.5em 0;
  padding: 1em;
  border: none;
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.9);
  color: #00ff00;
  cursor: pointer;
  text-align: center;
  font-size: 1em;
  transition: background-color 0.3s;
}

.contacts-container button.selected {
  background-color: rgba(0, 0, 0, 0.7);
  color: #ff00ff;
}

.message-area {
  width: 70%;
  padding: 1em;
  background-color: rgba(0, 0, 0, 0.8);
  border-radius: var(--border-radius);
  box-shadow: 0 0 5px var(--glitch-shadow);
  height: calc(100vh - 6em); /* Adjusted for nav and footer */
  overflow-y: auto;
}

.messages p {
  margin: 0.5em 0;
  padding: 0.5em;
  background-color: rgba(112, 13, 13, 0.219);
}

textarea,
input {
  width: 100%;
  padding: 0.5em;
  border-radius: var(--border-radius);
  border: 1px solid #ccc;
  margin-bottom: 1em;
}

button {
  padding: 0.5em;
  border: none;
  border-radius: var(--border-radius);
  background-color: rgba(0, 0, 0, 0.9);
  color: #00ff00;
  cursor: pointer;
  font-size: 1em;
  transition: background-color 0.3s;
}

button:hover {
  background-color: rgba(0, 0, 0, 0.7);
}
</style>
