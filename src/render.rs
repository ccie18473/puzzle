use crate::prelude::*;

pub type Point2 = cgmath::Point2<f32>;
pub type Vector2 = cgmath::Vector2<f32>;

pub const COLUMNS: usize = 4;
pub const ROWS: usize = 4;

pub struct TRender {
    pub table: TTable,
    pub resources: TResources,
    pub image: graphics::Image,
    pub original_puzzle: graphics::Rect,
    pub puzzle_width: f32,
    pub puzzle_height: f32,
    pub piece_width: f32,
    pub piece_height: f32,
}

impl TRender {
    pub fn new_render(ctx: &mut Context) -> TRender {
        let table = TTable::new_table();
        let mut resources = TResources::new_resources(ctx);
        let image = resources.get_image(table.game.random);
        let original_puzzle = image.dimensions();

        let s = Self {
            table,
            resources,
            image,
            original_puzzle,
            puzzle_width: 0.0,
            puzzle_height: 0.0,
            piece_width: 0.0,
            piece_height: 0.0,
        };
        s
    }
    pub fn game_render(&mut self, ctx: &mut Context) -> GameResult {
        let scale = Vector2::new(
            (self.puzzle_width / self.original_puzzle.w) as f32,
            (self.puzzle_height / self.original_puzzle.h) as f32,
        );

        for i in 0..COLUMNS * ROWS {
            let p = self.table.game.board.pieces[i];

            if p.number != COLUMNS * ROWS - 1 {
                let x1 = (p.number % COLUMNS) as f32 / COLUMNS as f32;
                let y1 = (p.number / ROWS) as f32 / ROWS as f32;

                let x2 = (i % COLUMNS) as f32 * self.piece_width;
                let y2 = (i / ROWS) as f32 * self.piece_height;

                let p = graphics::DrawParam::new()
                    .src(Rect::new(x1, y1, 1.0 / COLUMNS as f32, 1.0 / ROWS as f32))
                    .dest(Point2::new(x2, y2))
                    .scale(scale);

                let image = self.resources.get_image(self.table.game.random);
                self.original_puzzle = image.dimensions();
                graphics::draw(ctx, &image, p)?;
            }
        }

        Ok(())
    }
    pub fn solution_render(&mut self, ctx: &mut Context) -> GameResult {
        let scale = Vector2::new(
            (self.puzzle_width / self.original_puzzle.w) / 5.0 as f32,
            (self.puzzle_height / self.original_puzzle.h) / 5.0 as f32,
        );

        let p = graphics::DrawParam::new()
            .dest(Point2::new(self.puzzle_width + 50.0, 0.0))
            .scale(scale);

        let image = self.resources.get_image(self.table.game.random);
        self.original_puzzle = image.dimensions();
        graphics::draw(ctx, &image, p)?;

        Ok(())
    }
    pub fn fps_render(&mut self, ctx: &mut Context) -> GameResult {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(self.puzzle_width + 50.0, 400.0);
        graphics::draw(ctx, &fps_display, (p,))?;

        Ok(())
    }
    pub fn duration_render(&mut self, ctx: &mut Context) -> GameResult {
        let secs_display = graphics::Text::new(format!("Duration: {}", self.table.game.duration));
        let p = cgmath::Point2::new(self.puzzle_width + 50.0, 450.0);
        graphics::draw(ctx, &secs_display, (p,))?;

        Ok(())
    }
    pub fn moves_render(&mut self, ctx: &mut Context) -> GameResult {
        let moves_display = graphics::Text::new(format!("Moves: {}", self.table.game.moves));
        let p = cgmath::Point2::new(self.puzzle_width + 50.0, 500.0);
        graphics::draw(ctx, &moves_display, (p,))?;

        Ok(())
    }
    pub fn status_render(&mut self, ctx: &mut Context) -> GameResult {
        let status_display = graphics::Text::new(format!("{}", self.table.game.status));
        let p = cgmath::Point2::new(self.puzzle_width + 50.0, 550.0);
        graphics::draw(ctx, &status_display, (p,))?;

        Ok(())
    }
    pub fn display(&mut self, ctx: &mut Context) -> GameResult {
        self.game_render(ctx).unwrap();
        self.solution_render(ctx).unwrap();
        self.fps_render(ctx).unwrap();
        self.duration_render(ctx).unwrap();
        self.moves_render(ctx).unwrap();
        self.status_render(ctx).unwrap();

        Ok(())
    }
}
