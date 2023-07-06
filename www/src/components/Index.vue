<template>
  <v-container>
    <v-snackbar
        v-model="snackbar"
        :timeout="1000"
        location="top"
    >
      {{ text }}
    </v-snackbar>
    <v-dialog v-model="dialog" width="auto" height="auto">
      <v-card>
        <v-card-text>
          <v-row class="mt-0">
            <v-spacer></v-spacer>
            <v-icon @click="dialog = false">mdi-close</v-icon>
          </v-row>
          <v-row>
          <span style="font-weight: 700; font-size: large;">
            STATISTICS
          </span>
          </v-row>
          <v-row>
            <v-col>
              <div>
                Wins
              </div>
              <div>
                {{ wins }}
              </div>
            </v-col>
            <v-col>
              <div>
                Losses
              </div>
              <div>
                {{ total - wins }}
              </div>
            </v-col>
            <v-col>
              <div>
                Win Rate
              </div>
              <div>
                {{ total === 0 ? 0 : 100.0 * wins / total }} %
              </div>
            </v-col>
          </v-row>
          <v-row>
          <span style="font-weight: 700; font-size: large;">
            TOP WORDS
          </span>
          </v-row>
          <v-row>
            <v-col>
              <div v-for="word in top_words">
                {{ word[0] }}
              </div>
            </v-col>
            <v-col>
              <div v-for="word in top_words">
                {{ word[1] }}
              </div>
            </v-col>
          </v-row>
          <v-row justify="center" class="mb-0">
            <v-btn
                style="font-family: nyt-karnakcondensed; font-weight: 700; font-size: x-large;"
                @click="replay"
                variant="outlined"
            >
              Replay!
            </v-btn>
          </v-row>
        </v-card-text>
      </v-card>
    </v-dialog>
    <v-row class="mb-0 mt-0" justify="center">
      <v-col style="flex: none; width: 300px;">
        <v-row v-for="(word, i) in words">
          <v-col v-for="(letter, j) in word" class="pa-0 mr-1">
            <v-sheet class="letter-box" :class="words_status[i][j]">
              {{ letter }}
            </v-sheet>
          </v-col>
        </v-row>
      </v-col>
    </v-row>
    <v-row class="fill-height">
      <v-col>
        <v-row v-for="line in keyboard.slice(0, 2)" class="mb-2 justify-center">
          <div v-for="key in line" class="d-flex mr-1">
            <v-btn variant="tonal" size="small" height="58px" style="font-size: 1.25rem" class="font-weight-bold"
                   @click="press_key(key)" :class="keyboard_status[ind(key)]">
              {{ key }}
            </v-btn>
          </div>
        </v-row>
        <v-row class="justify-center d-flex">
          <div class="d-flex mr-1">
            <v-btn variant="tonal" size="large" height="58px" style="font-size: 12px" class="font-weight-bold"
                   @click="enter">
              Enter
            </v-btn>
          </div>
          <div v-for="key in keyboard[2]" class="d-flex mr-1">
            <v-btn variant="tonal" size="small" height="58px" style="font-size: 1.25rem" class="font-weight-bold"
                   @click="press_key(key)" :class="keyboard_status[ind(key)]">
              {{ key }}
            </v-btn>
          </div>
          <div class="d-flex mr-1">
            <v-btn variant="tonal" size="large" height="58px" style="font-size: 1.25rem" class="font-weight-bold"
                   @click="backspace">
              <v-icon icon="mdi-backspace-outline"></v-icon>
            </v-btn>
          </div>
        </v-row>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import * as wordle from "wordle"

