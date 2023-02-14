import { Tangle, TangleState, UserId } from "../tangle/tangle_ts/src/index";

async function setup_demo1() {
    let extra_elements: Array<Element> = [];

    let canvas = document.getElementById("demo1")! as HTMLCanvasElement;
    canvas.style.opacity = "0.0";

    var context = canvas.getContext("2d")!;

    let fixed_update_interval = 1000 / 60;

    function addEmoji(emoji: string, x: number, y: number): Element {
        let emojiElement = document.createElement("div");
        emojiElement.style.position = "absolute";
        emojiElement.style.left = x + "px";
        emojiElement.style.top = y + "px";
        emojiElement.style.display = "flex";
        emojiElement.style.justifyContent = "center";
        emojiElement.style.alignItems = "center";
        emojiElement.style.fontSize = "35px";
        emojiElement.innerText = emoji;
        emojiElement.style.transform = "translate(-50%, -50%)";
        document.body.appendChild(emojiElement);
        return emojiElement;
    }

    function addSVGArrow(x1: number, y1: number, x2: number, y2: number): Element {
        const stroke_width = 6;
        const padding = 20;
        const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("style", "position: absolute;");
        svg.setAttribute("width", `${Math.abs(x2 - x1) + padding * 2}`);
        svg.setAttribute("height", `${Math.abs(y2 - y1) + padding * 2}`);

        const minX = Math.min(x1, x2);
        const minY = Math.min(y1, y2);
        svg.setAttribute("style", `position: absolute; top: ${minY - padding}px; left: ${minX - padding}px;`);

        const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
        line.setAttribute("x1", `${Math.abs(x1 - minX) + padding}`);
        line.setAttribute("y1", `${Math.abs(y1 - minY) + padding}`);
        line.setAttribute("x2", `${Math.abs(x2 - minX) + padding}`);
        line.setAttribute("y2", `${Math.abs(y2 - minY) + padding}`);
        line.setAttribute("style", `stroke: currentColor; stroke-width: ${stroke_width}; stroke-dasharray: 20`);

        const dx = x2 - x1;
        const dy = y2 - y1;

        let distance = Math.sqrt(dx * dx + dy * dy);
        if (distance == 0.0) {
            distance = 0.1;
        }
        const normalized_dx = dx / distance;
        const normalized_dy = dy / distance;

        const size = 10.0;
        const center_x = (x2 + padding) - normalized_dx * size - minX;
        const center_y = (y2 + padding) - normalized_dy * size - minY;

        const perp_x = normalized_dy;
        const perp_y = -normalized_dx;

        const left_x = center_x - perp_x * size;
        const left_y = center_y - perp_y * size;

        const right_x = center_x + perp_x * size;
        const right_y = center_y + perp_y * size;

        const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
        path.setAttribute("d", `M ${left_x} ${left_y}, L ${x2 - minX + padding} ${y2 - minY + padding}, L ${right_x} ${right_y}`);
        path.setAttribute("style", `stroke:currentColor; stroke-width: ${stroke_width}; fill: none; stroke-linejoin:round; stroke-linecap:round;`);

        svg.appendChild(line);
        svg.appendChild(path);
        document.body.appendChild(svg);
        return svg;
    }

    function addSVGCircle(x: number, y: number, radius: number): SVGElement {
        const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("width", `${radius * 2}`);
        svg.setAttribute("height", `${radius * 2}`);
        svg.style.position = "absolute";
        svg.style.left = `${x - radius}px`;
        svg.style.top = `${y - radius}px`;
        document.body.appendChild(svg);

        const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
        circle.setAttribute("cx", `${radius}`);
        circle.setAttribute("cy", `${radius}`);
        circle.setAttribute("r", `${radius}`);
        svg.appendChild(circle);

        svg.style.filter = "drop-shadow(2px 3px 2px rgb(0 0 0 / 0.4))";
        return svg;
    };


    let imports = {
        env: {
            set_color: function (r: number, g: number, b: number, a: number) {
                context.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
            },
            draw_text: function (x: number, y: number, text_address: number, text_length: number) {
                let text = tangle.read_string(text_address, text_length);
                extra_elements.push(addEmoji(text, x, y));
            },
            draw_circle: function (x: number, y: number, radius: number) {
                //extra_elements.push(addSVGCircle(x, y, radius));
                context.beginPath();
                context.arc(x, y, radius, 0, 2 * Math.PI);
                context.fill();
            },
            draw_arrow: function (x0: number, y0: number, x1: number, y1: number) {
                extra_elements.push(addSVGArrow(x0, y0, x1, y1));
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

    let result = await Tangle.instanstiate(new Uint8Array(wasm_binary), imports, {
        fixed_update_interval,
        on_state_change_callback: (state) => {
            if (state == TangleState.Connected) {
                canvas.style.opacity = "1.0";
                if (exports.player_joined) {
                    exports.player_joined([UserId]);
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
            exports.pointer_up(UserId, event.clientX - rect.left, event.clientY - rect.top);
        }
    };

    document.onkeydown = async (event) => {
        let rect = canvas.getBoundingClientRect();
        if (exports.key_down) {
            exports.key_down(UserId, event.keyCode);
        }
        if (event.key == "h") {
            tangle.print_history();
        }
    };

    document.onkeyup = async (event) => {
        let rect = canvas.getBoundingClientRect();
        if (exports.key_up) {
            exports.key_up(UserId, event.keyCode);
        }
    };

    async function animation() {
        if (canvas.width != canvas.clientWidth || canvas.height != canvas.clientHeight) {
            canvas.width = canvas.clientWidth;
            canvas.height = canvas.clientHeight;
        }

        extra_elements.forEach((element) => {
            element.remove();
        });
        extra_elements = [];

        context.clearRect(0, 0, context.canvas.width, context.canvas.height);

        exports.draw.callAndRevert();

        window.requestAnimationFrame(animation);
    }
    animation();
}

setup_demo1();
