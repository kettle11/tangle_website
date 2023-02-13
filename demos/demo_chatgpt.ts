// Changes the color of all subsequent calls to draw_circle
@external("env", "set_color")
    declare function set_color(r: u8, g: u8, b: u8, a: u8): void

// Call from within the draw function to draw a ball
@external("env", "draw_circle")
    declare function draw_circle(x: f64, y: f64, radius: f64): void

// Store balls' properties
const BALL_RADIUS: f64 = 20.0;
const BALL_MASS: f64 = 1.0;
const FLOOR_Y: f64 = 400.0;
const GRAVITY: f64 = 0.5;

const DT: f64 = 0.01;
const RESTITUTION: f64 = 0.8;
const SPRING: f64 = 0.1;


// A class to represent a ball in the simulation
class Ball {
    x: f64;
    y: f64;
    vx: f64;
    vy: f64;
    ax: f64;
    ay: f64;
    last_x: f64;
    last_y: f64;

    // Constructor
    constructor(x: f64, y: f64) {
        this.x = x;
        this.y = y;
        this.vx = 0.0;
        this.vy = 0.0;
        this.ax = 0.0;
        this.ay = 0.0;
        this.last_x = x;
        this.last_y = y;
    }

    // Updates the ball's position and velocity using Verlet integration
    update(): void {
        this.vx += this.ax * DT;
        this.vy += (this.ay + GRAVITY) * DT;
        let nextX = this.x + this.vx * DT;
        let nextY = this.y + this.vy * DT;

        // Check for ball-to-ball collisions
        for (let i = 0; i < balls.length; i++) {
            let other = balls[i];
            if (other === this) continue;
            let dx = nextX - other.x;
            let dy = nextY - other.y;
            let distance = Math.sqrt(dx * dx + dy * dy);
            let minDist = BALL_RADIUS + BALL_RADIUS;
            if (distance < minDist) {
                let angle = Math.atan2(dy, dx);
                let targetX = other.x + Math.cos(angle) * minDist;
                let targetY = other.y + Math.sin(angle) * minDist;
                let ax = (targetX - this.x) * SPRING;
                let ay = (targetY - this.y) * SPRING;
                this.vx -= ax;
                this.vy -= ay;
            }
        }

        // Check for floor collision
        if (nextY > FLOOR_Y) {
            nextY = FLOOR_Y;
            this.vy = -this.vy * RESTITUTION;
        }

        this.x = nextX;
        this.y = nextY;
    }
}

// An array to store all the balls in the simulation
var balls: Ball[] = [];

// Add a new ball to the simulation
function new_ball(x: f64, y: f64): void {
    let ball = new Ball(x, y);
    balls.push(ball);
}

// Called whenever a user clicks. Each user has a unique ID.
// Spawn a ball when the player clicks.
export function pointer_down(player_id: u32, x: u32, y: u32): void {
    new_ball(x as f64, y as f64);
}

// This is called 60 times per second. Put logic here.
export function fixed_update(): void {
    for (let i = 0; i < balls.length; i++) {
        balls[i].update();
    }
}

// Called whenever a draw should occur. The host environment rolls back any changes made in this function.
export function draw(): void {
    set_color(255, 255, 255, 255);
    for (let i = 0; i < balls.length; i++) {
        draw_circle(balls[i].x, balls[i].y, BALL_RADIUS);
    }
}
