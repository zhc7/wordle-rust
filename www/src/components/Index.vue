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
          <span style="font-family: nyt-karnakcondensed; font-weight: 700; font-size: x-large;">
            STATISTICS
          </span>
          </v-row>
          <v-row>
            <v-col>
              <div class="stat-num">
                {{ wins }}
              </div>
              <div class="stat-label">
                Wins
              </div>
            </v-col>
            <v-col>
              <div class="stat-num">
                {{ total - wins }}
              </div>
              <div class="stat-label">
                Losses
              </div>
            </v-col>
            <v-col>
              <div class="stat-num">
                {{ Math.round(total === 0 ? 0 : 1000.0 * wins / total) / 10 }}
              </div>
              <div class="stat-label">
                Win Rate %
              </div>
            </v-col>
            <v-col>
              <div class="stat-num">
                {{ Math.round(wins === 0 ? 0 : 10.0 * trials / wins) / 10 }}
              </div>
              <div class="stat-label">
                Average Trials
              </div>
            </v-col>
          </v-row>
          <v-row>
          <span style="font-family: nyt-karnakcondensed; font-weight: 700; font-size: x-large;">
            TOP WORDS
          </span>
          </v-row>
          <v-row>
            <v-col>
              <div v-for="word in top_words" style="text-align: center; font-family: nyt-karnakcondensed">
                {{ word[0] }}
              </div>
            </v-col>
            <v-col>
              <div v-for="word in top_words"
                   style="text-align: center; font-family: nyt-karnakcondensed; font-weight: bold">
                {{ word[1] }}
              </div>
            </v-col>
          </v-row>
          <v-row justify="center" class="mb-2">
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
            <v-sheet class="letter-box"
                     :class="[words_status[i][j],
                     {
                       AnimateShake: animation[i][j] === 1,
                       AnimateBounce: animation[i][j] === 2,
                       AnimatePopIn: animation[i][j] === 3,
                       AnimateFlipIn: animation[i][j] === 4,
                     }
                     ]"
                     @animationend="animationEndHandler(i, j)"
                     :style="{
                       borderColor: letter === '' ? '' : '#878a8c',
                     }"
            >
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
            <v-btn variant="tonal" size="small" height="58px" class="keyboard-key"
                   @click="press_key(key)" :class="keyboard_status[ind(key)]">
              {{ key }}
            </v-btn>
          </div>
        </v-row>
        <v-row class="justify-center d-flex">
          <div class="d-flex mr-1">
            <v-btn variant="tonal" size="large" height="58px" class="keyboard-key"
                   style="width: 12vw; max-width: 75px; font-size: 70%"
                   @click="enter">
              Enter
            </v-btn>
          </div>
          <div v-for="key in keyboard[2]" class="d-flex mr-1">
            <v-btn variant="tonal" size="small" height="58px" class="keyboard-key"
                   @click="press_key(key)" :class="keyboard_status[ind(key)]">
              {{ key }}
            </v-btn>
          </div>
          <div class="d-flex mr-1">
            <v-btn variant="tonal" size="large" height="58px" class="keyboard-key" style="width: 12vw; max-width: 75px"
                   @click="backspace">
              <v-icon icon="mdi-backspace-outline"></v-icon>
            </v-btn>
          </div>
        </v-row>
      </v-col>
    </v-row>
    {{ answer }}
  </v-container>
</template>

<script>
import * as wordle from "wordle"

