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
            disabled: false
        }
    },
    computed: {
        hasContent: function() {
            return this.userInput.text.length > 0;
        },

        isEmpty: function() {
            return !this.hasContent;
        }

    },
    methods: {
        handleInput: function (event) {
            if (event.key === "Enter") {
                event.preventDefault();

                if (app.hasContent) {
                    //app.userInput.disabled = true;
                    document.getElementById("user-input").disabled = true;
                    document.getElementById("next-challenge").focus();
                    document.getElementById("buttonbar").classList.add("answer-incorrect");
                }

                return false;
            }
        },
        nextChallenge: function(event) {
            app.userInput.text = "";
            document.getElementById("buttonbar").classList.remove("answer-correct");
            document.getElementById("buttonbar").classList.remove("answer-incorrect");
            document.getElementById("user-input").disabled = false;
            document.getElementById("user-input").focus();
        }
    }
});