export default {
  name: "Index",

  expose: ['stats'],

  props: {
    hard: Boolean
  },

  data() {
    return {
      words: [
        ['', '', '', '', ''],
        ['', '', '', '', ''],
        ['', '', '', '', ''],
        ['', '', '', '', ''],
        ['', '', '', '', ''],
        ['', '', '', '', '']
      ],
      words_status: [
        ['X', 'X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X', 'X'],
      ],
      keyboard: [
        ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
        ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
        ['Z', 'X', 'C', 'V', 'B', 'N', 'M']
      ],
      keyboard_status: Array(26).fill("X"),
      col_ptr: 0,
      row_ptr: 0,
      snackbar: false,
      text: '',
      dialog: false,
      wins: 0,
      total: 0,
      trials: 0,
      top_words: [],
    }
  },

  methods: {
    replay() {
      for (let i = 0; i < 6; i++) {
        for (let j = 0; j < 5; j++) {
          this.words[i][j] = ''
          this.words_status[i][j] = 'X'
        }
      }
      this.col_ptr = 0
      this.row_ptr = 0
      this.keyboard_status = Array(26).fill("X")
      this.runner.start()
      this.dialog = false
    },

    message(str) {
      this.text = str
      this.snackbar = true
    },

    ind(c) {
      return c.charCodeAt(0) - 'A'.charCodeAt(0)
    },

    stats() {
      let stat = this.runner.nums()
      this.wins = stat[0]
      this.total = stat[1]
      this.trials = stat[2]
      this.runner.top_words()
      let words = []
      let counts = this.runner.top_word_counts()
      console.log(counts)
      for (let i = 0; i < (counts.length < 5 ? counts.length : 5); i++) {
        words.push([this.runner.top_word(), counts[i]])
      }
      this.top_words = words
      this.dialog = true
    },

    enter() {
      if (this.col_ptr < 5) {
        this.message('Not enough letters')
      } else {
        let guess = this.words[this.row_ptr].join('')
        console.log(guess)
        if (this.hard) {
          let check = this.runner.check(guess)
          if (check === 2) {
            this.message("Invalid in hard mode")
            return;
          }
        }
        let result = this.runner.guess(guess)
        if (result.length === 5) {
          this.words_status[this.row_ptr] = result.split('')
          this.row_ptr++
          this.col_ptr = 0
          for (let i = 0; i < 5; i++) {
            console.log(guess.charCodeAt(i) - 'A'.charCodeAt(0))
            this.keyboard_status[this.ind(guess[i])] = result[i]
          }
          let status = this.runner.status()
          if (status === 1 || status === 2) {
            this.runner.end()
            this.stats()
          }
        } else {
          this.message(result)
        }
      }
    },

    press_key(key) {
      if (this.col_ptr < 5) {
        this.words[this.row_ptr][this.col_ptr] = key.toUpperCase()
        this.col_ptr++
      }
    },

    backspace() {
      if (this.col_ptr > 0) {
        this.col_ptr--
        this.words[this.row_ptr][this.col_ptr] = ''
      }
    },

    color(status) {
      if (status === 'X') {
        return ''
      }
      if (status === 'G') {
        return '#6aaa64'
      }
      if (status === 'Y') {
        return 'orange'
      }
      if (status === 'R') {
        return '#787c7e'
      }
    },

    keyboard_event(event) {
      console.log(event.key)
      if (event.key.length === 1 && event.key.toUpperCase() >= 'A' && event.key.toUpperCase() <= 'Z') {
        this.press_key(event.key)
      }
      if (event.key === 'Backspace') {
        this.backspace()
      }
      if (event.key === 'Enter') {
        this.enter()
      }
    }
  },

  mounted() {
    this.runner = wordle.Runner.new();
    this.runner.start();
  },

  created() {
    document.addEventListener('keydown', this.keyboard_event)
  },

  destroyed() {
    document.removeEventListener('keydown', this.keyboard_event)
  }
}
</script>

<style scoped>
@keyframes Shake {
  10%, 90% {
    transform: translateX(-1px);
  }

  20%, 80% {
    transform: translateX(2px);
  }

  30%, 50%, 70% {
    transform: translateX(-4px);
  }

  40%, 60% {
    transform: translateX(4px);
  }
}

.R {
  background-color: #787c7e;
  color: white;
}

.Y {
  background-color: orange;
  color: white;
}

.G {
  background-color: #6aaa64;
  color: white;
}

.letter-box.X {
  border: 2px solid #d3d6da;
}

.letter-box {
  font-family: "nyt-franklin";
  width: 100%;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  font-size: 2rem;
  font-weight: bold;
  box-sizing: border-box;
  text-transform: uppercase;
}

.letter-box::before {
  content: "";
  display: inline-block;
  padding-bottom: 100%;
}
</style>