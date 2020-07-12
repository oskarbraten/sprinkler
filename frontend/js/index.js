const URL = './configuration';

function msToTime(ms) {
    const h = (Math.floor(ms / 1000 / 60 / 60) + '').padStart(2, '0');
    const m = (Math.floor(ms / 1000 / 60 % 60) + '').padStart(2, '0');
    const s = (Math.floor(ms / 1000 % 60) + '').padStart(2, '0');
    return `${h}:${m}:${s}`;
}

function timeToMs(time) {
    const [h, m, s] = time.split(':').map(parseFloat);
    return (h * 60 * 60 * 1000) + (m * 60 * 1000) + (s * 1000);
}

Vue.component('duration', {
    template: '<span>{{duration}}</span>',
    props: {
        from: {
            type: String,
            required: true
        },
        to: {
            type: String,
            required: true
        }
    },
    computed: {
        duration: function () {
            const fromAsMs = timeToMs(this.from);
            const toAsMs = timeToMs(this.to);

            return msToTime(toAsMs - fromAsMs);
        }
    }
})

const app = new Vue({
    el: '#app',
    data: {
        connected: false,
        changed: false,
        configuration: {},
        async getConfiguration() {
            let response = await fetch(URL);
            let configuration = await response.json();

            // Format time.
            configuration.schedule.events = configuration.schedule.events.map(({ from, to }) => ({
                from: msToTime(from),
                to: msToTime(to)
            }));

            return configuration;
        },
        setConfiguration(configuration) {
            // Deep-clone to avoid mutating state.
            configuration = JSON.parse(JSON.stringify(configuration));

            console.log(configuration.overwrite);

            configuration.schedule.events = configuration.schedule.events.map(({ from, to }) => ({
                from: timeToMs(from),
                to: timeToMs(to)
            }));

            return fetch(URL, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(configuration),
            });
        }
    },
    created() {
        this.getConfiguration()
            .then(configuration => {
                this.connected = true;
                this.configuration = configuration;
            })
            .catch(error => {
                console.log(error);
            });
    },
    methods: {
        update() {
            this.setConfiguration(this.configuration)
                .then(_ => {
                    console.log("Configuration updated..");
                    this.changed = false;
                })
                .catch(error => console.log(error));
        },
        add() {
            this.configuration.schedule.events.push({ from: '10:00:00', to: '10:00:30' });
            this.changed = true;
        },
        remove(index) {
            this.configuration.schedule.events.splice(index, 1);
            this.changed = true;
        },
        toggle() {
            this.configuration.overwrite = !this.configuration.overwrite;

            this.update();
        },
        notifyChange() {
            this.changed = true;
        }
    }
});