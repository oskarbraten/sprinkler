const URL = './configuration';

const app = new Vue({
    el: '#app',
    data: {
        connected: false,
        configuration: {},
        updateConfiguration(configuration) {
            fetch(URL, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(configuration),
            })
        }
    },
    created() {
        fetch(URL)
            .then(response => response.json())
            .then(configuration => {
                this.connected = true;
                this.configuration = configuration;
            })
            .catch(error => {
                // handle not connected?
                console.log(error);
            });
    },
    methods: {
        toggle() {
            this.configuration.enabled = !this.configuration.enabled;
            this.updateConfiguration(this.configuration)
            .then(_ => console.log("Configuration updated.."))
            .catch(error => console.log(error));
        }
    }
});