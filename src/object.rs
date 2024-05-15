


trait Object {
    fn update( &mut self, dt: f64);
    fn get_pos( &self ) -> (f64, f64);
    fn render( &self );
}

struct Bullet {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}
impl Object for Bullet {
    fn update( &mut self, dt: f64) {
        todo!()
    }

    fn get_pos( &self ) -> (f64, f64) {
        todo!()
    }

    fn render( &self ) {
        todo!()
    }
}