export default {
  name: "Index",

  expose: ['stats'],

  props: {
    hard: Boolean,
    lightning: Boolean,
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
      animation: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
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
      animating: false,
      answer: "",
      end_game: false,
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
      this.end_game = false
      this.answer = this.runner.answer();
    },

    message(str) {
      for (let i = 0; i < 5; i++) {
        this.triggerAnimation(this.row_ptr, i, 1)
      }
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
      if (this.end_game && this.dialog) {
        this.replay()
        return;
      }
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
          if (!this.lightning) this.animating = true;
          this.row_ptr++
          this.col_ptr = 0
          for (let i = 0; i < 5; i++) {
            console.log(guess.charCodeAt(i) - 'A'.charCodeAt(0))
            let ord = ["G", "Y", "R", "X"]
            if (!this.lightning) {
              setTimeout(() => {
                this.triggerAnimation(this.row_ptr - 1, i, 4)
                setTimeout(() => {
                  if (ord.indexOf(this.keyboard_status[this.ind(guess[i])]) > ord.indexOf(result[i])) {
                    this.keyboard_status[this.ind(guess[i])] = result[i]
                  }
                  this.words_status[this.row_ptr - 1][i] = result[i]
                  if (i === 4) {
                    this.animating = false;
                  }
                }, 250)
              }, 450 * i)
            } else {
              if (ord.indexOf(this.keyboard_status[this.ind(guess[i])]) > ord.indexOf(result[i])) {
                this.keyboard_status[this.ind(guess[i])] = result[i]
              }
              this.words_status[this.row_ptr - 1][i] = result[i]
            }
          }
          let status = this.runner.status()
          console.log(status)
          if (status === 1 || status === 2) {
            this.end_game = true
            this.runner.end()
            let delay = this.lightning ? 0 : 450 * 4 + 500
            if (status === 1) {
              if (!this.lightning) {
                for (let i = 0; i < 5; i++) {
                  setTimeout(() => {
                    this.triggerAnimation(this.row_ptr - 1, i, 2)
                  }, 150 * i + delay)
                }
                delay += 4 * 150 + 1000
              } else {
                delay = 0
              }
            } else if (status === 2) {
              this.message(this.answer)
              delay += 1000
            }
            setTimeout(() => {
              this.stats()
            }, delay)
          }
        } else {
          this.message(result)
        }
      }
    },

    press_key(key) {
      if (this.col_ptr < 5 && this.row_ptr < 6 && !this.animating && !this.end_game) {
        this.words[this.row_ptr][this.col_ptr] = key.toUpperCase()
        this.triggerAnimation(this.row_ptr, this.col_ptr, 3)
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
    },

    triggerAnimation(i, j, a) {
      if (i >= 6 || j >= 5) return;
      this.animation[i][j] = 0
      this.$nextTick(() => {
        this.animation[i][j] = a
      })
    },

    animationEndHandler(i, j) {
      this.animation[i][j] = 0
    }
  },

  mounted() {
    this.runner = wordle.Runner.new();
    this.runner.start();
    this.answer = this.runner.answer();
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

@keyframes Bounce {
  0%,
  20% {
    transform: translateY(0);
  }
  40% {
    transform: translateY(-30px);
  }
  50% {
    transform: translateY(5px);
  }
  60% {
    transform: translateY(-15px);
  }
  80% {
    transform: translateY(2px);
  }
  100% {
    transform: translateY(0);
  }
}

@keyframes PopIn {
  from {
    transform: scale(0.8);
    opacity: 0;
  }

  40% {
    transform: scale(1.1);
    opacity: 1;
  }
}

@keyframes FlipIn {
  0%, 100% {
    transform: rotateX(0);
  }
  50% {
    transform: rotateX(-90deg);
  }
}

.AnimateShake {
  animation: Shake 0.82s;
}

.AnimateBounce {
  animation: Bounce 1s;
}

.AnimatePopIn {
  animation-name: PopIn;
  animation-duration: 0.13s;
  animation-timing-function: ease-in;
}

.AnimateFlipIn {
  animation-name: FlipIn;
  animation-duration: 0.5s;
  animation-timing-function: ease-in;
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

.keyboard-key {
  font-size: 1.25rem;
  font-weight: bold;
  letter-spacing: unset !important;
  min-width: unset !important;
  width: 8vw;
  max-width: 50px;
  max-height: 12vw !important;
}

@media (max-width: 400px) {
  .keyboard-key {
    font-size: 1rem;
  }
}

.stat-num {
  font-family: nyt-karnakcondensed;
  font-size: 2rem;
  margin-right: 0.5rem;
}

.stat-label {
  font-family: nyt-franklin;
  font-size: 1rem;
  font-weight: normal;
  margin-right: 1rem;
  color: #787c7e;
}
</style>