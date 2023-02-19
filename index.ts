import { Tangle, TangleState, UserId } from "../tangle/tangle_ts/src/index";

async function setup_demo1() {
    set_random_name();

    let canvas = document.getElementById("demo1")! as HTMLCanvasElement;
    canvas.style.opacity = "0.0";

    var context = canvas.getContext("2d")!;

    let fixed_update_interval = 1000 / 60;

    let imports = {
        env: {
            set_color: function (r: number, g: number, b: number, a: number) {
                context.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
            },
            draw_circle: function (x: number, y: number, radius: number) {
                //extra_elements.push(addSVGCircle(x, y, radius));
                context.beginPath();
                context.arc(x, y, radius, 0, 2 * Math.PI);
                context.fill();
            },
            begin_path: function () {
                context.beginPath();
            },
            move_to: function (x: number, y: number) {
                context.moveTo(x, y);
            },
            line_to: function (x: number, y: number) {
                context.lineTo(x, y);
            },
            stroke: function () {
                context.stroke();
            },
            fill: function () {
                context.fill();
            },
            translate: function (x: number, y: number) {
                context.translate(x, y);
            },
            rotate: function (radians: number) {
                context.rotate(radians);
            },
            draw_rect: function (x: number, y: number, width: number, height: number) {
                context.beginPath();
                context.rect(x, y, width, height);
                context.fill();
            },
            set_transform: function (a: number, b: number, c: number, d: number, e: number, f: number) {
                context.setTransform(a, b, c, d, e, f);
            }
        },
    };

    let wasm_binary = await fetch("rust_project.wasm").then(response => response.arrayBuffer());

    let result = await Tangle.instantiate(new Uint8Array(wasm_binary), imports, {
        fixed_update_interval,
        on_state_change_callback: (state) => {
            if (state == TangleState.Connected) {
                canvas.style.opacity = "1.0";
                if (exports.player_joined) {
                    exports.player_joined(UserId);
                }
            }
        },
    });
    let tangle = result.tangle;
    let exports = result.instance.exports;

    document.onpointerdown = async (event) => {
        let rect = canvas.getBoundingClientRect();
        if (exports.pointer_down) {
            exports.pointer_down(UserId, event.clientX - rect.left, event.clientY - rect.top);
        }
    };

    document.onpointermove = async (event) => {
        let rect = canvas.getBoundingClientRect();
        if (exports.pointer_move) {
            exports.pointer_move(UserId, event.clientX - rect.left, event.clientY - rect.top);
        }
    };

    document.onpointerup = async (event) => {
        let rect = canvas.getBoundingClientRect();

        if (exports.pointer_up) {
            exports.pointer_up(UserId, event.pointerType === "mouse", event.clientX - rect.left, event.clientY - rect.top);
        }
    };

    document.onkeydown = async (event) => {
        if (exports.key_down) {
            exports.key_down(UserId, event.keyCode);
        }

        /*
        if (event.key == "h") {
            tangle.print_history();
        }
        */
    };

    document.onkeyup = async (event) => {
        if (exports.key_up) {
            exports.key_up(UserId, event.keyCode);
        }
    };

    async function animation() {
        if (canvas.width != canvas.clientWidth || canvas.height != canvas.clientHeight) {
            canvas.width = canvas.clientWidth;
            canvas.height = canvas.clientHeight;
        }

        context.clearRect(0, 0, context.canvas.width, context.canvas.height);

        exports.draw.callAndRevert();

        window.requestAnimationFrame(animation);
    }
    animation();
}

function set_random_name() {
    if (!window.location.hash) {
        window.location.hash += ADJECTIVES[Math.floor(Math.random() * ADJECTIVES.length)];
        window.location.hash += ADJECTIVES[Math.floor(Math.random() * ADJECTIVES.length)];
        window.location.hash += ANIMAL_NAMES[Math.floor(Math.random() * ANIMAL_NAMES.length)];
    }
}

const ANIMAL_NAMES = [
    "Albatross",
    "Alligator",
    "Alpaca",
    "Antelope",
    "Donkey",
    "Badger",
    "Bat",
    "Bear",
    "Bee",
    "Bison",
    "Buffalo",
    "Butterfly",
    "Camel",
    "Capybara",
    "Cat",
    "Cheetah",
    "Chicken",
    "Chinchilla",
    "Clam",
    "Cobra",
    "Crab",
    "Crane",
    "Crow",
    "Deer",
    "Dog",
    "Dolphin",
    "Dove",
    "Dragonfly",
    "Duck",
    "Eagle",
    "Elephant",
    "Elk",
    "Emu",
    "Falcon",
    "Ferret",
    "Finch",
    "Fish",
    "Flamingo",
    "Fox",
    "Frog",
    "Gazelle",
    "Gerbil",
    "Giraffe",
    "Goat",
    "Goldfish",
    "Goose",
    "Grasshopper",
    "Hamster",
    "Heron",
    "Horse",
    "Hyena",
    "Jaguar",
    "Jellyfish",
    "Kangaroo",
    "Koala",
    "Lemur",
    "Lion",
    "Lobster",
    "Manatee",
    "Mantis",
    "Meerkat",
    "Mongoose",
    "Moose",
    "Mouse",
    "Narwhal",
    "Octopus",
    "Okapi",
    "Otter",
    "Owl",
    "Panther",
    "Parrot",
    "Pelican",
    "Penguin",
    "Pony",
    "Porcupine",
    "Rabbit",
    "Raccoon",
    "Raven",
    "Salmon",
    "Seahorse",
    "Seal",
    "Shark",
    "Snake",
    "Sparrow",
    "Stingray",
    "Stork",
    "Swan",
    "Tiger",
    "Turtle",
    "Viper",
    "Walrus",
    "Wolf",
    "Wolverine",
    "Wombat",
    "Yak",
    "Zebra",
    "Gnome",
    "Unicorn",
    "Dragon",
    "Hippo",
];

const ADJECTIVES = [
    "Beefy",
    "Big",
    "Bold",
    "Brave",
    "Bright",
    "Buff",
    "Calm",
    "Charming",
    "Chill",
    "Creative",
    "Cute",
    "Cool",
    "Crafty",
    "Cunning",
    "Daring",
    "Elegant",
    "Excellent",
    "Fab",
    "Fluffy",
    "Grand",
    "Green",
    "Happy",
    "Heavy",
    "Honest",
    "Huge",
    "Humble",
    "Iconic",
    "Immense",
    "Jolly",
    "Jumbo",
    "Kind",
    "Little",
    "Loyal",
    "Lucky",
    "Majestic",
    "Noble",
    "Nefarious",
    "Odd",
    "Ornate",
    "Plucky",
    "Plump",
    "Polite",
    "Posh",
    "Quirky",
    "Quick",
    "Round",
    "Relaxed",
    "Rotund",
    "Shy",
    "Sleek",
    "Sly",
    "Spry",
    "Stellar",
    "Super",
    "Tactical",
    "Tidy",
    "Trendy",
    "Unique",
    "Vivid",
    "Wild",
    "Yappy",
    "Young",
    "Zany",
    "Zesty",
];

setup_demo1();
