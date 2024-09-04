<script>
import AddContactMyClient from "../components/message_components/AddContactMyClient.vue";
import AddContactOtherClient from "../components/message_components/AddContactOtherClient.vue";
import MyServerMessages from "../components/message_components/MyServerMessages.vue";
import OtherServerMessageServers from "../components/message_components/OtherServerMessageServers.vue";

export default {
  components: {
    AddContactMyClient,
    AddContactOtherClient,
    MyServerMessages,
    OtherServerMessageServers,
  },
  data() {
    return {
      selectedContact: "",
      selectedView: null,
    };
  },
  computed: {
    currentComponent() {
      switch (this.selectedView) {
        case "myServerMessages":
          return MyServerMessages;
        case "OtherServerMessageServers":
          return OtherServerMessageServers;
        case "addContactMyClient":
          return AddContactMyClient;
        case "addContactOtherClient":
          return AddContactOtherClient;
        default:
          return null;
      }
    },
  },
  methods: {
    showMyServerMessages() {
      this.selectedView = "myServerMessages";
    },
    showOtherServerMessageServers() {
      this.selectedView = "OtherServerMessageServers";
    },
    showAddContactMyClient() {
      this.selectedView = "addContactMyClient";
    },
    showAddContactOtherClient() {
      this.selectedView = "addContactOtherClient";
    },
    handleContactSelection(event) {
      this.selectedContact = event.contact;
    },
  },
};
</script>

<template>
  <div class="page-wrapper">
    <div class="container">
      <h2>Welcome to the Communication OS</h2>
      <div class="button-container">
        <button @click="showMyServerMessages">My Server Messages</button>
        <button @click="showOtherServerMessageServers">
          Other Server Messages
        </button>
        <button @click="showAddContactMyClient">Add Contact My Client</button>
        <button @click="showAddContactOtherClient">
          Add Contact Other Client
        </button>
      </div>

      <component
        :is="currentComponent"
        :selected-contact="selectedContact"
        @contact-selected="handleContactSelection"
      />
    </div>
  </div>
</template>
