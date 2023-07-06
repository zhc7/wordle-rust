<script setup>
import {ref} from "vue";
import Index from "./components/Index.vue";

const index = ref(null)

const theme = ref('light')
const isHard = ref(false)

function onClick() {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
}

const onShowStats = () => {
  index.value.stats()
}
</script>

<template>
  <v-app :theme="theme">
    <v-app-bar elevation="0" class="border-b">
      <v-row>
        <v-col cols="1" md="4"/>
        <v-col cols="3" md="4">
          <v-app-bar-title
              class="text-center align-center"
              style="font-family: nyt-karnakcondensed; font-weight: 700; font-size: xx-large; letter-spacing: .01em;"
          >Wordle
          </v-app-bar-title>
        </v-col>
        <v-col class="text-end">
          <v-row align="center">
            <v-switch
                v-model="isHard"
                :color="isHard ? 'blue': ''"
                label="Hard"
                class="mr-5"
                style="display: flex; justify-content: end"
                hide-details
            >
              <template v-slot:label>
                <span
                    style="font-weight: bold; font-family: nyt-karnakcondensed"
                    :style="{color: theme === 'light' ? 'black' : 'white'}"
                >Hard</span>
              </template>
            </v-switch>
            <v-icon
                @click="onShowStats"
                icon="mdi-chart-bar"
                size="x-large"
                class="mr-5"
            />
            <v-icon
                @click="onClick"
                :icon="theme === 'light' ? 'mdi-weather-sunny' : 'mdi-weather-night'"
                size="x-large"
                class="mr-5"
            />
          </v-row>
        </v-col>
      </v-row>
    </v-app-bar>

    <v-main>
      <Index :hard="isHard" ref="index"/>
    </v-main>
  </v-app>
</template>

<style>
html {
  overflow: auto !important; /* the scroll bar */
}

.v-input__slot .v-label {
  color: black !important
}
</style>
