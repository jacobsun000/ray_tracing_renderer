use crate::geometry::Hitable;
use crate::image::Image;
use crate::progress;
use crate::random;
use crate::Color;
use crate::Pixel;
use crate::Ray;
use crate::Scene;
use crate::Vector3d;

fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> Color {
    if depth == 0 {
        return Color::new([0.0, 0.0, 0.0]);
    }
    if let Some(rec) = scene.objects.hit(ray) {
        let target = rec.point + Vector3d::random_in_hemisphere(rec.normal);
        return (ray_color(
            &Ray {
                origin: rec.point,
                direction: target - rec.point,
            },
            scene, depth - 1
        )) * 0.5;
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t;
}

pub struct RenderOption {
    pub samples_per_pixel: i32,
    pub max_depth: usize,
}

pub struct Renderer<'a> {
    pub scene: &'a Scene,
    pub image: &'a mut Image,
    pub option: RenderOption,
}

impl<'a> Renderer<'a> {
    fn render_pixel(&self, x: usize, y: usize) -> Pixel {
        let width = self.image.width;
        let height = self.image.height;
        let samples = self.option.samples_per_pixel;
        let camera = &self.scene.camera;
        let r = || -> f64 { random() };
        let u = || (x as f64 + r()) / (width - 1) as f64;
        let v = || (y as f64 + r()) / (height - 1) as f64;
        let color: Color = (0..samples)
            .map(|_| camera.get_ray(u(), v()))
            .map(|r| ray_color(&r, &self.scene, self.option.max_depth))
            .sum();
        return color.to_pixel(samples);
    }

    pub fn render(&mut self) {
        let height = self.image.height;
        for y in 0..height {
            let percentage = (y as f32 / height as f32) * 100.0;
            progress::show(percentage);
            for x in 0..self.image.width {
                self.image.canvas[y][x] = self.render_pixel(x, height - 1 - y);
            }
        }
    }
}
