<script>
import MyServerMessages from "../components/message_components/MyServerMessages.vue";
import OtherServerMessageServers from "../components/message_components/OtherServerMessageServers.vue";

export default {
  name: "MessageView",
  components: {
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
      </div>

      <component
        :is="currentComponent"
        :selected-contact="selectedContact"
        @contact-selected="handleContactSelection"
      />
    </div>
  </div>
</template>
