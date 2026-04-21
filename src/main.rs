use macroquad::{prelude::*, rand::gen_range};



struct Ball{
    x:f32,
    y:f32,
    vel_x:f32,
    vel_y:f32,
    acc_x:f32,
    acc_y:f32,
    r:f32,
    color:Color
}

impl Ball{
    fn new(x: f32, y:f32, vel_x:f32, vel_y:f32, acc_x:f32, acc_y:f32, r:f32, color: Color) -> Self{
       Ball{x,y,vel_x, vel_y, acc_x, acc_y, r, color} 
    }
    fn draw(&self, color: Color){
        draw_circle(self.x, self.y, self.r, color);
    } 
    fn update(&mut self, delta_t: f32){
        self.x += delta_t * self.vel_x;
        self.y += delta_t * self.vel_y;
        self.vel_x += delta_t * self.acc_x;
        self.vel_y += delta_t * self.acc_y;
    }

    fn wall_contact(&mut self, width: f32, height:f32){
        if (self.x - self.r < 0.0 && self.vel_x < 0.0) || (self.x + self.r > width && self.vel_x > 0.0){
            self.vel_x = -self.vel_x;
            self.color = RED;
        }
        if (self.y - self.r < 0.0 && self.vel_y < 0.0) || (self.y + self.r > height && self.vel_y > 0.0){
            self.vel_y = -self.vel_y;
            self.color = BLUE;
        }   

    }
    


}

fn is_contact(a: &Ball, b: &Ball) -> bool{
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    if (dx*dx) + (dy*dy) <= (a.r + b.r) * (a.r + b.r){
        true
    }else{
        false
    }
}

fn detect_collision(balls: &mut Vec<Ball>){
    for i in 0..balls.len() {
        for j in (i + 1)..balls.len() {
            let (left, right) = balls.split_at_mut(j);
            let ball_a = &mut left[i];
            let ball_b = &mut right[0];
            
            if is_contact(ball_a, ball_b){
                ball_a.color = GREEN;
                ball_b.color = GREEN;
            }
        }
    }
}

#[macroquad::main("MyGame")]
async fn main() {

    let radius = 50;

    let mut balls: Vec<Ball> = Vec::new();

    let ball_count = 2;
    for _ in 0..ball_count{
        let x = gen_range(radius, (screen_width() as i32) - radius) as f32;
        let y = gen_range(radius, (screen_height() as i32) - radius) as f32;
        let vel_x = (gen_range(-5,5) * 100) as f32;
        let vel_y = (gen_range(-5,5) * 100) as f32;
        let acc_x = 0.0;
        let acc_y = 0.0;
        let color = YELLOW;
        balls.push(Ball::new(x, y, vel_x, vel_y, acc_x, acc_y, radius as f32, color));
    }
    

    loop {

        for ball in balls.iter_mut(){
            ball.color = YELLOW;
        }
        

        clear_background(BLACK);
        let delta_t = get_frame_time();

        let screen_width = screen_width();
        let screen_height = screen_height();

        
        
        for ball in balls.iter_mut(){
            ball.update(delta_t);
            ball.wall_contact(screen_width, screen_height);
        }
        detect_collision(&mut balls);
        
        for ball in balls.iter_mut(){
            ball.draw(ball.color);
        }

        next_frame().await
    }
}