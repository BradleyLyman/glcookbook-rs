use ::BaseVertex;

pub struct Grid<T: BaseVertex> {
    pub vertices : Vec<T>,
    pub indices  : Vec<u16>
}

impl <T: BaseVertex> Grid<T> {
    pub fn new(depth: f32, width: f32, x_count: u16, z_count: u16) -> Grid<T> {
        let mut grid = Grid { vertices : vec![], indices  : vec![] };

        grid.build_vertices(depth, width, x_count, z_count);
        grid.build_indices(x_count, z_count);
        grid
    }

    fn build_vertices(
        &mut self, depth: f32, width: f32, x_count: u16, z_count: u16
    ) {
        for j in 0..z_count {
            let scaled_j = ((j as f32)/(z_count as f32 -1.0)) * 2.0 - 1.0;
            for i in 0..x_count {
                let scaled_i = ((i as f32)/(x_count as f32 - 1.0)) * 2.0 - 1.0;

                self.vertices.push(
                    T::from_position(scaled_i * width, 0.0, scaled_j * depth)
                );
            }
        }
    }

    fn build_indices(&mut self, x_count: u16, z_count: u16) {
        let mut count = 0;
        for row in 0..z_count-1 {
            for col in 0..x_count-1 {
                let tl = row * x_count + col;
                let bl = tl + 1;
                let tr = tl + x_count;
                let br = tr + 1;

                if count % 2 == 0 {
                    self.indices.push(tl);
                    self.indices.push(bl);
                    self.indices.push(br);

                    self.indices.push(tl);
                    self.indices.push(br);
                    self.indices.push(tr);
                }
                else {
                    self.indices.push(tl);
                    self.indices.push(bl);
                    self.indices.push(tr);

                    self.indices.push(tr);
                    self.indices.push(bl);
                    self.indices.push(br);
                }
                count += 1;
            }
        }
    }
}















