Vue.directive('focus', {
    inserted: function(element) {
        element.focus();
    }
});

Vue.component('question', {
    template: '<div class="question-text"><slot></slot></div>'
});

let app = new Vue({
    el: "#app",
    data: {
        userInput: {
            text: "",
            submitted: false,
            verified: false
        },
        challenge: {
            responseSuccessful: false,
            responseFailed: false
        }
    },
    computed: {
        hasContent: function() {
            return this.userInput.text.length > 0;
        },

        isEmpty: function() {
            return !this.hasContent;
        },
        answerCorrect: function() {
            return this.challenge.responseSuccessful;
        },
        answerIncorrect: function() {
            return this.challenge.responseFailed;
        }

    },
    methods: {
        handleInput: function (event) {
            if (event.key === "Enter") {
                event.preventDefault();

                this.verifyInput(event);
            }
        },
        verifyInput: function(event) {
            this.userInput.submitted = true;

            let request = {
                course_id: "test-course",
                challenge_id: "test-lesson",
                answer: this.userInput.text.trim()
            };

            fetch("/course/submit", {
                method: 'POST',
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(request)
            })
                .then((response) => {
                    this.userInput.verified = true;
                    this.challenge.responseSuccessful = true;

                    this.$refs.verifyInput.focus();
                });
        },
        nextChallenge: function(event) {
            this.userInput.submitted = false;
            this.userInput.verified = false;
            this.userInput.text = "";

            this.challenge.responseSuccessful = false;
            this.challenge.responseFailed = false;

            // Not sure if this is the best way to deal with focuses
            this.$refs.userInput.disabled = false;
            this.$refs.userInput.focus();
        }

    }
});


let crypto = window.crypto.subtle;

function stringToBuffer(input) {
    let buffer = new ArrayBuffer(input.length);
    let view = new Uint8Array(buffer);

    for (var i = 0; i < input.length; i++) {
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

async function decryptAnswer(normalizedText, iv64, ciphertext64) {
    let iv = atob(iv64);
    let ciphertext = atob(ciphertext64);

    let hash = await calculateHash(normalizedText);
    let key = await importKey(hash);

    return await decrypt(iv, key, ciphertext);
}

