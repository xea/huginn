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
            this.userInput.verified = true;
            this.challenge.responseSuccessful = true;

            this.$refs.verifyInput.focus();
        },
        nextChallenge: function(event) {
            this.userInput.verified = false;
            this.userInput.text = "";

            this.challenge.responseSuccessful = false;
            this.challenge.responseFailed = false;

            this.$refs.userInput.focus();
        }

    }
});

