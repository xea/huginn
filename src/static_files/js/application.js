"use strict";

Vue.component('app-header', {
    template: `
  <div class="header">Project Húginn</div>`
});

Vue.component('exercise', {
    template: `
  <div class="exercise">
  	<div class="exercise-box">
    	<slot></slot>
    </div>
  </div>`
});

Vue.component('task', {
    template: `
  <div class="task"><slot></slot></div>`
});

Vue.component('question', {
    template: `
  <div class="question"><slot></slot></div>`
});

Vue.component('user-input', {
    inject: [ 'challenge' ],
    data: function() {
        return {
            userInput: ""
        };
    },
    template: `
  <div class="user-input">
  	<textarea rows="5" ref="userInput" v-model="userInput" @keypress="handleKey" :disabled="challenge.submitted" placeholder="Type your answer"></textarea>
  </div>`,
    methods: {
        handleKey: function(event) {
            if (event.key === 'Enter') {
                event.preventDefault();
                this.challenge.submitted = true;
                this.$emit('answer:submitted', this.userInput);
            }
        },
        reset: function() {
            this.userInput = "";
            this.challenge.submitted = false;
            this.$refs.userInput.disabled = false;
            this.$refs.userInput.focus();
        }
    }
});

Vue.component('app-footer', {
    template: `
  <div ref="footer" class="footer"><slot></slot></div>`
});

Vue.component('correct-answer', {
    methods: {
        refocus: function() {
            this.$nextTick(() => this.$refs.btnNext.focus());
        }
    },
    template: `
  <div class="correct-answer">
  	<div class="button-bar">
			<div class="button-wrapper2">
				Correct answer!
			</div>
    	<div class="button-wrapper3">
    		<button ref="btnNext" class="next-question" @click="$emit('challenge:next')">Next</button>
      </div>
    </div>
  </div>`
});

Vue.component('incorrect-answer', {
    methods: {
        refocus: function() {
            this.$nextTick(() => this.$refs.btnNext.focus());
        }
    },
    template: `
  <div class="incorrect-answer">
  	<div class="button-bar">
			<div class="button-wrapper2">
				Some text
			</div>
    	<div class="button-wrapper3">
    		<button ref="btnNext" class="next-question" @click="$emit('challenge:next')">Next</button>
    	</div>
    </div>
  </div>`
});

Vue.component('in-progress', {
    inject: [ 'challenge' ],
    methods: {
        submitClicked: function() {
            this.challenge.submitted = true;
            this.$emit('answer:submitted');
        }
    },
    template: `
  <div class="in-progress">
  	<div class="button-bar">
			<div class="button-wrapper2">&nbsp;</div>
      <div class="button-wrapper3">
  			<button class="submit" @click="submitClicked">Submit</button>
    	</div>
    </div>
  </div>`
});

var app = new Vue({
    el: "#app",
    data: {
        challenge: {
            submitted: false,
            correct: false
        }
    },
    created: function() {
        // Create alias so that we can access it within the promise
        var vm = this;

        vm.loadData();
    },
    computed: {
        showSuccess: function() { return this.challenge.submitted && this.challenge.correct; },
        showFailure: function() { return this.challenge.submitted && !this.challenge.correct; },
        showSubmit: function() { return !this.challenge.submitted; }
    },
    methods: {
        onSubmit: function(data) {
            this.challenge.submitted = true;
            // decide whether answer is correct
            this.challenge.correct = (data === "apple");

            if (this.challenge.correct) {
                this.$refs.correct.refocus();
            } else {
                this.$refs.incorrect.refocus();
            }
        },
        onNext: function() {
            this.challenge.submitted = false;
            this.challenge.correct = false;
            this.$refs.userInput.reset();
        },
        loadData: function() {
            let vm = this;

            fetch("/echo/html/")
                .then(function(response) {
                    console.log(response);
                })
        }
    },
    provide: function() {
        return {
            challenge: this.challenge
        };
    }
});

let crypto = window.crypto.subtle;

function stringToBuffer(input) {
    let buffer = new ArrayBuffer(input.length);
    let view = new Uint8Array(buffer);

    for (let i = 0; i < input.length; i++) {
        view[i] = input.charCodeAt(i);
    }

    return buffer;
}

async function calculateHash(inputStr) {
    return crypto.digest({ name: "SHA-256" }, stringToBuffer(inputStr))
        .then(function(hash) {
            return hash.slice(0, 16);
        })
        .catch(function(error) {
            console.log("Hash error: " + error);
        });
}

async function importKey(key) {
    return crypto.importKey("raw", key, { name: "AES-CBC" }, true, [ "encrypt", "decrypt" ])
        .catch(function(error) {
            console.log("Import key error: " + error);
        });
}

async function encrypt(ivStr, keyBuffer, data) {
    return crypto.encrypt({ name: "AES-CBC", iv: stringToBuffer(ivStr) }, keyBuffer, stringToBuffer(data))
        .catch(function(error) {
            console.log("Encryption error: " + error);
        });
}

async function decrypt(ivStr, keyBuffer, data) {
    return crypto.decrypt({ name: "AES-CBC", iv: stringToBuffer(ivStr) }, keyBuffer, stringToBuffer(data))
        .catch(function(error) {
            console.log("Decryption error: " + error);
        });

}

async function decryptAnswer(normalizedText, iv64, cipherText64) {
    let iv = atob(iv64);
    let cipherText = atob(cipherText64);

    let hash = await calculateHash(normalizedText);
    let key = await importKey(hash);

    return await decrypt(iv, key, cipherText);
}

