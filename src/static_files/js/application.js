Vue.directive('focus', {
    inserted: function(element) {
        element.focus();
    }
});

Vue.component('challenge', {
    template: '<div class="challenge"><div class="challenge-content"><slot></slot></div></div>'
});

Vue.component('task', {
    template: '<div class="task"><slot></slot></div>'
});

Vue.component('question', {
    template: '<div class="question-text"><slot></slot></div>'
});

Vue.component('user-input', {
    template: '<div class="form-group user-input-holder"><textarea rows="3" v-focus class="form-control user-input" placeholder="Type your answer here"></textarea></div>'
});

let app = new Vue({
    el: "#app",
    data: {
    }
});

