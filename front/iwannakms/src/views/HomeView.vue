<script>
export default {
  name: "HomeView",
  data() {
    return {
      currentQuote: this.getRandomQuote(),
    };
  },
  mounted() {
    setInterval(() => {
      this.currentQuote = this.getRandomQuote();
    }, 15000); // Change quote every 15 seconds
  },
  methods: {
    getRandomQuote() {
      const quotes = [
        "The world is made up of walls.",
        "I am just a regular girl, nothing more.",
        "You cannot truly be alone.",
        "I need to understand everything.",
        "I am not me. I am not anyone. I am the world.",
      ];
      return quotes[Math.floor(Math.random() * quotes.length)];
    },
  },
};
</script>

<template>
  <div class="page-wrapper">
    <div class="container">
      <h1>Welcome to the Communication OS</h1>
      <h1>Welcome ferivonus</h1>

      <div class="intro">
        <p>
          Welcome to your new communication platform, inspired by *Serial
          Experiments Lain*. Dive into the depths of digital interaction with
          our retro-styled OS.
        </p>
      </div>

      <div class="quote-section">
        <p class="quote">{{ currentQuote }}</p>
      </div>

      <div class="navigation-links">
        <h2>Explore More</h2>
        <ul>
          <li>
            <router-link href="/">
              Home
              <p class="link-description">
                Return to the main page of the Communication OS.
              </p>
            </router-link>
          </li>
          <li>
            <router-link to="/about">
              About
              <p class="link-description">
                Learn more about the concept and development of this
                application.
              </p>
            </router-link>
          </li>
          <li>
            <router-link to="/form">
              Form
              <p class="link-description">
                Access the form page to input and manage various forms.
              </p>
            </router-link>
          </li>
          <li>
            <router-link to="/messages">
              Messages
              <p class="link-description">
                View and manage messages in the communication system.
              </p>
            </router-link>
          </li>
          <li>
            <router-link to="/wailing-wall">
              Wailing Wall
              <p class="link-description">
                Contribute and read messages on the virtual wailing wall.
              </p>
            </router-link>
          </li>
        </ul>
      </div>
    </div>

    <footer class="footer">
      <p>Embrace the Chaos to Be Done and Gone | Created by Ferivonus</p>
    </footer>
  </div>
</template>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style>
@keyframes glitch {
  0% {
    transform: translate(0, 0);
    opacity: 1;
  }
  20% {
    transform: translate(-5px, -5px);
    opacity: 0.7;
  }
  40% {
    transform: translate(5px, 5px);
    opacity: 0.7;
  }
  60% {
    transform: translate(-5px, 5px);
    opacity: 0.7;
  }
  80% {
    transform: translate(5px, -5px);
    opacity: 0.7;
  }
  100% {
    transform: translate(0, 0);
    opacity: 1;
  }
}
</style>
