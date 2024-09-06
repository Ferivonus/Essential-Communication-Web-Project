<script>
import { ref, onMounted } from "vue";
export default {
  name: "WailingWallView",
  setup() {
    const wailingWallMessage = ref("");

    async function fetchWailingWallMessage() {
      try {
        // API çağrınızı yapın, örneğin axios veya fetch kullanarak
        const response = await fetch(
          "https://api.example.com/wailing-wall-message"
        );
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        wailingWallMessage.value = await response.json();
      } catch (error) {
        console.error("Error fetching wailing wall message:", error);
        wailingWallMessage.value = "Error fetching message";
      }
    }

    onMounted(() => {
      fetchWailingWallMessage();
    });

    return {
      wailingWallMessage,
    };
  },
};
</script>

<template>
  <div class="wailing-container">
    <h1>Welcome to Wailing wall page!</h1>
    <p>this is the Wailing page.</p>
    <p>{{ wailingWallMessage }}</p>

    <div class="wailing-row"></div>
  </div>
</template>

<style>
.wailing-container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.wailing-row {
  display: flex;
  justify-content: center;
}

@media (prefers-color-scheme: dark) {
  nav a:hover {
    color: #ff00ff;
  }
}
</style>
